use common_macros::b_tree_map;
use dune::{Error, Expression, Int};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("E")   => std::f64::consts::E.into(),
        String::from("PI")  => std::f64::consts::PI.into(),
        String::from("TAU") => std::f64::consts::TAU.into(),

        String::from("isodd") => Expression::builtin("isodd", |args, env| {
            super::check_exact_args_len("odd", &args, 1)?;
            Ok(match args[0].eval(env)? {
                Expression::Integer(i) => i % 2 == 1,
                Expression::Float(f) => (f as Int) % 2 == 1,
                e => return Err(Error::CustomError(format!("invalid isodd argument {}", e)))
            }.into())
        }, "is a number odd?"),

        String::from("iseven") => Expression::builtin("iseven", |args, env| {
            super::check_exact_args_len("even", &args, 1)?;
            Ok(match args[0].eval(env)? {
                Expression::Integer(i) => i % 2 == 0,
                Expression::Float(f) => (f as Int) % 2 == 0,
                e => return Err(Error::CustomError(format!("invalid iseven argument {}", e)))
            }.into())
        }, "is a number even?"),

        String::from("pow") => Expression::builtin("pow", |args, env| {
            super::check_exact_args_len("pow", &args, 2)?;
            match (args[0].eval(env)?, args[1].eval(env)?) {
                (Expression::Float(base), Expression::Float(exponent)) => Ok(base.powf(exponent).into()),
                (Expression::Float(base), Expression::Integer(exponent)) => Ok(base.powf(exponent as f64).into()),
                (Expression::Integer(base), Expression::Float(exponent)) => Ok((base as f64).powf(exponent).into()),
                (Expression::Integer(base), Expression::Integer(exponent)) => match base.checked_pow(exponent as u32) {
                    Some(n) => Ok(n.into()),
                    None => Err(Error::CustomError(format!("overflow when raising int {} to the power {}", base, exponent)))
                },
                (a, b) => Err(Error::CustomError(format!("cannot raise {} to the power {}", a, b)))
            }
        }, "raise a number to a power"),


        String::from("ln") => Expression::builtin("ln", |args, env| {
            super::check_exact_args_len("ln", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid natural log argument {}", e)))
            };

            Ok(x.ln().into())
        }, "get the natural log of a number"),


        String::from("log") => Expression::builtin("log", |args, env| {
            super::check_exact_args_len("log", &args, 2)?;

            let base = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid log base {}", e)))
            };

            let x = match args[1].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid log argument {}", e)))
            };

            Ok(x.log(base).into())
        }, "get the log of a number using a given base"),


        String::from("log2") => Expression::builtin("log2", |args, env| {
            super::check_exact_args_len("log2", &args, 1)?;

            let base = 2.0;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid log2 argument {}", e)))
            };

            Ok(x.log(base).into())
        }, "get the log base 2 of a number"),

        String::from("log10") => Expression::builtin("log10", |args, env| {
            super::check_exact_args_len("log10", &args, 1)?;

            let base = 10.0;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid log10 argument {}", e)))
            };

            Ok(x.log(base).into())
        }, "get the log base 10 of a number"),

        String::from("sqrt") => Expression::builtin("sqrt", |args, env| {
            super::check_exact_args_len("sqrt", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid sqrt argument {}", e)))
            };

            Ok(x.sqrt().into())
        }, "get the square root of a number"),

        String::from("cbrt") => Expression::builtin("cbrt", |args, env| {
            super::check_exact_args_len("cbrt", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid cbrt argument {}", e)))
            };

            Ok(x.cbrt().into())
        }, "get the cube root of a number"),


        String::from("sin") => Expression::builtin("sin", |args, env| {
            super::check_exact_args_len("sin", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid sin argument {}", e)))
            };

            Ok(x.sin().into())
        }, "get the sin of a number"),

        String::from("cos") => Expression::builtin("cos", |args, env| {
            super::check_exact_args_len("cos", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid cos argument {}", e)))
            };

            Ok(x.cos().into())
        }, "get the cosine of a number"),

        String::from("tan") => Expression::builtin("tan", |args, env| {
            super::check_exact_args_len("tan", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid tan argument {}", e)))
            };

            Ok(x.tan().into())
        }, "get the tangent of a number"),



        String::from("asin") => Expression::builtin("asin", |args, env| {
            super::check_exact_args_len("asin", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid asin argument {}", e)))
            };

            Ok(x.asin().into())
        }, "get the inverse sin of a number"),

        String::from("acos") => Expression::builtin("acos", |args, env| {
            super::check_exact_args_len("acos", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid acos argument {}", e)))
            };

            Ok(x.acos().into())
        }, "get the inverse cosine of a number"),

        String::from("atan") => Expression::builtin("atan", |args, env| {
            super::check_exact_args_len("atan", &args, 1)?;

            let x = match args[0].eval(env)? {
                Expression::Float(f) => f,
                Expression::Integer(i) => i as f64,
                e => return Err(Error::CustomError(format!("invalid atan argument {}", e)))
            };

            Ok(x.atan().into())
        }, "get the inverse tangent of a number"),
    })
    .into()
}
