#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate unicode_categories;

use std::borrow::Cow;
use std::num::ParseIntError;
use std::{char, str};
use thiserror::Error;
use unicode_categories::UnicodeCategories;

/// Escape the provided string with shell-like quoting and escapes.
/// Strings which do not need to be escaped will be returned unchanged.
///
/// # Details
///
/// Escape will prefer to avoid quoting when possible. When quotes are required, it will prefer
/// single quotes (which have simpler semantics, namely no escaping). In all other cases it will
/// use double quotes and escape whatever characters it needs to.
///
/// For the full list of escapes which will be used, see the table in
/// [unescape](unescape).
///
/// # Examples
/// ```
/// use snailquote::escape;
/// # // The println/assert duplication is because I want to show the output you'd get without
/// # // rust's string quoting/escaping getting in the way
/// # // Ideally we could just assert on stdout, not duplicate, see
/// # // https://github.com/rust-lang/rfcs/issues/2270
/// println!("{}", escape("foo")); // no escapes needed
/// // foo
/// # assert_eq!(escape("foo"), "foo");
/// println!("{}", escape("String with spaces")); // single-quoteable
/// // 'String with spaces'
/// # assert_eq!(escape("String with spaces"), "'String with spaces'");
/// println!("{}", escape("東方")); // no escapes needed
/// // 東方
/// # assert_eq!(escape("東方"), "東方");
/// println!("{}", escape("\"new\nline\"")); // escape needed
/// // "\"new\nline\""
/// # assert_eq!(escape("\"new\nline\""), "\"\\\"new\\nline\\\"\"");
/// ```
// escape performs some minimal 'shell-like' escaping on a given string
pub fn escape(s: &str) -> Cow<str> {
    let mut needs_quoting = false;
    let mut single_quotable = true;

    for c in s.chars() {
        if c == '\'' || c == '\\' {
            single_quotable = false;
            needs_quoting = true;
        } else if c == '"' {
            needs_quoting = true;
        } else if c == ' ' {
            // special case; whitespace that can be single quoted.
            // Other whitespace (e.g. '\t') needs double-quoting escaping, but literal spaces only
            // need quoting, not escaping.
            needs_quoting = true;
        } else if c.is_whitespace() || c.is_separator() || c.is_other() {
            single_quotable = false;
            needs_quoting = true;
        }
        if needs_quoting && !single_quotable {
            // We know we'll need double quotes, no need to check further
            break;
        }
    }

    if !needs_quoting {
        return Cow::from(s);
    }
    if single_quotable {
        return format!("'{}'", s).into();
    }
    // otherwise we need to double quote it

    let mut output = String::with_capacity(s.len());
    output.push('"');

    for c in s.chars() {
        if c == '"' {
            output += "\\\"";
        } else if c == '\\' {
            output += "\\\\";
        } else if c == ' ' {
            // avoid 'escape_unicode' for ' ' even though it's a separator
            output.push(c);
        } else if c == '$' {
            output += "\\$";
        } else if c == '`' {
            output += "\\`";
        } else if c.is_other() || c.is_separator() {
            output += &escape_character(c);
        } else {
            output.push(c);
        }
    }

    output.push('"');
    output.into()
}

// escape_character is an internal helper method which converts the given unicode character into an
// escape sequence. It is assumed the character passed in *must* be escaped (e.g. it is some non-printable
// or unusual character).
// escape_character will prefer more human readable escapes (e.g. '\n' over '\u{0a}'), but will
// fall back on dumb unicode escaping.
// It is similar to rust's "char::escape_default", but supports additional escapes that rust does
// not. For strings that don't contain these unusual characters, it's identical to 'escape_default'.
fn escape_character(c: char) -> String {
    match c {
        '\u{07}' => "\\a".to_string(),
        '\u{08}' => "\\b".to_string(),
        '\u{0b}' => "\\v".to_string(),
        '\u{0c}' => "\\f".to_string(),
        '\u{1b}' => "\\e".to_string(),
        c => {
            // escape_default does the right thing for \t, \r, \n, and unicode
            c.escape_default().to_string()
        }
    }
}

