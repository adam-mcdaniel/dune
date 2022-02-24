use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("map") => Expression::builtin("map", map,
            "map a function over a list of values"),
        String::from("filter") => Expression::builtin("filter", filter,
            "filter a list of values with a condition function"),
        String::from("reduce") => Expression::builtin("reduce", reduce,
            "reduce a function over a list of values")
    })
    .into()
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
