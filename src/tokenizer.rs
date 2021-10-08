use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, map},
    multi::fold_many_m_n,
    sequence::tuple,
    IResult,
};

use crate::tokens::{Token, TokenKind};
use crate::SyntaxError;

fn parse_token(input: &str) -> IResult<&str, Option<Token<'_>>, SyntaxError> {
    if input.is_empty() {
        Ok((input, None))
    } else if let Ok((input, op)) = long_operator(input) {
        Ok((input, Some(Token::new(TokenKind::Operator, op))))
    } else if let Ok((input, op)) = any_punctuation(input) {
        Ok((input, Some(Token::new(TokenKind::Punctuation, op))))
    } else if let Ok((input, kwd)) = any_keyword(input) {
        Ok((input, Some(Token::new(TokenKind::Keyword, kwd))))
    } else if let Ok((input, op)) = short_operator(input) {
        Ok((input, Some(Token::new(TokenKind::Operator, op))))
    } else if let Ok((input, lit)) = bool_literal(input) {
        Ok((input, Some(Token::new(TokenKind::BooleanLiteral, lit))))
    } else {
        map(
            alt((
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

fn any_punctuation(input: &str) -> IResult<&str, &'static str, ()> {
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

fn long_operator(input: &str) -> IResult<&str, &'static str, ()> {
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

fn short_operator(input: &str) -> IResult<&str, &'static str, ()> {
    alt((
        keyword_tag("<"),
        keyword_tag(">"),
        keyword_tag("+"),
        keyword_tag("-"),
        keyword_tag("*"),
        keyword_tag("%"),
    ))(input)
}

fn any_keyword(input: &str) -> IResult<&str, &'static str, ()> {
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

fn string_literal(input: &str) -> IResult<&str, &str, SyntaxError> {
    let old_input = input;
    let (input, _) = tag("\"")(input)?;
    let (input, _) = ignore_string_inner(input)?;
    let (input, _) = alt((tag("\""), eof))(input)?;

    Ok((input, &old_input[0..old_input.len() - input.len()]))
}

fn number_literal(input: &str) -> IResult<&str, Token<'_>, SyntaxError> {
    // skip sign
    let rest = input.strip_prefix('-').unwrap_or(input);

    // skip places before the dot
    let rest = match rest.strip_prefix('0') {
        Some(s) => s,
        None => {
            let places = rest.chars().take_while(char::is_ascii_digit).count();
            if places == 0 {
                return Err(nom::Err::Error(SyntaxError::InternalError));
            }
            &rest[places..]
        }
    };

    // skip the dot, if present
    let rest = match rest.strip_prefix('.') {
        Some(s) => s,
        None => {
            let number_len = input.len() - rest.len();
            let (number, rest) = input.split_at(number_len);
            return Ok((rest, Token::new(TokenKind::IntegerLiteral, number)));
        }
    };

    // skip places after the dot
    let places = rest.chars().take_while(char::is_ascii_digit).count();
    if places == 0 {
        return SyntaxError::expected(
            input,
            "float",
            None::<&str>,
            Some("valid floats can be written like 1.0 or 5.23"),
        );
    }
    let rest = &rest[places..];

    let number_len = input.len() - rest.len();
    let (number, rest) = input.split_at(number_len);
    Ok((rest, Token::new(TokenKind::FloatLiteral, number)))
}

fn bool_literal(input: &str) -> IResult<&str, &'static str, ()> {
    alt((keyword_tag("True"), keyword_tag("False")))(input)
}

fn symbol(input: &str) -> IResult<&str, &str, SyntaxError> {
    let symbol_chars = input
        .chars()
        .take_while(|&c| is_symbol_char(c))
        .map(|c| c.len_utf8())
        .sum();
    if symbol_chars == 0 {
        return Err(nom::Err::Error(SyntaxError::InternalError));
    }

    let (symbol, rest) = input.split_at(symbol_chars);
    Ok((rest, symbol))
}

fn whitespace(input: &str) -> IResult<&str, &str, SyntaxError> {
    let ws_chars = input.chars().take_while(char::is_ascii_whitespace).count();

    if ws_chars == 0 {
        return Err(nom::Err::Error(SyntaxError::InternalError));
    }

    let (ws, rest) = input.split_at(ws_chars);
    Ok((rest, ws))
}

fn other(input: &str) -> IResult<&str, &str, SyntaxError> {
    let mut chars = input.chars();
    match chars.next() {
        Some(_) => {
            let rest = chars.as_str();
            Ok((rest, &input[..input.len() - rest.len()]))
        }
        None => Err(nom::Err::Error(SyntaxError::InternalError)),
    }
}

fn comment(input: &str) -> IResult<&str, &str, SyntaxError> {
    if input.starts_with('#') {
        let index = input
            .chars()
            .take_while(|&c| !matches!(c, '\r' | '\n'))
            .map(|c| c.len_utf8())
            .sum::<usize>();

        let (comment, rest) = input.split_at(index);
        Ok((rest, comment))
    } else {
        Err(nom::Err::Error(SyntaxError::InternalError))
    }
}

fn ignore_string_inner(mut input: &str) -> IResult<&str, (), SyntaxError> {
    loop {
        match input.chars().next() {
            Some('\"') | None => break,
            Some('\\') => {
                match parse_escape(input) {
                    Ok((new_input, _)) => input = new_input,
                    Err(_) => return SyntaxError::unrecoverable(
                        input,
                        "string",
                        Some("invalid escape"),
                        Some(
                            "escape codes can be one of: \\\", \\\\, \\/, \\b, \\f, \\n, \\r, \\t ",
                        ),
                    ),
                }
            }
            Some(ch) => {
                input = &input[ch.len_utf8()..];
            }
        }
    }

    Ok((input, ()))
}

fn parse_escape(input: &str) -> IResult<&str, (), SyntaxError> {
    fn parse_hex_digit(input: &str) -> IResult<&str, &str, SyntaxError> {
        let mut chars = input.chars();
        chars
            .next()
            .filter(char::is_ascii_hexdigit)
            .ok_or(nom::Err::Error(SyntaxError::InternalError))?;
        Ok((chars.as_str(), ""))
    }

    let (input, _) = tag("\\")(input)?;
    let (input, _) = alt((
        tag("\""),
        tag("\\"),
        tag("b"),
        tag("f"),
        tag("n"),
        tag("r"),
        tag("t"),
        map(
            tuple((
                tag("u{"),
                fold_many_m_n(1, 5, parse_hex_digit, || "", |_, _| ""),
                tag("}"),
            )),
            |_| "",
        ),
    ))(input)?;
    Ok((input, ()))
}

/// Parses a word that contains characters which can also appear in a symbol.
///
/// This parser ensures that the word is *not* immediately followed by symbol characters.
fn keyword_tag<'a>(keyword: &'a str) -> impl Fn(&str) -> IResult<&str, &'a str, ()> {
    move |input: &str| match input.strip_prefix(keyword) {
        Some(rest) if !rest.starts_with(is_symbol_char) => Ok((rest, keyword)),
        _ => Err(nom::Err::Error(())),
    }
}

/// Parses a word that is allowed to be immediately followed by symbol characters.
///
/// This is essentially the same as `nom::bytes::complete::tag`, but with different lifetimes:
/// If the provided string has a 'static lifetime, so does the returned string.
fn punctuation_tag<'a>(punct: &'a str) -> impl Fn(&str) -> IResult<&str, &'a str, ()> {
    move |input: &str| match input.strip_prefix(punct) {
        Some(rest) => Ok((rest, punct)),
        _ => Err(nom::Err::Error(())),
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

pub fn parse_tokens(mut input: &str) -> IResult<&str, Vec<Token>, SyntaxError> {
    let mut result = Vec::new();
    loop {
        match parse_token(input) {
            Ok((new_input, Some(token))) => {
                input = new_input;
                result.push(token);
            }
            Ok((_, None)) => break,
            Err(e) => return Err(e),
        }
    }
    if input.is_empty() {
        Ok((input, result))
    } else {
        Err(nom::Err::Failure(SyntaxError::InternalError))
    }
}
