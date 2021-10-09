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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizationError {
    NotFound,
    InvalidString(StrSlice),
    InvalidNumber(StrSlice),
    Internal {
        kind: nom::error::ErrorKind,
        at: usize,
    },
    LeftoverTokens(StrSlice),
}

impl TokenizationError {
    fn not_found() -> nom::Err<TokenizationError> {
        nom::Err::Error(TokenizationError::NotFound)
    }
}

impl ParseError<Input<'_>> for TokenizationError {
    fn from_error_kind(input: Input<'_>, kind: nom::error::ErrorKind) -> Self {
        TokenizationError::Internal {
            kind,
            at: input.offset(),
        }
    }

    fn append(input: Input<'_>, kind: nom::error::ErrorKind, _: Self) -> Self {
        TokenizationError::Internal {
            kind,
            at: input.offset(),
        }
    }
}

fn parse_token(input: Input) -> IResult<Input, Option<Token>, TokenizationError> {
    if input.is_empty() {
        Ok((input, None))
    } else {
        map(
            alt((
                map(long_operator, |s| Token::new(TokenKind::Operator, s)),
                map(any_punctuation, |s| Token::new(TokenKind::Punctuation, s)),
                map(any_keyword, |s| Token::new(TokenKind::Keyword, s)),
                map(short_operator, |s| Token::new(TokenKind::Operator, s)),
                map(bool_literal, |s| Token::new(TokenKind::BooleanLiteral, s)),
                map(comment, |s| Token::new(TokenKind::Comment, s)),
                map(string_literal, |s| Token::new(TokenKind::StringLiteral, s)),
                number_literal,
                map(symbol, |s| Token::new(TokenKind::Symbol, s)),
                map(whitespace, |s| Token::new(TokenKind::Whitespace, s)),
                map(other, |s| Token::new(TokenKind::Other, s)),
            )),
            Some,
        )(input)
    }
}

fn any_punctuation(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    alt((
        punctuation_tag("("),
        punctuation_tag(")"),
        punctuation_tag("["),
        punctuation_tag("]"),
        punctuation_tag("{"),
        punctuation_tag("}"),
        punctuation_tag("@"),
        punctuation_tag("\'"),
        punctuation_tag(","),
        punctuation_tag(";"),
        punctuation_tag("="),
        keyword_tag("|"),  // must be surrounded by whitespace
        keyword_tag(">>"), // `>>foo` is also a valid symbol
        keyword_tag("->"), // `->foo` is also a valid symbol
        keyword_tag("~>"), // `~>foo` is also a valid symbol
    ))(input)
}

fn long_operator(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    alt((
        keyword_tag("to"),
        keyword_tag("=="),
        keyword_tag("!="),
        keyword_tag(">="),
        keyword_tag("<="),
        keyword_tag("&&"),
        keyword_tag("||"),
        keyword_tag("//"),
    ))(input)
}

fn short_operator(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    alt((
        keyword_tag("<"),
        keyword_tag(">"),
        keyword_tag("+"),
        keyword_tag("-"),
        keyword_tag("*"),
        keyword_tag("%"),
    ))(input)
}

