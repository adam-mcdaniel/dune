use super::curry;
use super::Int;
use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};
pub fn get() -> Expression {
    (b_tree_map! {
        String::from("list") => Expression::builtin("list", list,
            "create a list from a variable number of arguments"),
        String::from("tail") => Expression::builtin("tail", tail,
            "get the tail of a list"),
        String::from("head") => Expression::builtin("head", head,
            "get the head of a list"),
        String::from("cons") => Expression::builtin("cons", cons,
            "prepend an element to a list"),
        String::from("append") => Expression::builtin("append", append,
            "append an element to a list"),
        String::from("len") => Expression::builtin("len", len,
            "get the length of a list"),
        String::from("rev") => Expression::builtin("rev", rev,
            "reverse a list"),
        String::from("range") => Expression::builtin("range", range,
            "create a list of integers from a to b"),
        String::from("foldl") => Expression::builtin("foldl", foldl,
            "fold a list from the left"),
        String::from("foldr") => Expression::builtin("foldr", foldr,
            "fold a list from the right"),
        String::from("zip") => Expression::builtin("zip", zip,
            "zip two lists together"),
        String::from("unzip") => Expression::builtin("unzip", unzip,
            "unzip a list of pairs into a pair of lists"),
        String::from("take") => curry(Expression::builtin("take", take,
            "take the first n elements of a list"), 2),
        String::from("drop") => curry(Expression::builtin("drop", drop,
            "drop the first n elements of a list"), 2),
        String::from("split-at") => Expression::builtin("split-at", split_at,
            "split a list at a given index"),
        String::from("nth") => Expression::builtin("nth", nth,
            "get the nth element of a list"),
    })
    .into()
}

fn list(args: Vec<Expression>, _env: &mut Environment) -> Result<Expression, Error> {
    Ok(Expression::List(args))
}

fn tail(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 1 {
        return Err(Error::CustomError(
            "tail requires exactly one argument".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    if let Expression::List(list) = list {
        Ok(Expression::List(list.into_iter().skip(1).collect()))
    } else {
        Err(Error::CustomError(
            "tail requires a list as its argument".to_string(),
        ))
    }
}

fn head(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 1 {
        return Err(Error::CustomError(
            "head requires exactly one argument".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    if let Expression::List(list) = list {
        if list.is_empty() {
            Ok(Expression::List(list))
        } else {
            Ok(Expression::None)
        }
    } else {
        Err(Error::CustomError(
            "head requires a list as its argument".to_string(),
        ))
    }
}

fn cons(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "cons requires exactly two arguments".to_string(),
        ));
    }
    let list = args[1].eval(env)?;
    if let Expression::List(mut list) = list {
        list.insert(0, args[0].eval(env)?);
        Ok(Expression::List(list))
    } else {
        Err(Error::CustomError(
            "cons requires a list as its second argument".to_string(),
        ))
    }
}

fn append(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "append requires exactly two arguments".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    if let Expression::List(mut list) = list {
        list.push(args[1].eval(env)?);
        Ok(Expression::List(list))
    } else {
        Err(Error::CustomError(
            "append requires a list as its first argument".to_string(),
        ))
    }
}

pub(super) fn len(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 1 {
        return Err(Error::CustomError(
            "len requires exactly one argument".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    match list {
        Expression::List(list) => Ok(Expression::Integer(list.len() as Int)),
        Expression::String(string) => Ok(Expression::Integer(string.len() as Int)),
        Expression::Bytes(bytes) => Ok(Expression::Integer(bytes.len() as Int)),
        Expression::Map(map) => Ok(Expression::Integer(map.len() as Int)),
        _ => Err(Error::CustomError(
            "len requires a list or string as its argument".to_string(),
        )),
    }
}

pub(super) fn rev(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 1 {
        return Err(Error::CustomError(
            "rev requires exactly one argument".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    match list {
        Expression::List(list) => Ok(Expression::List(list.into_iter().rev().collect())),
        Expression::String(string) => Ok(Expression::String(string.chars().rev().collect())),
        Expression::Symbol(string) => Ok(Expression::Symbol(string.chars().rev().collect())),
        Expression::Bytes(bytes) => Ok(Expression::Bytes(bytes.into_iter().rev().collect())),
        _ => Err(Error::CustomError(
            "rev requires a list or string as its argument".to_string(),
        )),
    }
}

fn range(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "range requires exactly two arguments".to_string(),
        ));
    }
    let a = args[0].eval(env)?;
    let b = args[1].eval(env)?;
    if let (Expression::Integer(a), Expression::Integer(b)) = (a, b) {
        Ok(Expression::List((a..=b).map(|n| n.into()).collect()))
    } else {
        Err(Error::CustomError(
            "range requires two integers as its arguments".to_string(),
        ))
    }
}

fn foldl(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 3 {
        return Err(Error::CustomError(
            "foldl requires exactly three arguments".to_string(),
        ));
    }
    let f = args[0].eval(env)?;
    let mut acc = args[1].eval(env)?;
    let list = args[2].eval(env)?;
    if let Expression::List(list) = list {
        for item in list {
            acc = Expression::Apply(Box::new(f.clone()), vec![acc, item]).eval(env)?;
        }
        Ok(acc)
    } else {
        Err(Error::CustomError(
            "foldl requires a list as its third argument".to_string(),
        ))
    }
}

fn foldr(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 3 {
        return Err(Error::CustomError(
            "foldr requires exactly three arguments".to_string(),
        ));
    }
    let f = args[0].eval(env)?;
    let mut acc = args[1].eval(env)?;
    let list = args[2].eval(env)?;
    if let Expression::List(list) = list {
        for item in list.into_iter().rev() {
            acc = Expression::Apply(Box::new(f.clone()), vec![item, acc]).eval(env)?;
        }
        Ok(acc)
    } else {
        Err(Error::CustomError(
            "foldr requires a list as its third argument".to_string(),
        ))
    }
}

fn zip(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "zip requires exactly two arguments".to_string(),
        ));
    }
    let list1 = args[0].eval(env)?;
    let list2 = args[1].eval(env)?;
    if let (Expression::List(list1), Expression::List(list2)) = (list1, list2) {
        let mut result = vec![];
        for (item1, item2) in list1.into_iter().zip(list2.into_iter()) {
            result.push(Expression::List(vec![item1, item2]));
        }
        Ok(Expression::List(result))
    } else {
        Err(Error::CustomError(
            "zip requires two lists as its arguments".to_string(),
        ))
    }
}

fn unzip(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 1 {
        return Err(Error::CustomError(
            "unzip requires exactly one argument".to_string(),
        ));
    }
    let list = args[0].eval(env)?;
    if let Expression::List(list) = list {
        let mut list1 = vec![];
        let mut list2 = vec![];
        for item in list {
            if let Expression::List(pair) = item {
                if pair.len() != 2 {
                    return Err(Error::CustomError(
                        "unzip requires a list of pairs as its argument".to_string(),
                    ));
                }
                list1.push(pair[0].clone());
                list2.push(pair[1].clone());
            } else {
                return Err(Error::CustomError(
                    "unzip requires a list of pairs as its argument".to_string(),
                ));
            }
        }
        Ok(Expression::List(vec![
            Expression::List(list1),
            Expression::List(list2),
        ]))
    } else {
        Err(Error::CustomError(
            "unzip requires a list of pairs as its argument".to_string(),
        ))
    }
}

fn take(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "take requires exactly two arguments".to_string(),
        ));
    }
    let n = args[0].eval(env)?;
    let list = args[1].eval(env)?;
    if let Expression::Integer(n) = n {
        if let Expression::List(list) = list {
            Ok(Expression::List(
                list.into_iter().take(n as usize).collect(),
            ))
        } else {
            Err(Error::CustomError(
                "take requires a list as its second argument".to_string(),
            ))
        }
    } else {
        Err(Error::CustomError(
            "take requires an integer as its first argument".to_string(),
        ))
    }
}

