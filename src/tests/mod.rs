use crate::{parse_script, tokenize, Diagnostic, SyntaxError};

#[track_caller]
fn tokenize_test(input: &str, expected: &str) {
    let (tokens, mut diagnostics) = tokenize(input);
    diagnostics.retain(|d| d != &Diagnostic::Valid);
    assert_eq!(diagnostics.as_slice(), []);

    let got = format!("{:#?}", tokens);
    assert_eq!(got.as_str(), expected);
}

#[track_caller]
fn tokenize_test_err(input: &str) {
    let (_, mut diagnostics) = tokenize(input);
    diagnostics.retain(|d| d != &Diagnostic::Valid);
    assert_ne!(diagnostics.as_slice(), []);
}

#[track_caller]
fn parse_test(input: &str, expected: &str) -> Result<(), nom::Err<SyntaxError>> {
    let expr = parse_script(input)?;
    let got = format!("{:#?}", expr);
    assert_eq!(got.as_str(), expected);
    Ok(())
}

#[test]
fn tokenize_function() {
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
    );
}

#[test]
fn tokenize_string() {
    tokenize_test(
        r#"let a = "Hello \t world \u{254B} \"\\";"#,
        r#"[
    Keyword(0..3),
    Whitespace(3..4),
    Symbol(4..5),
    Whitespace(5..6),
    Punctuation(6..7),
    Whitespace(7..8),
    StringLiteral(8..38),
    Punctuation(38..39),
]"#,
    );
}

#[test]
fn tokenize_object() {
    tokenize_test(
        r#"{a=5, b = "hello"}"#,
        r#"[
    Punctuation(0..1),
    Symbol(1..2),
    Punctuation(2..3),
    IntegerLiteral(3..4),
    Punctuation(4..5),
    Whitespace(5..6),
    Symbol(6..7),
    Whitespace(7..8),
    Punctuation(8..9),
    Whitespace(9..10),
    StringLiteral(10..17),
    Punctuation(17..18),
]"#,
    );
}

#[test]
fn tokenize_unclosed_string() {
    tokenize_test(
        r#""Hello"#,
        r#"[
    StringLiteral(0..6),
]"#,
    );
}

#[test]
fn tokenize_numbers() {
    tokenize_test(
        r#"3 -4 1.1 4646345653 -3.14159"#,
        r#"[
    IntegerLiteral(0..1),
    Whitespace(1..2),
    IntegerLiteral(2..4),
    Whitespace(4..5),
    FloatLiteral(5..8),
    Whitespace(8..9),
    IntegerLiteral(9..19),
    Whitespace(19..20),
    FloatLiteral(20..28),
]"#,
    );
}

#[test]
fn tokenize_comments() {
    tokenize_test(
        r#"# test
let x # test
= 3;"#,
        r#"[
    Comment(0..6),
    Whitespace(6..7),
    Keyword(7..10),
    Whitespace(10..11),
    Symbol(11..12),
    Whitespace(12..13),
    Comment(13..19),
    Whitespace(19..20),
    Punctuation(20..21),
    Whitespace(21..22),
    IntegerLiteral(22..23),
    Punctuation(23..24),
]"#,
    );
}

#[test]
fn tokenize_symbols_and_operators() {
    tokenize_test(
        r#"to == != >= <= && || // < > + - * % _+-.~\/?&<>$%^: abcXYZ"#,
        r#"[
    Operator(0..2),
    Whitespace(2..3),
    Operator(3..5),
    Whitespace(5..6),
    Operator(6..8),
    Whitespace(8..9),
    Operator(9..11),
    Whitespace(11..12),
    Operator(12..14),
    Whitespace(14..15),
    Operator(15..17),
    Whitespace(17..18),
    Operator(18..20),
    Whitespace(20..21),
    Operator(21..23),
    Whitespace(23..24),
    Operator(24..25),
    Whitespace(25..26),
    Operator(26..27),
    Whitespace(27..28),
    Operator(28..29),
    Whitespace(29..30),
    Operator(30..31),
    Whitespace(31..32),
    Operator(32..33),
    Whitespace(33..34),
    Operator(34..35),
    Whitespace(35..36),
    Symbol(36..51),
    Whitespace(51..52),
    Symbol(52..58),
]"#,
    );
}

#[test]
fn tokenize_invalid_numbers() {
    tokenize_test_err(r#"3."#);
    tokenize_test_err(r#"-15."#);
}

#[test]
fn tokenize_invalid_strings() {
    tokenize_test_err(r#""\"#);
    tokenize_test_err(r#""\x""#);
    tokenize_test_err(r#""\u""#);
    tokenize_test_err(r#""\u{}""#);
    tokenize_test_err(r#""\u{FFFFFF}""#); // at most 5 hex digits allowed
    tokenize_test_err(r#""\u{D800}""#); // lower surrogate
    tokenize_test_err(r#""\u{g}""#); // not a hex digit
}

#[test]
fn tokenize_invalid_symbols() {
    tokenize_test_err(r#"`"#);
    tokenize_test_err(r#"§"#);
    tokenize_test_err(r#"°"#);
    tokenize_test_err(r#"–"#); // em dash
    tokenize_test_err(r#"ä"#); // German umlaut
    tokenize_test_err(r#"€"#); // Euro sign
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
