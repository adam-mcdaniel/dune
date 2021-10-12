use detached_str::{Str, StrSlice};

use core::{cmp::max, fmt};

use crate::Diagnostic;

use super::{Expression, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    CannotApply(Expression, Vec<Expression>),
    SymbolNotDefined(String),
    CommandFailed(String, Vec<Expression>),
    ForNonList(Expression),
    RecursionDepth(Expression),
    CustomError(String),
    SyntaxError(Str, SyntaxError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CannotApply(expr, args) => {
                write!(f, "cannot apply `{:?}` to the arguments {:?}", expr, args)
            }
            Self::SymbolNotDefined(name) => {
                write!(f, "symbol \"{}\" not defined", name)
            }
            Self::RecursionDepth(expr) => {
                write!(f, "recursion depth exceeded while evaluating {:?}", expr)
            }
            Self::CommandFailed(name, args) => {
                write!(
                    f,
                    "command `{:?}` failed",
                    Expression::Apply(Box::new(Expression::Symbol(name.clone())), args.clone())
                )
            }
            Self::ForNonList(nonlist) => {
                write!(f, "attempted to iterate over non-list `{:?}`", nonlist)
            }
            Self::CustomError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(string, err) => fmt_syntax_error(string, err, f),
        }
    }
}

fn fmt_syntax_error(string: &Str, err: &SyntaxError, f: &mut fmt::Formatter) -> fmt::Result {
    match err {
        SyntaxError::Expected {
            input,
            expected,
            found,
            hint,
        } => {
            write!(f, "{}{}syntax error{}: ", RED_START, BOLD, RESET)?;
            write!(f, "expected {}", expected)?;
            if let Some(found) = found {
                write!(f, ", found {}", found)?;
            }
            writeln!(f)?;
            print_error_lines(string, *input, f, 72)?;
            if let Some(hint) = *hint {
                writeln!(f, "    hint: {}", hint)?;
            }
            Ok(())
        }
        SyntaxError::TokenizationErrors(errors) => {
            for err in errors.iter() {
                fmt_token_error(string, err, f)?;
            }
            Ok(())
        }
        SyntaxError::ExpectedChar { expected, at } => {
            write!(f, "{}{}syntax error{}: ", RED_START, BOLD, RESET)?;
            writeln!(f, "expected {:?}", expected)?;
            if let Some(at) = *at {
                print_error_lines(string, at, f, 72)?;
            }
            Ok(())
        }
        SyntaxError::NomError { kind, at, cause } => {
            write!(f, "{}{}unexpected syntax error{}: ", RED_START, BOLD, RESET)?;
            writeln!(f, "`{:?}`", kind)?;
            if let Some(at) = *at {
                print_error_lines(string, at, f, 72)?;
            }
            if let Some(cause) = cause {
                fmt_syntax_error(string, cause, f)?;
            }
            Ok(())
        }
        SyntaxError::InternalError => {
            writeln!(f, "{}{}unexpected syntax error{}", RED_START, BOLD, RESET)
        }
    }
}

fn fmt_token_error(string: &Str, err: &Diagnostic, f: &mut fmt::Formatter) -> fmt::Result {
    match err {
        Diagnostic::Valid => Ok(()),
        Diagnostic::InvalidStringEscapes(ranges) => {
            for &at in ranges.iter() {
                write!(f, "{}{}syntax error{}: ", RED_START, BOLD, RESET)?;
                let escape = at.to_str(string).trim();
                writeln!(f, "invalid string escape sequence `{}`", escape)?;
                print_error_lines(string, at, f, 72)?;
            }
            Ok(())
        }
        &Diagnostic::InvalidNumber(at) => {
            write!(f, "{}{}syntax error{}: ", RED_START, BOLD, RESET)?;
            let num = at.to_str(string).trim();
            writeln!(f, "invalid number `{}`", num)?;
            print_error_lines(string, at, f, 72)
        }
        &Diagnostic::IllegalChar(at) => {
            write!(f, "{}{}syntax error{}: ", RED_START, BOLD, RESET)?;
            writeln!(f, "invalid token {:?}", at.to_str(string))?;
            print_error_lines(string, at, f, 72)
        }
        &Diagnostic::NotTokenized(at) => {
            write!(f, "{}{}error{}: ", RED_START, BOLD, RESET)?;
            writeln!(
                f,
                "there are leftover tokens after tokenizing: {}",
                at.to_str(string)
            )?;
            print_error_lines(string, at, f, 72)
        }
    }
}

fn print_error_lines(
    string: &Str,
    at: StrSlice,
    f: &mut fmt::Formatter,
    max_width: usize,
) -> fmt::Result {
    let mut lines = at.to_str(string).lines().collect::<Vec<&str>>();
    if lines.is_empty() {
        lines.push("");
    }
    let singleline = lines.len() == 1;

    let before = &string[..at.start()];
    let after = &string[at.end()..];

    let line_before = before.lines().next_back().unwrap_or_default();
    let line_after = after.lines().next().unwrap_or_default();

    let first_line_number = max(before.lines().count(), 1);

    writeln!(f, "      |")?;

    if singleline {
        let before_len = line_before.chars().take(max_width).count().min(max_width);

        let line = line_before
            .chars()
            .take(max_width)
            .chain(RED_START.chars())
            .chain(lines[0].chars())
            .chain(RESET.chars())
            .chain(line_after.chars().take(max_width - before_len))
            .collect::<String>();

        writeln!(f, "{:>5} | {}", first_line_number, line)?;
    } else {
        let first_line = line_before
            .chars()
            .chain(RED_START.chars())
            .chain(lines[0].chars())
            .take(max_width)
            .chain(RESET.chars())
            .collect::<String>();
        write!(f, "{:>5} | {}", first_line_number, first_line)?;

        for (i, line) in lines.iter().copied().enumerate().skip(1) {
            let line = RED_START
                .chars()
                .chain(line.chars().take(max_width))
                .chain(RESET.chars())
                .collect::<String>();
            write!(f, "\n{:>5} | {}", first_line_number + i, line)?;
        }

        let last_len = lines.last().unwrap().chars().count();
        let suffix = line_after
            .chars()
            .take(max_width - last_len)
            .chain(RESET.chars())
            .collect::<String>();
        writeln!(f, "\n{}", suffix)?;
    }

    writeln!(f, "      |")?;

    Ok(())
}

const RED_START: &str = "\x1b[38;5;9m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[m\x1b[0m";