fn drop(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "drop requires exactly two arguments".to_string(),
        ));
    }
    let n = args[0].eval(env)?;
    let list = args[1].eval(env)?;
    if let Expression::Integer(n) = n {
        if let Expression::List(list) = list {
            Ok(Expression::List(
                list.into_iter().skip(n as usize).collect(),
            ))
        } else {
            Err(Error::CustomError(
                "drop requires a list as its second argument".to_string(),
            ))
        }
    } else {
        Err(Error::CustomError(
            "drop requires an integer as its first argument".to_string(),
        ))
    }
}

fn split_at(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "split-at requires exactly two arguments".to_string(),
        ));
    }
    let n = args[0].eval(env)?;
    let list = args[1].eval(env)?;
    if let Expression::Integer(n) = n {
        if let Expression::List(list) = list {
            let taken = list.iter().take(n as usize).cloned().collect();
            let dropped = list.into_iter().skip(n as usize).collect();
            Ok(Expression::List(vec![
                Expression::List(taken),
                Expression::List(dropped),
            ]))
        } else {
            Err(Error::CustomError(
                "split-at requires a list as its second argument".to_string(),
            ))
        }
    } else {
        Err(Error::CustomError(
            "split-at requires an integer as its first argument".to_string(),
        ))
    }
}

fn nth(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 2 {
        return Err(Error::CustomError(
            "nth requires exactly two arguments".to_string(),
        ));
    }
    let n = args[0].eval(env)?;
    let list = args[1].eval(env)?;
    if let Expression::Integer(n) = n {
        if let Expression::List(list) = list {
            if n < 0 {
                Ok(list[list.len() - (-n as usize)].clone())
            } else {
                Ok(list[n as usize].clone())
            }
        } else {
            Err(Error::CustomError(
                "nth requires a list as its second argument".to_string(),
            ))
        }
    } else {
        Err(Error::CustomError(
            "nth requires an integer as its first argument".to_string(),
        ))
    }
}
