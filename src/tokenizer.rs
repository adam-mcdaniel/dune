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
        // dbg!("------>", input);

        Ok(alt((
            // 优先处理续航、换行符（新增）
            map_valid_token(line_continuation, TokenKind::Whitespace),
            // triple_quote_string,
            map_valid_token(linebreak, TokenKind::LineBreak),
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
// 新增函数：专门处理三重引号字符串
// fn triple_quote_string(input: Input<'_>) -> TokenizationResult<'_, (Token, Diagnostic)> {
//     // 1. 匹配起始 """
//     let (rest, _) = input.strip_prefix("\"\"\"").ok_or(NOT_FOUND)?;

//     let mut content = String::new();
//     let mut errors = Vec::new();
//     let mut current = rest;
//     let start_offset = input.get(offset);

//     // 2. 遍历直到找到结束 """ 或输入结束
//     loop {
//         // 检测结束标记 """
//         if let Some(new_rest) = current.strip_prefix("\"\"\"") {
//             current = new_rest;
//             break;
//         }

//         // 处理转义字符（可选，根据需求）
//         if let Some('\\') = current.chars().next() {
//             let (r, escaped_char) = parse_escape(current)?;
//             content.push(escaped_char);
//             current = r;
//             continue;
//         }

//         // 消费普通字符
//         let next_special = current.find(|c| c == '\\' || c == '"');
//         let (text_part, remaining) = match next_special {
//             Some(pos) => current.split_at(pos),
//             None => current.split_at(current.len()),
//         };

//         content.push_str(text_part.to_str(current.get(str)));
//         current = remaining;

//         // 输入耗尽但未找到结束符
//         if current.is_empty() {
//             errors.push(input.get(str).get(start_offset..input.len()));
//             break;
//         }
//     }

//     // 3. 生成Token和诊断信息
//     let (rest, range) = input.split_until(current);
//     let token = Token::new(TokenKind::StringLiteral, range);
//     let diag = if errors.is_empty() {
//         Diagnostic::Valid
//     } else {
//         Diagnostic::InvalidStringEscapes(errors.into_boxed_slice())
//     };
//     Ok((rest, (token, diag)))
// }

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

fn find_prev_char(original_str: &str, current_offset: usize) -> Option<char> {
    // let current_offset = original_str.len() - rest.len();
    let first_c = original_str.get(..current_offset);
    match first_c {
        Some(c) => {
            if !c.is_empty() {
                return c.chars().last();
            }
            // 2. 反向计算前导空白字节长度
            let ws_len = c
                .chars()
                .rev()
                .take_while(|c| c.is_whitespace() && *c != '\n')
                .map(|c| c.len_utf8())
                .sum();

            // 3. 安全切割空白部分
            let ws_start = current_offset.saturating_sub(ws_len);
            let before_nl = original_str.get(..ws_start).unwrap_or("");

            // 4. 获取最后一个非空白字符
            return before_nl.chars().last();
        }
        None => return None,
    }
}

fn linebreak(input: Input<'_>) -> TokenizationResult<'_> {
    // dbg!("--->", input.as_str_slice());

    if let Some((rest, nl_slice)) = input.strip_prefix("\n") {
        // dbg!(nl_slice);
        let original_str = input.as_original_str();

        // 1. 计算换行符的字节位置
        let current_offset = original_str.len().saturating_sub(rest.len() + 1);

        match find_prev_char(original_str, current_offset) {
            Some(c) => {
                // dbg!(c);
                if matches!(c, '{' | '(' | '[' | ',' | '>' | '=' | ';' | '\n' | '\\') {
                    // skip ; and \n because there's already a linebreak parsed.
                    // > is for ->
                    // dbg!("=== skip ");
                    return Err(NOT_FOUND);
                }
            }
            // 读取前面字符失败，跳过
            None => return Err(NOT_FOUND),
        }
        // dbg!("---> LineBreak ");

        Ok((rest, nl_slice))
    } else if let Some((rest, matched)) = input.strip_prefix(";") {
        Ok((rest, matched))
    } else {
        Err(NOT_FOUND)
    }
}
// 新增续行符解析函数
fn line_continuation(input: Input<'_>) -> TokenizationResult<'_> {
    if let Some((rest, matched)) = input.strip_prefix("\\\n") {
        // println!("rest={},matched=", rest, matched);
        // // dbg!(rest, matched);
        Ok((rest, matched))
    } else {
        Err(NOT_FOUND)
    }
}
// 新增行继续符识别逻辑
// fn line_continuation(input: Input<'_>) -> TokenizationResult<'_> {
//     if let Some((rest, _)) = input.strip_prefix("\\") {
//         // 消费后续所有空白（包括换行符）
//         let ws = rest.chars().take_while(char::is_ascii_digit).count();
//         let (rest, _) = rest.split_at(ws);
//         Ok((rest, input.split_at(1).1))
//     } else {
//         Err(NOT_FOUND)
//     }
// }
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
