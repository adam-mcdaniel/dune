use common_macros::b_tree_map;
use dune::{parse_script, Environment, Error, Expression, SyntaxError};
use json::JsonValue;
use std::collections::BTreeMap;

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("toml") => Expression::builtin("toml", parse_toml, "parse a TOML value into a Dune expression"),
        String::from("json") => Expression::builtin("json", parse_json, "parse a JSON value into a Dune expression"),
        String::from("expr") => Expression::builtin("expr", parse_expr, "parse a Dune script"),
    })
    .into()
}

fn parse_expr(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("json", &args, 1)?;
    let script = args[0].eval(env)?.to_string();
    match parse_script(&script) {
        Ok(val) => Ok(val),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            Err(Error::SyntaxError(script.into(), e))
        }
        Err(nom::Err::Incomplete(_)) => Err(Error::SyntaxError(
            script.into(),
            SyntaxError::InternalError,
        )),
    }
}

fn parse_json(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("json", &args, 1)?;
    let text = args[0].eval(env)?.to_string();
    if let Ok(val) = json::parse(&text) {
        Ok(json_to_expr(val))
    } else {
        Err(Error::CustomError(format!(
            "could not parse `{}` as JSON",
            text
        )))
    }
}

fn json_to_expr(val: JsonValue) -> Expression {
    match val {
        JsonValue::Null => Expression::None,
        JsonValue::Boolean(b) => Expression::Boolean(b),
        JsonValue::Number(n) => Expression::Float(n.into()),
        JsonValue::Short(s) => Expression::String(s.to_string()),
        JsonValue::String(s) => Expression::String(s),
        JsonValue::Array(a) => {
            let mut v = Vec::new();
            for e in a {
                v.push(json_to_expr(e));
            }
            Expression::List(v)
        }
        JsonValue::Object(o) => {
            let mut m = BTreeMap::new();
            for (k, v) in o.iter() {
                m.insert(k.to_string(), json_to_expr(v.clone()));
            }
            Expression::Map(m)
        }
    }
}

fn parse_toml(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("toml", &args, 1)?;
    let text = args[0].eval(env)?.to_string();
    if let Ok(val) = text.parse::<toml::Value>() {
        Ok(toml_to_expr(val))
    } else {
        Err(Error::CustomError(format!(
            "could not parse `{}` as TOML",
            text
        )))
    }
}

fn toml_to_expr(val: toml::Value) -> Expression {
    match val {
        toml::Value::Boolean(b) => Expression::Boolean(b),
        toml::Value::Float(n) => Expression::Float(n),
        toml::Value::Integer(n) => Expression::Integer(n),
        toml::Value::Datetime(s) => Expression::String(s.to_string()),
        toml::Value::String(s) => Expression::String(s),
        toml::Value::Array(a) => {
            let mut v = Vec::new();
            for e in a {
                v.push(toml_to_expr(e));
            }
            Expression::List(v)
        }
        toml::Value::Table(o) => {
            let mut m = BTreeMap::new();
            for (k, v) in o.iter() {
                m.insert(k.to_string(), toml_to_expr(v.clone()));
            }
            Expression::Map(m)
        }
    }
}
