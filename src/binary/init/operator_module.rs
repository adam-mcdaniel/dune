use dune::{Environment, Error, Expression};

pub fn add_to(env: &mut Environment) {
    env.define_builtin(
        "add",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? + arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot add {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "add two numbers",
    );

    env.define_builtin(
        "sub",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? - arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot subtract {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "subtract two numbers",
    );

    env.define_builtin(
        "neg",
        |args, env| match args[0].clone().eval(env)? {
            Expression::Integer(n) => Ok(Expression::Integer(-n)),
            Expression::Float(n) => Ok(Expression::Float(-n)),
            x => Err(Error::CustomError(format!("cannot negate {:?}", x))),
        },
        "negate a number",
    );

    env.define_builtin(
        "mul",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? * arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot multiply {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "multiply two numbers",
    );

    env.define_builtin(
        "div",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? / arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot divide {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "divide two numbers",
    );

    env.define_builtin(
        "rem",
        |args, env| {
            let mut result = args[0].clone().eval(env)?;
            for arg in &args[1..] {
                let old_result = result.clone();
                result = result.eval(env)? % arg.clone().eval(env)?;

                if let Expression::None = result {
                    return Err(Error::CustomError(format!(
                        "cannot remainder {:?} and {:?}",
                        old_result, arg
                    )));
                }
            }
            Ok(result)
        },
        "remainder two numbers",
    );

    env.define_builtin(
        "and",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .all(|item| item.is_truthy()),
            ))
        },
        "perform a boolean and for a list of truthy values",
    );

    env.define_builtin(
        "or",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .any(|item| item.is_truthy()),
            ))
        },
        "perform a boolean or for a list of truthy values",
    );

    env.define_builtin(
        "not",
        |args, env| {
            Ok(Expression::Boolean(
                args.into_iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<Expression>, Error>>()?
                    .iter()
                    .all(|item| !item.is_truthy()),
            ))
        },
        "perform a boolean not for one or many truthy values",
    );

    env.define_builtin(
        "eq",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].eval(env)? == args[1].eval(env)?,
            ))
        },
        "compare two values for equality",
    );

    env.define_builtin(
        "neq",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].eval(env)? != args[1].eval(env)?,
            ))
        },
        "compare two values for inequality",
    );

    env.define_builtin(
        "lt",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? < args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "lte",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? <= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "gt",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? > args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "gte",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? >= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    env.define_builtin(
        "index",
        |args, env| {
            let mut val = args[0].eval(env)?;
            for arg in &args[1..] {
                val = match arg {
                    Expression::Integer(_) | Expression::Symbol(_) => &val[arg.clone()],
                    otherwise => &val[otherwise.eval(env)?],
                }
                .clone()
            }
            Ok(val)
        },
        "index a dictionary or list",
    );
}
