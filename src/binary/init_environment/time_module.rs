use std::{thread, time::Duration};

use chrono::{Datelike, Timelike};
use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get() -> Expression {
    b_tree_map! {
        String::from("sleep") => Expression::builtin("sleep", sleep,
            "sleep for a given number of milliseconds"),
        String::from("now") => Expression::builtin("now", now,
            "get information about the current time"),
    }
    .into()
}

fn sleep(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("sleep", &args, 1)?;

    match args[0].eval(env)? {
        Expression::Float(n) => thread::sleep(Duration::from_millis(n as u64)),
        Expression::Integer(n) => thread::sleep(Duration::from_millis(n as u64)),
        otherwise => {
            return Err(Error::CustomError(format!(
                "expected integer or float, but got {}",
                otherwise
            )))
        }
    }

    Ok(Expression::None)
}

fn now(_: Vec<Expression>, _: &mut Environment) -> Result<Expression, Error> {
    let now = chrono::Local::now();

    Ok(Expression::Map(b_tree_map! {
        String::from("year") => Expression::Integer(now.year() as i64),
        String::from("month") => Expression::Integer(now.month() as i64),
        String::from("day") => Expression::Integer(now.day() as i64),
        String::from("hour") => Expression::Integer(now.hour() as i64),
        String::from("time") => Expression::Map(b_tree_map! {
            String::from("str") => Expression::String(now.time().format("%-I:%M %p").to_string()),
        }),
        String::from("date") => Expression::Map(b_tree_map! {
            String::from("str") => Expression::String(now.format("%D").to_string()),
        }),
    }))
}
