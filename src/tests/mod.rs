use crate::{parse_script, tokenize, SyntaxError};

fn tokenize_test(input: &str, expected: &str) -> Result<(), SyntaxError> {
    let tokens = tokenize(input)?;
    let got = format!("{:#?}", tokens);
    assert_eq!(got.as_str(), expected);
    Ok(())
}

fn parse_test(input: &str, expected: &str) -> Result<(), nom::Err<SyntaxError>> {
    let expr = parse_script(input)?;
    let got = format!("{:#?}", expr);
    assert_eq!(got.as_str(), expected);
    Ok(())
}

#[test]
fn tokenize1() -> Result<(), SyntaxError> {
    tokenize_test(
        r#"let a = foo -> bar -> {
    foo == bar
}"#,
        r#"[
    Keyword(0..3),
    Whitespace(3..4),
    Symbol(4..5),
    Whitespace(5..6),
    Punctuation(6..7),
    Whitespace(7..8),
    Symbol(8..11),
    Whitespace(11..12),
    Punctuation(12..14),
    Whitespace(14..15),
    Symbol(15..18),
    Whitespace(18..19),
    Punctuation(19..21),
    Whitespace(21..22),
    Punctuation(22..23),
    Whitespace(23..28),
    Symbol(28..31),
    Whitespace(31..32),
    Operator(32..34),
    Whitespace(34..35),
    Symbol(35..38),
    Whitespace(38..39),
    Punctuation(39..40),
]"#,
    )
}

#[test]
fn parse1() -> Result<(), nom::Err<SyntaxError>> {
    parse_test(r#""String\t\r\n\"""#, r#"{ "String\t\r\n\"" }"#)
}

#[test]
fn parse2() -> Result<(), nom::Err<SyntaxError>> {
    parse_test(
        r#"let hello = "world\u{21}";"#,
        r#"{ let hello = "world!" }"#,
    )
}
