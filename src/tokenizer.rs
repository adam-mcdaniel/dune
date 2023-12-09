use std::convert::TryFrom;

use detached_str::StrSlice;
use nom::{
    branch::alt,
    combinator::{eof, map},
    error::ParseError,
    multi::fold_many_m_n,
    sequence::tuple,
    IResult,
};

use crate::tokens::{Input, Token, TokenKind};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct NotFoundError;

const NOT_FOUND: nom::Err<NotFoundError> = nom::Err::Error(NotFoundError);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diagnostic {
    Valid,
    InvalidStringEscapes(Box<[StrSlice]>),
    InvalidNumber(StrSlice),
    IllegalChar(StrSlice),
    NotTokenized(StrSlice),
}

impl<I> ParseError<I> for NotFoundError {
    fn from_error_kind(_: I, _: nom::error::ErrorKind) -> Self {
        NotFoundError
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

type TokenizationResult<'a, T = StrSlice> = IResult<Input<'a>, T, NotFoundError>;

fn parse_token(input: Input) -> TokenizationResult<'_, (Token, Diagnostic)> {
    if input.is_empty() {
        Err(NOT_FOUND)
    } else {
        Ok(alt((
            map_valid_token(long_operator, TokenKind::Operator),
            map_valid_token(any_punctuation, TokenKind::Punctuation),
            map_valid_token(any_keyword, TokenKind::Keyword),
            map_valid_token(short_operator, TokenKind::Operator),
            map_valid_token(bool_literal, TokenKind::BooleanLiteral),
            map_valid_token(comment, TokenKind::Comment),
            string_literal,
            number_literal,
            map_valid_token(symbol, TokenKind::Symbol),
            map_valid_token(whitespace, TokenKind::Whitespace),
        ))(input)
        .unwrap_or_else(|_| {
            let next = input.chars().next().unwrap();
            let (rest, range) = input.split_at(next.len_utf8());
            let token = Token::new(TokenKind::Symbol, range);
            (rest, (token, Diagnostic::IllegalChar(range)))
        }))
    }
}

fn map_valid_token(
    mut parser: impl FnMut(Input<'_>) -> TokenizationResult<'_>,
    kind: TokenKind,
) -> impl FnMut(Input<'_>) -> TokenizationResult<'_, (Token, Diagnostic)> {
    move |input| {
        let (input, s) = parser(input)?;
        Ok((input, (Token::new(kind, s), Diagnostic::Valid)))
    }
}

fn any_punctuation(input: Input<'_>) -> TokenizationResult<'_> {
    alt((
        punctuation_tag("("),
        punctuation_tag(")"),
        punctuation_tag("["),
        punctuation_tag("]"),
        punctuation_tag("{"),
        punctuation_tag("}"),
        punctuation_tag("\'"),
        punctuation_tag(","),
        punctuation_tag(";"),
        punctuation_tag("="),
        keyword_tag("->"), // `->foo` is also a valid symbol
        keyword_tag("~>"), // `~>foo` is also a valid symbol
    ))(input)
}

fn long_operator(input: Input<'_>) -> TokenizationResult<'_> {
    alt((
        keyword_tag("to"),
        keyword_tag("=="),
        keyword_tag("!="),
        keyword_tag(">="),
        keyword_tag("<="),
        keyword_tag("&&"),
        keyword_tag("||"),
        keyword_tag("//"),
        keyword_tag("<<"),
        keyword_tag(">>"),
        keyword_tag(">>>"),
    ))(input)
}

fn short_operator(input: Input<'_>) -> TokenizationResult<'_> {
    alt((
        keyword_tag("<"),
        keyword_tag(">"),
        keyword_tag("+"),
        keyword_tag("-"),
        keyword_tag("*"),
        keyword_tag("%"),
        keyword_tag("|"),
        punctuation_tag("@"),
        punctuation_tag("!"),
    ))(input)
}

fn any_keyword(input: Input<'_>) -> TokenizationResult<'_> {
    alt((
        keyword_tag("None"),
        keyword_tag("then"),
        keyword_tag("else"),
        keyword_tag("let"),
        keyword_tag("for"),
        keyword_tag("if"),
        keyword_tag("in"),
    ))(input)
}

fn string_literal(input: Input<'_>) -> TokenizationResult<'_, (Token, Diagnostic)> {
    let (rest, _) = punctuation_tag("\"")(input)?;
    let (rest, diagnostics) = parse_string_inner(rest)?;
    let (rest, _) = alt((punctuation_tag("\""), map(eof, |_| input.split_empty())))(rest)?;

    let (rest, range) = input.split_until(rest);
    let token = Token::new(TokenKind::StringLiteral, range);
    Ok((rest, (token, diagnostics)))
}

fn number_literal(input: Input<'_>) -> TokenizationResult<'_, (Token, Diagnostic)> {
    // skip sign
    let (rest, _) = input.strip_prefix("-").unwrap_or_else(|| input.split_at(0));

    // skip places before the dot
    let (rest, _) = rest
        .strip_prefix("0")
        .or_else(|| {
            let places = rest.chars().take_while(char::is_ascii_digit).count();
            if places > 0 {
                Some(rest.split_at(places))
            } else {
                None
            }
        })
        .ok_or(NOT_FOUND)?;

    // skip the dot, if present
    let (rest, _) = match rest.strip_prefix(".") {
        Some(s) => s,
        None => {
            let (rest, number) = input.split_until(rest);
            let token = Token::new(TokenKind::IntegerLiteral, number);
            return Ok((rest, (token, Diagnostic::Valid)));
        }
    };

    // skip places after the dot
    let places = rest.chars().take_while(char::is_ascii_digit).count();
    if places == 0 {
        let (rest, range) = input.split_until(rest);
        let token = Token::new(TokenKind::FloatLiteral, range);
        return Ok((rest, (token, Diagnostic::InvalidNumber(range))));
    }
    let (rest, _) = rest.split_at(places);

    let (rest, range) = input.split_until(rest);
    let token = Token::new(TokenKind::FloatLiteral, range);
    Ok((rest, (token, Diagnostic::Valid)))
}

fn bool_literal(input: Input<'_>) -> TokenizationResult<'_> {
    alt((keyword_tag("True"), keyword_tag("False")))(input)
}

fn symbol(input: Input<'_>) -> TokenizationResult<'_> {
    let len = input
        .chars()
        .take_while(|&c| is_symbol_char(c))
        .map(char::len_utf8)
        .sum();

    if len == 0 {
        return Err(NOT_FOUND);
    }

    Ok(input.split_at(len))
}

fn whitespace(input: Input<'_>) -> TokenizationResult<'_> {
    let ws_chars = input.chars().take_while(char::is_ascii_whitespace).count();

    if ws_chars == 0 {
        return Err(NOT_FOUND);
    }

    Ok(input.split_at(ws_chars))
}

fn comment(input: Input<'_>) -> TokenizationResult<'_> {
    if input.starts_with('#') {
        let len = input
            .chars()
            .take_while(|&c| !matches!(c, '\r' | '\n'))
            .map(|c| c.len_utf8())
            .sum();

        Ok(input.split_at(len))
    } else {
        Err(NOT_FOUND)
    }
}

fn parse_string_inner(input: Input<'_>) -> TokenizationResult<'_, Diagnostic> {
    let mut rest = input;
    let mut errors = Vec::new();

    loop {
        match rest.chars().next() {
            Some('"') | None => break,
            Some('\\') => {
                let (r, diagnostic) = parse_escape(rest)?;
                rest = r;
                if let Diagnostic::InvalidStringEscapes(ranges) = diagnostic {
                    errors.push(ranges[0]);
                }
            }
            Some(ch) => rest = rest.split_at(ch.len_utf8()).0,
        }
    }

    let diagnostic = match errors.is_empty() {
        true => Diagnostic::Valid,
        false => Diagnostic::InvalidStringEscapes(errors.into_boxed_slice()),
    };

    Ok((rest, diagnostic))
}

fn parse_escape(input: Input<'_>) -> TokenizationResult<'_, Diagnostic> {
    fn parse_hex_digit(input: Input<'_>) -> TokenizationResult<'_> {
        input
            .chars()
            .next()
            .filter(char::is_ascii_hexdigit)
            .ok_or(NOT_FOUND)?;

        Ok(input.split_at(1))
    }

    let (rest, _) = punctuation_tag("\\")(input)?;

    let mut parser1 = alt((
        punctuation_tag("\""),
        punctuation_tag("\\"),
        punctuation_tag("b"),
        punctuation_tag("f"),
        punctuation_tag("n"),
        punctuation_tag("r"),
        punctuation_tag("t"),
    ));
    if let Ok((rest, _)) = parser1(rest) {
        return Ok((rest, Diagnostic::Valid));
    }

    let mut parser2 = tuple((
        punctuation_tag("u{"),
        fold_many_m_n(
            1,
            5,
            parse_hex_digit,
            || None::<StrSlice>,
            |a, b| match a {
                Some(a) => Some(a.join(b)),
                None => Some(b),
            },
        ),
    ));

    let rest = match parser2(rest) {
        Ok((rest, (_, range))) => {
            let range = range.unwrap();
            let hex = range.to_str(input.as_original_str());
            let code_point = u32::from_str_radix(hex, 16).unwrap();
            if char::try_from(code_point).is_ok() {
                rest
            } else {
                let ranges = vec![range].into_boxed_slice();
                return Ok((rest, Diagnostic::InvalidStringEscapes(ranges)));
            }
        }
        Err(_) => {
            let (rest, range) = input.split_saturating(2);
            let ranges = vec![range].into_boxed_slice();
            return Ok((rest, Diagnostic::InvalidStringEscapes(ranges)));
        }
    };

    match punctuation_tag("}")(rest) {
        Ok((rest, _)) => Ok((rest, Diagnostic::Valid)),
        Err(_) => {
            let (rest, range) = input.split_until(rest);
            let ranges = vec![range].into_boxed_slice();
            Ok((rest, Diagnostic::InvalidStringEscapes(ranges)))
        }
    }
}

/// Parses a word that contains characters which can also appear in a symbol.
///
/// This parser ensures that the word is *not* immediately followed by symbol characters.
fn keyword_tag(keyword: &str) -> impl '_ + Fn(Input<'_>) -> TokenizationResult<'_> {
    move |input: Input<'_>| {
        input
            .strip_prefix(keyword)
            .filter(|(rest, _)| !rest.starts_with(is_symbol_char))
            .ok_or(NOT_FOUND)
    }
}

/// Parses a word that is allowed to be immediately followed by symbol characters.
///
/// This is essentially the same as `nom::bytes::complete::tag`, but with different lifetimes:
/// If the provided string has a 'static lifetime, so does the returned string.
fn punctuation_tag(punct: &str) -> impl '_ + Fn(Input<'_>) -> TokenizationResult<'_> {
    move |input: Input<'_>| input.strip_prefix(punct).ok_or(NOT_FOUND)
}

/// Checks whether the character is allowed in a symbol.
fn is_symbol_char(c: char) -> bool {
    macro_rules! special_char_pattern {
        () => {
            '_' | '+' | '-' | '.' | '~' | '\\' | '/' | '?' |
            '&' | '<' | '>' | '$' | '%' | '#' | '^' | ':'
        };
    }

    static ASCII_SYMBOL_CHARS: [bool; 128] = {
        let mut array = [false; 128];
        let mut i = 0u8;

        while i < 128 {
            array[i as usize] = matches!(
                i as char,
                'a'..='z' | 'A'..='Z' | '0'..='9' | special_char_pattern!()
            );
            i += 1;
        }

        array
    };

    if c.is_ascii() {
        ASCII_SYMBOL_CHARS[c as usize]
    } else {
        false
        // currently only ASCII identifiers are supported :/
    }
}

pub(crate) fn parse_tokens(mut input: Input<'_>) -> (Vec<Token>, Vec<Diagnostic>) {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    loop {
        match parse_token(input) {
            Err(_) => break,
            Ok((new_input, (token, diagnostic))) => {
                input = new_input;
                tokens.push(token);
                diagnostics.push(diagnostic);
            }
        }
    }
    if !input.is_empty() {
        diagnostics.push(Diagnostic::NotTokenized(input.as_str_slice()))
    }
    (tokens, diagnostics)
}

pub fn tokenize(input: &str) -> (Vec<Token>, Vec<Diagnostic>) {
    let str = input.into();
    let input = Input::new(&str);
    parse_tokens(input)
}