/// Error type of [unescape](unescape).
#[derive(Debug, Error, PartialEq)]
pub enum UnescapeError {
    #[error("invalid escape {escape} at {index} in {string}")]
    InvalidEscape {
        escape: String,
        index: usize,
        string: String,
    },
    #[error("\\u could not be parsed at {index} in {string}: {source}")]
    InvalidUnicode {
        #[source]
        source: ParseUnicodeError,
        index: usize,
        string: String,
    },
}

/// Source error type of [UnescapeError::InvalidUnicode](UnescapeError::InvalidUnicode).
#[derive(Debug, Error, PartialEq)]
pub enum ParseUnicodeError {
    #[error("expected '{{' character in unicode escape")]
    BraceNotFound,
    #[error("could not parse {string} as u32 hex: {source}")]
    ParseHexFailed {
        #[source]
        source: ParseIntError,
        string: String,
    },
    #[error("could not parse {value} as a unicode char")]
    ParseUnicodeFailed { value: u32 },
}

/// Parse the provided shell-like quoted string, such as one produced by [escape](escape).
///
/// # Details
///
/// Unescape is able to handle single quotes (which cannot contain any additional escapes), double
/// quotes (which may contain a set of escapes similar to ANSI-C, i.e. '\n', '\r', '\'', etc.
/// Unescape will also parse unicode escapes of the form "\u{01ff}". See
/// [char::escape_unicode](std::char::EscapeUnicode) in the Rust standard library for more
/// information on these escapes.
///
/// Multiple different quoting styles may be used in one string, for example, the following string
/// is valid: `'some spaces'_some_unquoted_"and a \t tab"`.
///
/// The full set of supported escapes between double quotes may be found below:
///
/// | Escape | Unicode | Description |
/// |--------|---------|-------------|
/// | \a     | \u{07}  | Bell        |
/// | \b     | \u{08}  | Backspace   |
/// | \v     | \u{0B}  | Vertical tab |
/// | \f     | \u{0C}  | Form feed |
/// | \n     | \u{0A}  | Newline |
/// | \r     | \u{0D}  | Carriage return |
/// | \t     | \u{09}  | Tab
/// | \e     | \u{1B}  | Escape |
/// | \E     | \u{1B}  | Escape |
/// | \\     | \u{5C}  | Backslash |
/// | \'     | \u{27}  | Single quote |
/// | \"     | \u{22}  | Double quote |
/// | \$     | \u{24}  | Dollar sign (sh compatibility) |
/// | \`     | \u{60}  | Backtick (sh compatibility) |
/// | \u{XX} | \u{XX}  | Unicode character with hex code XX |
///
/// # Errors
///
/// The returned result can display a human readable error if the string cannot be parsed as a
/// valid quoted string.
///
/// # Examples
/// ```
/// use snailquote::unescape;
/// # // The println/assert duplication is because I want to show the output you'd get without
/// # // rust's string quoting/escaping getting in the way
/// # // Ideally we could just assert on stdout, not duplicate, see
/// # // https://github.com/rust-lang/rfcs/issues/2270
/// println!("{}", unescape("foo").unwrap());
/// // foo
/// # assert_eq!(unescape("foo").unwrap(), "foo");
/// println!("{}", unescape("'String with spaces'").unwrap());
/// // String with spaces
/// # assert_eq!(unescape("'String with spaces'").unwrap(), "String with spaces");
/// println!("{}", unescape("\"new\\nline\"").unwrap());
/// // new
/// // line
/// # assert_eq!(unescape("\"new\\nline\"").unwrap(), "new\nline");
/// println!("{}", unescape("'some spaces'_some_unquoted_\"and a \\t tab\"").unwrap());
/// // some spaces_some_unquoted_and a 	 tab
/// # assert_eq!(unescape("'some spaces'_some_unquoted_\"and a \\t tab\"").unwrap(), "some spaces_some_unquoted_and a \t tab");
/// ```
pub fn unescape(s: &str) -> Result<String, UnescapeError> {
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    let mut chars = s.chars().enumerate();

    let mut res = String::with_capacity(s.len());

    while let Some((idx, c)) = chars.next() {
        // when in a single quote, no escapes are possible
        if in_single_quote {
            if c == '\'' {
                in_single_quote = false;
                continue;
            }
        } else if in_double_quote {
            if c == '"' {
                in_double_quote = false;
                continue;
            }

            if c == '\\' {
                match chars.next() {
                    None => {
                        return Err(UnescapeError::InvalidEscape {
                            escape: format!("{}", c),
                            index: idx,
                            string: String::from(s),
                        });
                    }
                    Some((idx, c2)) => {
                        res.push(match c2 {
                            'a' => '\u{07}',
                            'b' => '\u{08}',
                            'v' => '\u{0B}',
                            'f' => '\u{0C}',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'e' | 'E' => '\u{1B}',
                            '\\' => '\\',
                            '\'' => '\'',
                            '"' => '"',
                            '$' => '$',
                            '`' => '`',
                            ' ' => ' ',
                            'u' => parse_unicode(&mut chars).map_err(|x| {
                                UnescapeError::InvalidUnicode {
                                    source: x,
                                    index: idx,
                                    string: String::from(s),
                                }
                            })?,
                            _ => {
                                return Err(UnescapeError::InvalidEscape {
                                    escape: format!("{}{}", c, c2),
                                    index: idx,
                                    string: String::from(s),
                                });
                            }
                        });
                        continue;
                    }
                };
            }
        } else if c == '\'' {
            in_single_quote = true;
            continue;
        } else if c == '"' {
            in_double_quote = true;
            continue;
        }

        res.push(c);
    }

    Ok(res)
}

