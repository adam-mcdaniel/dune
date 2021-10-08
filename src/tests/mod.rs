use crate::{parse_script, parse_tokens, SyntaxError};

fn tokenize_test(input: &str, expected: &str) -> Result<(), nom::Err<SyntaxError>> {
    let (rest, tokens) = parse_tokens(input)?;
    assert!(
        rest.is_empty(),
        "Not all input was tokenized. Rest: {:?}",
        rest
    );
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
fn tokenize1() -> Result<(), nom::Err<SyntaxError>> {
    tokenize_test(
        r#"let a = foo -> bar -> {
    foo == bar
}"#,
        r#"[
    Keyword(let),
    Whitespace( ),
    Symbol(a),
    Whitespace( ),
    Punctuation(=),
    Whitespace( ),
    Symbol(foo),
    Whitespace( ),
    Punctuation(->),
    Whitespace( ),
    Symbol(bar),
    Whitespace( ),
    Punctuation(->),
    Whitespace( ),
    Punctuation({),
    Whitespace(
        ),
    Symbol(foo),
    Whitespace( ),
    Operator(==),
    Whitespace( ),
    Symbol(bar),
    Whitespace(
    ),
    Punctuation(}),
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
