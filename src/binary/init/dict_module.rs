use super::Int;
use common_macros::b_tree_map;
use dune::Expression;

pub(crate) fn flatten(expr: Expression) -> Vec<Expression> {
    match expr {
        Expression::List(list) => {
            let mut new_list = Vec::new();
            for item in list {
                new_list.extend(flatten(item));
            }
            new_list
        }
        Expression::Map(map) => {
            let mut new_list = Vec::new();
            for (_, item) in map {
                new_list.extend(flatten(item));
            }
            new_list
        }
        expr => vec![expr],
    }
}

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("flatten") => Expression::builtin("flatten", |args, env| {
            super::check_exact_args_len("flatten", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(Expression::List(flatten(expr)))
        }, "flatten a list"),

        String::from("items") => Expression::builtin("items", |args, env| {
            super::check_exact_args_len("items", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(map) => Expression::List(map.into_iter().map(|(k, v)| Expression::List(vec![k.into(), v])).collect()),
                Expression::List(list) => Expression::List(list.into_iter().enumerate().map(|(i, v)| Expression::List(vec![Expression::Integer(i as Int), v])).collect()),
                _ => Expression::None
            })
        }, "get the items of a map or list"),

        String::from("keys") => Expression::builtin("keys", |args, env| {
            super::check_exact_args_len("keys", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(map) => Expression::List(map.into_keys().map(|k| k.into()).collect()),
                _ => Expression::None
            })
        }, "get the keys of a map"),

        String::from("values") => Expression::builtin("values", |args, env| {
            super::check_exact_args_len("values", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(map) => Expression::List(map.into_values().collect()),
                _ => Expression::None
            })
        }, "get the values of a map"),

        String::from("insert") => Expression::builtin("insert", |args, env| {
            super::check_exact_args_len("insert", &args, 3)?;
            let expr = args[0].clone().eval(env)?;
            let key = args[1].clone().eval(env)?;
            let value = args[2].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(mut map) => {
                    map.insert(key.to_string(), value);
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "insert a key-value pair into a map"),

        String::from("remove") => Expression::builtin("remove", |args, env| {
            super::check_exact_args_len("remove", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let key = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(mut map) => {
                    map.remove(&key.to_string());
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "remove a key-value pair from a map"),

        String::from("has?") => Expression::builtin("has?", |args, env| {
            super::check_exact_args_len("has?", &args, 2)?;
            let expr = args[0].clone().eval(env)?;
            let key = args[1].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(map) => Expression::Boolean(map.contains_key(&key.to_string())),
                _ => Expression::None
            })
        }, "check if a map has a key"),

        String::from("len") => Expression::builtin("len", |args, env| {
            super::check_exact_args_len("len", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::Map(map) => Expression::Integer(map.len() as Int),
                Expression::List(list) => Expression::Integer(list.len() as Int),
                Expression::String(string) => Expression::Integer(string.len() as Int),
                Expression::Bytes(bytes) => Expression::Integer(bytes.len() as Int),
                _ => Expression::None
            })
        }, "get the length of a map, list, string, or bytes"),

        String::from("from-items") => Expression::builtin("from-items", |args, env| {
            super::check_exact_args_len("from-items", &args, 1)?;
            let expr = args[0].clone().eval(env)?;
            Ok(match expr {
                Expression::List(list) => {
                    let mut map = std::collections::BTreeMap::new();
                    for item in list {
                        if let Expression::List(item) = item {
                            if item.len() == 2 {
                                map.insert(item[0].to_string(), item[1].clone());
                            }
                        }
                    }
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "create a map from a list of key-value pairs"),

        String::from("union") => Expression::builtin("union", |args, env| {
            super::check_exact_args_len("union", &args, 2)?;
            let expr1 = args[0].clone().eval(env)?;
            let expr2 = args[1].clone().eval(env)?;
            Ok(match (expr1, expr2) {
                (Expression::Map(map1), Expression::Map(map2)) => {
                    let mut map = map1.clone();
                    for (key, value) in map2 {
                        map.insert(key, value);
                    }
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "combine two maps"),

        String::from("intersect") => Expression::builtin("intersect", |args, env| {
            super::check_exact_args_len("intersect", &args, 2)?;
            let expr1 = args[0].clone().eval(env)?;
            let expr2 = args[1].clone().eval(env)?;
            Ok(match (expr1, expr2) {
                (Expression::Map(map1), Expression::Map(map2)) => {
                    let mut map = std::collections::BTreeMap::new();
                    for (key, value) in map1 {
                        if map2.contains_key(&key) {
                            map.insert(key, value);
                        }
                    }
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "get the intersection of two maps"),

        String::from("difference") => Expression::builtin("difference", |args, env| {
            super::check_exact_args_len("difference", &args, 2)?;
            let expr1 = args[0].clone().eval(env)?;
            let expr2 = args[1].clone().eval(env)?;
            Ok(match (expr1, expr2) {
                (Expression::Map(map1), Expression::Map(map2)) => {
                    let mut map = map1.clone();
                    for key in map2.keys() {
                        map.remove(key);
                    }
                    Expression::Map(map)
                },
                _ => Expression::None
            })
        }, "get the difference of two maps"),
    })
    .into()
}
