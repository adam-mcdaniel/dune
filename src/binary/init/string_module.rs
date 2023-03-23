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
        String::from("isws") => Expression::builtin("isws", |args, env| {
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

        String::from("isalpha") => Expression::builtin("isalpha", |args, env| {
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

        String::from("isalphanumeric") => Expression::builtin("isalphanumeric", |args, env| {
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

        String::from("isnumeric") => Expression::builtin("isnumeric", |args, env| {
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
    })
    .into()
}
