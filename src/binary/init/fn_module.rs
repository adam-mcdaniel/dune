use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub(super) fn curry_env(
    f: Expression,
    args: usize,
    env: &mut Environment,
) -> Result<Expression, Error> {
    let mut result = Expression::Apply(
        Box::new(f.clone()),
        (0..args)
            .map(|i| Expression::Symbol(format!("arg{}", i)))
            .collect(),
    );
    for i in (0..args).rev() {
        result = Expression::Lambda(
            format!("arg{}", i),
            Box::new(result),
            Environment::default(),
        )
        .eval(env)?;
    }
    Ok(result)
}

pub(super) fn reverse_curry_env(
    f: Expression,
    args: usize,
    env: &mut Environment,
) -> Result<Expression, Error> {
    let mut result = Expression::Apply(
        Box::new(f.clone()),
        (0..args)
            .rev()
            .map(|i| Expression::Symbol(format!("arg{}", i)))
            .collect(),
    );
    for i in (0..args).rev() {
        result = Expression::Lambda(
            format!("arg{}", i),
            Box::new(result),
            Environment::default(),
        )
        .eval(env)?;
    }
    Ok(result)
}

pub fn reverse_curry(f: Expression, args: usize) -> Expression {
    let mut env = Environment::default();
    reverse_curry_env(f, args, &mut env).unwrap()
}

pub(super) fn curry(f: Expression, args: usize) -> Expression {
    let mut env = Environment::default();
    curry_env(f, args, &mut env).unwrap()
}

fn curry_builtin(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() < 2 {
        return Err(Error::CustomError(
            "curry requires at least two arguments".to_string(),
        ));
    }
    let f = args[0].eval(env)?;
    if let Expression::Integer(arg_count) = args[1].eval(env)? {
        curry_env(f, arg_count as usize, env)
    } else {
        Ok(f)
    }
}

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("id") => crate::parse("x -> x").expect("failed to parse id").eval(&mut Environment::default()).expect("failed to eval id"),
        String::from("const") => crate::parse("x -> y -> x").expect("failed to parse const").eval(&mut Environment::default()).expect("failed to eval const"),
        String::from("flip") => crate::parse("f -> x -> y -> f y x").expect("failed to parse flip").eval(&mut Environment::default()).expect("failed to eval flip"),
        String::from("compose") => crate::parse("f -> g -> x -> f (g x)").expect("failed to parse compose").eval(&mut Environment::default()).expect("failed to eval compose"),

        String::from("apply") => Expression::builtin("apply", |args, env| {
            if args.len() != 2 {
                return Err(Error::CustomError(
                    "apply requires exactly two arguments".to_string(),
                ));
            }
            let f = args[0].eval(env)?;
            let args = args[1].eval(env)?;
            if let Expression::List(args) = args {
                Expression::Apply(Box::new(f), args).eval(env)
            } else {
                Err(Error::CustomError(format!(
                    "invalid arguments to apply: {}",
                    args
                )))
            }
        }, "apply a function to a list of arguments"),
        String::from("curry") => Expression::builtin("curry", curry_builtin,
            "curry a function that takes multiple arguments"),
        String::from("map") => Expression::builtin("map", map,
            "map a function over a list of values"),
        String::from("filter") => Expression::builtin("filter", filter,
            "filter a list of values with a condition function"),
        String::from("reduce") => Expression::builtin("reduce", reduce,
            "reduce a function over a list of values"),
        String::from("?") => Expression::builtin("?", conditional,
        "conditionally evaluate two expressions based on the truthiness of a condition"),
    })
    .into()
}

fn conditional(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() != 3 {
        return Err(Error::CustomError(
            "conditional requires exactly three arguments".to_string(),
        ));
    }
    let condition = args[0].eval(env)?;
    if condition.is_truthy() {
        args[1].eval(env)
    } else {
        args[2].eval(env)
    }
}

fn map(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if !(1..=2).contains(&args.len()) {
        return Err(Error::CustomError(
            if args.len() > 2 {
                "too many arguments to function map"
            } else {
                "too few arguments to function map"
            }
            .to_string(),
        ));
    }

    if args.len() == 1 {
        Expression::Apply(
            Box::new(crate::parse("f -> list -> for item in list {f item}")?),
            args.clone(),
        )
        .eval(env)
    } else if let Expression::List(list) = args[1].eval(env)? {
        let f = args[0].eval(env)?;
        let mut result = vec![];
        for item in list {
            result.push(Expression::Apply(Box::new(f.clone()), vec![item]).eval(env)?)
        }
        Ok(result.into())
    } else {
        Err(Error::CustomError(format!(
            "invalid arguments to map: {}",
            Expression::from(args)
        )))
    }
}

fn filter(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if !(1..=2).contains(&args.len()) {
        return Err(Error::CustomError(
            if args.len() > 2 {
                "too many arguments to function filter"
            } else {
                "too few arguments to function filter"
            }
            .to_string(),
        ));
    }

    if args.len() == 1 {
        Expression::Apply(
            Box::new(crate::parse(
                r#"f -> list -> {
                    let result = [];
                    for item in list {
                        if (f item) {
                            let result = result + [item];
                        }
                    }
                    result
                }"#,
            )?),
            args.clone(),
        )
        .eval(env)
    } else if let Expression::List(list) = args[1].eval(env)? {
        let f = args[0].eval(env)?;
        let mut result = vec![];
        for item in list {
            if Expression::Apply(Box::new(f.clone()), vec![item.clone()])
                .eval(env)?
                .is_truthy()
            {
                result.push(item)
            }
        }
        Ok(result.into())
    } else {
        Err(Error::CustomError(format!(
            "invalid arguments to filter: {}",
            Expression::from(args)
        )))
    }
}

fn reduce(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if !(1..=3).contains(&args.len()) {
        return Err(Error::CustomError(
            if args.len() > 3 {
                "too many arguments to function reduce"
            } else {
                "too few arguments to function reduce"
            }
            .to_string(),
        ));
    }

    if args.len() < 3 {
        Expression::Apply(
            Box::new(crate::parse(
                "f -> acc -> list -> { \
                        for item in list { let acc = f acc item } acc }",
            )?),
            args.clone(),
        )
        .eval(env)
    } else if let Expression::List(list) = args[2].eval(env)? {
        let f = args[0].eval(env)?;
        let mut acc = args[1].eval(env)?;
        for item in list {
            acc = Expression::Apply(Box::new(f.clone()), vec![acc, item]).eval(env)?
        }
        Ok(acc)
    } else {
        Err(Error::CustomError(format!(
            "invalid arguments to reduce: {}",
            Expression::from(args)
        )))
    }
}
