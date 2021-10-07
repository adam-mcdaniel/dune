use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("strip") => Expression::builtin("strip", |args, env| {
            super::check_exact_args_len("strip", &args, 1)?;
            Ok(crate::strip_ansi_escapes(args[0].eval(env)?).into())
        }, "strips all colors and styling from a string"),

        String::from("wrap") => Expression::builtin("wrap", wrap,
            "wrap text such that it fits in a specific number of columns"),

        String::from("href") => Expression::builtin("href", href,
            "create a hyperlink on the console"),

        String::from("bold") => Expression::builtin("bold", |args, env| {
            Ok(format!("\x1b[1m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to bold on the console"),

        String::from("faint") => Expression::builtin("faint", |args, env| {
            Ok(format!("\x1b[2m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to italics on the console"),

        String::from("italics") => Expression::builtin("italics", |args, env| {
            Ok(format!("\x1b[3m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to italics on the console"),

        String::from("underline") => Expression::builtin("underline", |args, env| {
            Ok(format!("\x1b[4m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "underline text on the console"),

        String::from("blink") => Expression::builtin("blink", |args, env| {
            Ok(format!("\x1b[5m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "blink text on the console"),

        String::from("invert") => Expression::builtin("invert", |args, env| {
            Ok(format!("\x1b[7m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "invert text on the console"),

        String::from("strike") => Expression::builtin("strike", |args, env| {
            Ok(format!("\x1b[9m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "strike out text on the console"),

        String::from("black") => Expression::builtin("black", |args, env| {
            Ok(format!("\x1b[90m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to black on the console"),

        String::from("red") => Expression::builtin("red", |args, env| {
            Ok(format!("\x1b[91m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to red on the console"),

        String::from("green") => Expression::builtin("green", |args, env| {
            Ok(format!("\x1b[92m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to green on the console"),

        String::from("yellow") => Expression::builtin("yellow", |args, env| {
            Ok(format!("\x1b[93m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to yellow on the console"),

        String::from("blue") => Expression::builtin("blue", |args, env| {
            Ok(format!("\x1b[94m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to blue on the console"),

        String::from("magenta") => Expression::builtin("magenta", |args, env| {
            Ok(format!("\x1b[95m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to magenta on the console"),

        String::from("cyan") => Expression::builtin("cyan", |args, env| {
            Ok(format!("\x1b[96m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
        }, "convert text to cyan on the console"),

        String::from("white") => Expression::builtin("white", |args, env| {
            Ok(format!("\x1b[97m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
        }, "convert text to white on the console"),

        String::from("dark") => b_tree_map! {
            String::from("black") => Expression::builtin("black", |args, env| {
                Ok(format!("\x1b[30m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to black on the console"),

            String::from("red") => Expression::builtin("red", |args, env| {
                Ok(format!("\x1b[31m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to red on the console"),

            String::from("green") => Expression::builtin("green", |args, env| {
                Ok(format!("\x1b[32m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to green on the console"),

            String::from("yellow") => Expression::builtin("yellow", |args, env| {
                Ok(format!("\x1b[33m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to yellow on the console"),

            String::from("blue") => Expression::builtin("blue", |args, env| {
                Ok(format!("\x1b[34m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to blue on the console"),

            String::from("magenta") => Expression::builtin("magenta", |args, env| {
                Ok(format!("\x1b[35m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to magenta on the console"),

            String::from("cyan") => Expression::builtin("cyan", |args, env| {
                Ok(format!("\x1b[36m{}\x1b[m\x1b[0m", args[0].eval(env)?).into())
            }, "convert text to cyan on the console"),

            String::from("white") => Expression::builtin("white", |args, env| {
                Ok(format!("\x1b[37m{}\x1b[m\x6b[0m", args[0].eval(env)?).into())
            }, "convert text to white on the console"),
        }.into()
    })
    .into()
}

fn wrap(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("wrap", &args, 2)?;
    match args[1].eval(env)? {
        Expression::Integer(columns) => {
            Ok(textwrap::fill(&args[0].eval(env)?.to_string(), columns as usize).into())
        }
        otherwise => Err(Error::CustomError(format!(
            "expected number of columns in wrap, but got {}",
            otherwise
        ))),
    }
}

fn href(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("href", &args, 2)?;
    Ok(format!(
        "\x1b]8;;{url}\x1b\\{text}\x1b]8;;\x1b\\",
        url = args[0].eval(env)?,
        text = args[1].eval(env)?
    )
    .into())
}