// parse_unicode takes an iterator over characters and attempts to extract a single unicode
// character from it.
// It parses escapes of the form '\u{65b9}', but this internal helper function expects the cursor
// to be advanced to between the 'u' and '{'.
// It also expects to be passed an iterator which includes the index for the purpose of advancing
// it  as well, such as is produced by enumerate.
fn parse_unicode<I>(chars: &mut I) -> Result<char, ParseUnicodeError>
where
    I: Iterator<Item = (usize, char)>,
{
    match chars.next() {
        Some((_, '{')) => {}
        _ => {
            return Err(ParseUnicodeError::BraceNotFound);
        }
    }

    let unicode_seq: String = chars
        .take_while(|&(_, c)| c != '}')
        .map(|(_, c)| c)
        .collect();

    u32::from_str_radix(&unicode_seq, 16)
        .map_err(|e| ParseUnicodeError::ParseHexFailed {
            source: e,
            string: unicode_seq,
        })
        .and_then(|u| {
            char::from_u32(u).ok_or_else(|| ParseUnicodeError::ParseUnicodeFailed { value: u })
        })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_escape() {
        let test_cases = vec![
            ("東方", "東方"),
            ("\"'", r#""\"'""#),
            ("\\", "\"\\\\\""),
            ("spaces only", "'spaces only'"),
            ("some\ttabs", "\"some\\ttabs\""),
            ("💩", "💩"),
            ("\u{202e}RTL", "\"\\u{202e}RTL\""),
            ("no\u{202b}space", "\"no\\u{202b}space\""),
            ("cash $ money $$ \t", "\"cash \\$ money \\$\\$ \\t\""),
            ("back ` tick `` \t", "\"back \\` tick \\`\\` \\t\""),
            (
                "\u{07}\u{08}\u{0b}\u{0c}\u{0a}\u{0d}\u{09}\u{1b}\u{1b}\u{5c}\u{27}\u{22}",
                "\"\\a\\b\\v\\f\\n\\r\\t\\e\\e\\\\'\\\"\"",
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(escape(s), expected);
        }
    }

    #[test]
    fn test_unescape() {
        assert_eq!(unescape("\"\\u{6771}\\u{65b9}\""), Ok("東方".to_string()));
        assert_eq!(unescape("東方"), Ok("東方".to_string()));
        assert_eq!(unescape("\"\\\\\"'\"\"'"), Ok("\\\"\"".to_string()));
        assert_eq!(unescape("'\"'"), Ok("\"".to_string()));
        assert_eq!(unescape("'\"'"), Ok("\"".to_string()));
        // Every escape between double quotes
        assert_eq!(
            unescape("\"\\a\\b\\v\\f\\n\\r\\t\\e\\E\\\\\\'\\\"\\u{09}\\$\\`\""),
            Ok(
                "\u{07}\u{08}\u{0b}\u{0c}\u{0a}\u{0d}\u{09}\u{1b}\u{1b}\u{5c}\u{27}\u{22}\u{09}$`"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_unescape_error() {
        assert_eq!(
            unescape("\"\\x\""),
            Err(UnescapeError::InvalidEscape {
                escape: "\\x".to_string(),
                index: 2,
                string: "\"\\x\"".to_string()
            })
        );
        assert_eq!(
            unescape("\"\\u6771}\""),
            Err(UnescapeError::InvalidUnicode {
                source: ParseUnicodeError::BraceNotFound,
                index: 2,
                string: "\"\\u6771}\"".to_string()
            })
        );
        // Can't compare ParseIntError directly until 'int_error_matching' becomes stable
        assert_eq!(
            format!("{}", unescape("\"\\u{qqqq}\"").err().unwrap()),
            "\\u could not be parsed at 2 in \"\\u{qqqq}\": could not parse qqqq as u32 hex: invalid digit found in string"
        );
        assert_eq!(
            unescape("\"\\u{ffffffff}\""),
            Err(UnescapeError::InvalidUnicode {
                source: ParseUnicodeError::ParseUnicodeFailed { value: 0xffffffff },
                index: 2,
                string: "\"\\u{ffffffff}\"".to_string()
            })
        );
    }

    #[test]
    fn test_round_trip() {
        let test_cases = vec![
            "東方",
            "foo bar baz",
            "\\",
            "\0",
            "\"'",
            "\"'''''\"()())}{{}{}{{{!////",
        ];

        for case in test_cases {
            assert_eq!(unescape(&escape(case)), Ok(case.to_owned()));
        }
    }

    quickcheck! {
        fn round_trips(s: String) -> bool {
            s == unescape(&escape(&s)).unwrap()
        }
    }

    #[test]
    fn test_os_release_parsing() {
        let tests = vec![
            ("fedora-19", "Fedora 19 (Schrödinger’s Cat)"),
            ("fedora-29", "Fedora 29 (Twenty Nine)"),
            ("gentoo", "Gentoo/Linux"),
            ("fictional", "Fictional $ OS: ` edition"),
        ];

        for (file, pretty_name) in tests {
            let mut data = String::new();
            std::fs::File::open(format!("./src/testdata/os-releases/{}", file))
                .unwrap()
                .read_to_string(&mut data)
                .unwrap();

            let mut found_prettyname = false;
            // partial os-release parser
            for line in data.lines() {
                if line.trim().starts_with("#") {
                    continue;
                }
                let mut iter = line.splitn(2, "=");
                let key = iter.next().unwrap();
                let value = iter.next().unwrap();
                // assert we can parse the value
                let unescaped = unescape(value).unwrap();
                if key == "PRETTY_NAME" {
                    assert_eq!(unescaped, pretty_name);
                    found_prettyname = true;
                }
            }
            assert!(
                found_prettyname,
                "expected os-release to have 'PRETTY_NAME' key"
            );
        }
    }
}
