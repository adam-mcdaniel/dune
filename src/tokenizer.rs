use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, map},
    IResult,
};

use crate::SyntaxError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Punctuation(&'a str),
    Operator(&'a str),
    Keyword(&'a str),
    StringLiteral(&'a str),
    IntegerLiteral(&'a str),
    FloatLiteral(&'a str),
    BooleanLiteral(&'a str),
    Symbol(&'a str),
    Whitespace(&'a str),
    Eof,
}

fn parse_token(input: &str) -> IResult<&str, Token<'_>, SyntaxError> {
    if input.is_empty() {
        Ok((input, Token::Eof))
    } else if let Ok((input, op)) = any_punctuation(input) {
        Ok((input, Token::Punctuation(op)))
    } else if let Ok((input, kwd)) = any_keyword(input) {
        Ok((input, Token::Keyword(kwd)))
    } else if let Ok((input, kwd)) = any_operator(input) {
        Ok((input, Token::Operator(kwd)))
    } else if let Ok((input, lit)) = bool_literal(input) {
        Ok((input, Token::BooleanLiteral(lit)))
    } else {
        alt((
            map(string_literal, |s| Token::StringLiteral(s)),
            number_literal,
            map(symbol, |s| Token::Symbol(s)),
            map(whitespace, |s| Token::Whitespace(s)),
        ))(input)
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
        keyword_tag("="),  // must be surrounded by whitespace
        keyword_tag("|"),  // must be surrounded by whitespace
        keyword_tag(">>"), // `>>foo` is also a valid symbol
        keyword_tag("->"), // `->foo` is also a valid symbol
        keyword_tag("~>"), // `~>foo` is also a valid symbol
    ))(input)
}

fn any_operator(input: &str) -> IResult<&str, &'static str, ()> {
    alt((
        keyword_tag("to"),
        keyword_tag("=="),
        keyword_tag("!="),
        keyword_tag(">="),
        keyword_tag("<="),
        keyword_tag("&&"),
        keyword_tag("||"),
        keyword_tag("<"),
        keyword_tag(">"),
        keyword_tag("+"),
        keyword_tag("-"),
        keyword_tag("*"),
        keyword_tag("//"),
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
            return Ok((rest, Token::IntegerLiteral(number)));
        }
    };

    // skip places after the dot
    let places = rest.chars().take_while(char::is_ascii_digit).count();
    if places == 0 {
        return SyntaxError::expected(
            input,
            "float",
            None,
            Some("valid floats can be written like 1.0 or 5.23"),
        );
    }
    let rest = &rest[places..];

    let number_len = input.len() - rest.len();
    let (number, rest) = input.split_at(number_len);
    Ok((rest, Token::FloatLiteral(number)))
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
    // const ASCII_HEX_DIGIT: &str = "0123456789ABCDEFabcdef";

    let (input, _) = tag("\\")(input)?;
    let (input, _) = alt((
        tag("\""),
        tag("\\"),
        tag("/"),
        tag("b"),
        tag("f"),
        tag("n"),
        tag("r"),
        tag("t"),
        // This doesn't actually work!
        // map(count(one_of(ASCII_HEX_DIGIT), 4), |_| ""),
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
            '&' | '<' | '>' | '$' | '%' | '#' | '^' | ':' | '='
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
            Ok((new_input, token)) => {
                input = new_input;
                let is_eof = token == Token::Eof;
                result.push(token);
                if is_eof {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    if input.is_empty() {
        Ok((input, result))
    } else {
        Err(nom::Err::Failure(SyntaxError::InternalError))
    }
}