fn any_keyword(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
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

fn string_literal(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    let (rest, _) = punctuation_tag("\"")(input)?;
    let (rest, _) = parse_string_inner(rest)?;
    let (rest, _) = alt((punctuation_tag("\""), map(eof, |_| input.split_empty())))(rest)?;

    Ok(input.split_until(rest))
}

fn number_literal(input: Input<'_>) -> IResult<Input<'_>, Token, TokenizationError> {
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
        .ok_or_else(TokenizationError::not_found)?;

    // skip the dot, if present
    let (rest, _) = match rest.strip_prefix(".") {
        Some(s) => s,
        None => {
            let (rest, number) = input.split_until(rest);
            return Ok((rest, Token::new(TokenKind::IntegerLiteral, number)));
        }
    };

    // skip places after the dot
    let places = rest.chars().take_while(char::is_ascii_digit).count();
    if places == 0 {
        return Err(nom::Err::Failure(TokenizationError::InvalidNumber(
            input.split_until(rest).1,
        )));
    }
    let (rest, _) = rest.split_at(places);

    let (rest, number) = input.split_until(rest);
    Ok((rest, Token::new(TokenKind::FloatLiteral, number)))
}

fn bool_literal(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    alt((keyword_tag("True"), keyword_tag("False")))(input)
}

fn symbol(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    let len = input
        .chars()
        .take_while(|&c| is_symbol_char(c))
        .map(char::len_utf8)
        .sum();

    if len == 0 {
        return Err(TokenizationError::not_found());
    }

    Ok(input.split_at(len))
}

fn whitespace(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    let ws_chars = input.chars().take_while(char::is_ascii_whitespace).count();

    if ws_chars == 0 {
        return Err(TokenizationError::not_found());
    }

    Ok(input.split_at(ws_chars))
}

fn other(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    match input.chars().next() {
        Some(c) => Ok(input.split_at(c.len_utf8())),
        None => Err(TokenizationError::not_found()),
    }
}

fn comment(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    if input.starts_with('#') {
        let len = input
            .chars()
            .take_while(|&c| !matches!(c, '\r' | '\n'))
            .map(|c| c.len_utf8())
            .sum();

        Ok(input.split_at(len))
    } else {
        Err(TokenizationError::not_found())
    }
}

fn parse_string_inner(input: Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    let mut rest = input;
    loop {
        match rest.chars().next() {
            Some('"') | None => break,
            Some('\\') => rest = parse_escape(rest)?.0,
            Some(ch) => rest = rest.split_at(ch.len_utf8()).0,
        }
    }

    Ok(input.split_until(rest))
}

fn parse_escape(input: Input) -> IResult<Input, (), TokenizationError> {
    fn parse_hex_digit(input: Input) -> IResult<Input, StrSlice, TokenizationError> {
        input
            .chars()
            .next()
            .filter(char::is_ascii_hexdigit)
            .ok_or_else(TokenizationError::not_found)?;
        Ok(input.split_at(1))
    }

    let (rest, _) = punctuation_tag("\\")(input)?;
    let (rest, _) = alt((
        punctuation_tag("\""),
        punctuation_tag("\\"),
        punctuation_tag("b"),
        punctuation_tag("f"),
        punctuation_tag("n"),
        punctuation_tag("r"),
        punctuation_tag("t"),
        map(
            tuple((
                punctuation_tag("u{"),
                fold_many_m_n(
                    1,
                    5,
                    parse_hex_digit,
                    || rest.split_empty(),
                    |_, _| rest.split_empty(),
                ),
                punctuation_tag("}"),
            )),
            |_| rest.split_empty(),
        ),
    ))(rest)
    .map_err(|_| {
        nom::Err::Failure(TokenizationError::InvalidString(
            input.split_saturating(2).1,
        ))
    })?;
    Ok((rest, ()))
}

/// Parses a word that contains characters which can also appear in a symbol.
///
/// This parser ensures that the word is *not* immediately followed by symbol characters.
fn keyword_tag(
    keyword: &str,
) -> impl '_ + Fn(Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    move |input: Input<'_>| match input.strip_prefix(keyword) {
        Some((rest, keyword)) if !rest.starts_with(is_symbol_char) => Ok((rest, keyword)),
        _ => Err(TokenizationError::not_found()),
    }
}

/// Parses a word that is allowed to be immediately followed by symbol characters.
///
/// This is essentially the same as `nom::bytes::complete::tag`, but with different lifetimes:
/// If the provided string has a 'static lifetime, so does the returned string.
fn punctuation_tag(
    punct: &str,
) -> impl '_ + Fn(Input<'_>) -> IResult<Input<'_>, StrSlice, TokenizationError> {
    move |input: Input<'_>| match input.strip_prefix(punct) {
        Some((rest, punct)) => Ok((rest, punct)),
        None => Err(TokenizationError::not_found()),
    }
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

pub(crate) fn parse_tokens(mut input: Input<'_>) -> Result<Vec<Token>, TokenizationError> {
    let mut result = Vec::new();
    loop {
        match parse_token(input) {
            Ok((new_input, Some(token))) => {
                input = new_input;
                result.push(token);
            }
            Ok((_, None)) => break,
            Err(e) => match e {
                nom::Err::Incomplete(_) => unreachable!(),
                nom::Err::Error(e) | nom::Err::Failure(e) => return Err(e),
            },
        }
    }
    if input.is_empty() {
        Ok(result)
    } else {
        Err(TokenizationError::LeftoverTokens(input.as_str_slice()))
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizationError> {
    let str = input.into();
    let mut input = Input::new(&str);

    let mut result = Vec::new();
    loop {
        match parse_token(input) {
            Ok((new_input, Some(token))) => {
                input = new_input;
                result.push(token);
            }
            Ok((_, None)) => break,
            Err(e) => match e {
                nom::Err::Incomplete(_) => unreachable!(),
                nom::Err::Error(e) | nom::Err::Failure(e) => return Err(e),
            },
        }
    }
    if input.is_empty() {
        Ok(result)
    } else {
        Err(TokenizationError::LeftoverTokens(input.as_str_slice()))
    }
}
