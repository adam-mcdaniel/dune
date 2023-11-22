use super::fn_module::curry;
use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

fn split(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(format!(
            "expected 2 arguments, got {}",
            args.len()
        )));
    }
    match (args[0].eval(env)?, args[1].eval(env)?) {
        (
            Expression::Symbol(x) | Expression::String(x),
            Expression::Symbol(y) | Expression::String(y),
        ) => {
            let mut v = Vec::new();
            for s in y.split(&x) {
                v.push(Expression::String(s.to_string()));
            }
            Ok(Expression::List(v))
        }
        (a, b) => Err(Error::CustomError(format!(
            "expected string, got values {} and {}",
            a, b
        ))),
    }
}

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("is-whitespace?") => Expression::builtin("is-whitespace?", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_whitespace())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string whitespace?"),

        String::from("is-alpha?") => Expression::builtin("is-alpha?", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_alphabetic())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string alphabetic?"),

        String::from("is-alphanumeric?") => Expression::builtin("is-alphanumeric?", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_alphanumeric())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string alphanumeric?"),

        String::from("is-numeric?") => Expression::builtin("is-numeric?", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_numeric())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string numeric?"),

        String::from("split") => Expression::builtin("split", |args, env| {
            curry(Expression::builtin("", split, ""), 2, env)?
                .eval(env)?
                .apply(args)
                .eval(env)
        }, "split a string on a given character"),

        String::from("to-lower") => Expression::builtin("to-lower", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::String(x.to_lowercase()))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "convert a string to lowercase"),

        String::from("to-upper") => Expression::builtin("to-upper", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::String(x.to_uppercase()))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "convert a string to uppercase"),

        String::from("to-title") => Expression::builtin("to-title", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    let mut title = String::new();
                    let mut capitalize = true;
                    for c in x.chars() {
                        if capitalize {
                            title.push(c.to_uppercase().next().unwrap());
                            capitalize = false;
                        } else {
                            title.push(c.to_lowercase().next().unwrap());
                        }
                        if c.is_whitespace() {
                            capitalize = true;
                        }
                    }
                    Ok(Expression::String(title))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "convert a string to title case"),

        String::from("is-lower") => Expression::builtin("is-lower", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_lowercase())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string lowercase?"),

        String::from("is-upper") => Expression::builtin("is-upper", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::Boolean(x.chars().all(|c| c.is_uppercase())))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string uppercase?"),

        String::from("is-title") => Expression::builtin("is-title", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    let mut title = String::new();
                    let mut capitalize = true;
                    for c in x.chars() {
                        if capitalize {
                            title.push(c.to_uppercase().next().unwrap());
                            capitalize = false;
                        } else {
                            title.push(c.to_lowercase().next().unwrap());
                        }
                        if c.is_whitespace() {
                            capitalize = true;
                        }
                    }
                    Ok(Expression::Boolean(x == title))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "is this string title case?"),

        String::from("reverse") => Expression::builtin("reverse", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    let mut reversed = String::new();
                    for c in x.chars().rev() {
                        reversed.push(c);
                    }
                    Ok(Expression::String(reversed))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "reverse a string"),

        String::from("join") => Expression::builtin("join", |args, env| {
            super::check_exact_args_len("join", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let separator = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::List(list) => {
                    let mut joined = String::new();
                    for (i, item) in list.iter().enumerate() {
                        if i != 0 {
                            joined.push_str(&separator.to_string());
                        }
                        joined.push_str(&item.to_string());
                    }
                    Expression::String(joined)
                }
                _ => Expression::None,
            })
        }, "join a list of strings with a separator"),

        String::from("lines") => Expression::builtin("lines", |args, env| {
            super::check_exact_args_len("lines", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => Expression::List(
                    x.lines()
                        .map(|line| Expression::String(line.to_string()))
                        .collect(),
                ),
                _ => Expression::None,
            })
        }, "split a string into lines"),

        String::from("chars") => Expression::builtin("chars", |args, env| {
            super::check_exact_args_len("chars", &args, 1)?;
            // Ok(match expr {
            //     Expression::Symbol(x) | Expression::String(x) => Expression::List(
            //         x.chars()
            //             .map(|c| Expression::String(c.to_string()))
            //             .collect(),
            //     ),
            //     _ => Expression::None,
            // })

            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => Ok(Expression::List(
                    x.chars()
                        .map(|ch| Expression::String(ch.to_string()))
                        .collect::<Vec<Expression>>(),
                )),
                otherwise => Err(Error::CustomError(format!(
                    "cannot get characters of non-string {}",
                    otherwise
                ))),
            }
        }, "split a string into characters"),

        String::from("words") => Expression::builtin("words", |args, env| {
            super::check_exact_args_len("words", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => Expression::List(
                    x.split_whitespace()
                        .map(|word| Expression::String(word.to_string()))
                        .collect(),
                ),
                _ => Expression::None,
            })
        }, "split a string into words"),

        String::from("paragraphs") => Expression::builtin("paragraphs", |args, env| {
            super::check_exact_args_len("paragraphs", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => Expression::List(
                    x.split("\n\n")
                        .map(|paragraph| Expression::String(paragraph.to_string()))
                        .collect(),
                ),
                _ => Expression::None,
            })
        }, "split a string into paragraphs"),

        String::from("split-at") => Expression::builtin("split-at", |args, env| {
            super::check_exact_args_len("split-at", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let index = args[1].clone().eval(env)?;
            Ok(match (expr, index) {
                (Expression::Symbol(x) | Expression::String(x), Expression::Integer(i)) => {
                    let mut split = Vec::new();
                    split.push(Expression::String(x[..i as usize].to_string()));
                    split.push(Expression::String(x[i as usize..].to_string()));
                    Expression::List(split)
                }
                _ => Expression::None,
            })
        }, "split a string at a given index"),

        String::from("trim") => Expression::builtin("trim", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::String(x.trim().to_string()))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "trim whitespace from a string"),

        String::from("trim-start") => Expression::builtin("trim-start", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::String(x.trim_start().to_string()))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "trim whitespace from the start of a string"),

        String::from("trim-end") => Expression::builtin("trim-end", |args, env| {
            match args[0].eval(env)? {
                Expression::Symbol(x) | Expression::String(x) => {
                    Ok(Expression::String(x.trim_end().to_string()))
                }
                otherwise => Err(Error::CustomError(format!(
                    "expected string, got value {}",
                    otherwise
                ))),
            }
        }, "trim whitespace from the end of a string"),

        String::from("replace") => Expression::builtin("replace", |args, env| {
            super::check_exact_args_len("replace", &args, 3)?;
            let expr = args[0].clone().eval(env)?;
            let old = args[1].clone().eval(env)?;
            let new = args[2].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => {
                    Expression::String(x.replace(&old.to_string(), &new.to_string()))
                }
                _ => Expression::None,
            })
        }, "replace all instances of a substring in a string with another string"),

        String::from("starts-with?") => Expression::builtin("starts-with?", |args, env| {
            super::check_exact_args_len("starts-with?", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let prefix = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => {
                    Expression::Boolean(x.starts_with(&prefix.to_string()))
                }
                _ => Expression::None,
            })
        }, "check if a string starts with a given substring"),

        String::from("ends-with?") => Expression::builtin("ends-with?", |args, env| {
            super::check_exact_args_len("ends-with?", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let suffix = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => {
                    Expression::Boolean(x.ends_with(&suffix.to_string()))
                }
                _ => Expression::None,
            })
        }, "check if a string ends with a given substring"),

        String::from("contains?") => Expression::builtin("contains?", |args, env| {
            super::check_exact_args_len("contains?", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let substring = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::Symbol(x) | Expression::String(x) => {
                    Expression::Boolean(x.contains(&substring.to_string()))
                }
                _ => Expression::None,
            })
        }, "check if a string contains a given substring"),
    })
    .into()
}
