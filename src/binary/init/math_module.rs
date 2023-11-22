use super::curry;
use common_macros::b_tree_map;
use dune::{Error, Environment, Expression, Int};

pub fn get(env: &mut Environment) -> Expression {
    (b_tree_map! {
        String::from("E")   => std::f64::consts::E.into(),
        String::from("PI")  => std::f64::consts::PI.into(),
        String::from("TAU") => std::f64::consts::TAU.into(),

        String::from("max") => crate::parse("x -> y -> if (x > y) { x } else { y }").unwrap().eval(env).unwrap(),
        String::from("min") => crate::parse("x -> y -> if (x < y) { x } else { y }").unwrap().eval(env).unwrap(),

        String::from("l-rsh") => curry(Expression::builtin("l-rsh", |args, env| {
            super::check_exact_args_len("l-rsh", &args, 2)?;

            let a = match args[0].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid l-rsh argument {}", e)))
            };

            let b = match args[1].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid l-rsh argument {}", e)))
            };

            let a = a as u64;
            let b = b as u64;

            Ok(((a >> b) as Int).into())
        }, "logical right shift"), 2),

        String::from("a-rsh") => curry(Expression::builtin("a-rsh", |args, env| {
            super::check_exact_args_len("a-rsh", &args, 2)?;

            let a = match args[0].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid a-rsh argument {}", e)))
            };
            let b = match args[1].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid a-rsh argument {}", e)))
            };

            Ok((a >> b).into())
        }, "arithmetic right shift"), 2),

        String::from("rsh") => curry(Expression::builtin("a-rsh", |args, env| {
            super::check_exact_args_len("a-rsh", &args, 2)?;

            let a = match args[0].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid a-rsh argument {}", e)))
            };
            let b = match args[1].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid a-rsh argument {}", e)))
            };

            Ok((a >> b).into())
        }, "arithmetic right shift"), 2),

        String::from("lsh") => Expression::builtin("lsh", |args, env| {
            super::check_exact_args_len("lsh", &args, 2)?;

            let a = match args[0].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid lsh argument {}", e)))
            };
            let b = match args[1].eval(env)? {
                Expression::Integer(i) => i,
                e => return Err(Error::CustomError(format!("invalid lsh argument {}", e)))
            };

            Ok((a << b).into())
        }, "left shift"),


        String::from("sum") => Expression::builtin("sum", |args, env| {
            let mut int_sum = 0;
            let mut float_sum = 0.0;
            for arg in &args {
                match arg.eval(env)? {
                    Expression::Integer(i) => int_sum += i,
                    Expression::Float(f) => float_sum += f,
                    Expression::List(list) => {
                        for item in list {
                            match item.eval(env)? {
                                Expression::Integer(i) => int_sum += i,
                                Expression::Float(f) => float_sum += f,
                                e => return Err(Error::CustomError(format!("invalid sum argument {:?}", e)))
                            }
                        }
                    },
                    e => return Err(Error::CustomError(format!("invalid sum argument {:?}", e)))
                }
            }

            if float_sum == 0.0 {
                Ok(int_sum.into())
            } else {
                Ok((int_sum as f64 + float_sum).into())
            }
        }, "sum a list of numbers"),

        String::from("product") => Expression::builtin("product", |args, env| {
            let mut int_product = 1;
            let mut float_product = 1.0;

            for arg in &args {
                match arg.eval(env)? {
                    Expression::Integer(i) => int_product *= i,
                    Expression::Float(f) => float_product *= f,
                    Expression::List(list) => {
                        for item in list {
                            match item.eval(env)? {
                                Expression::Integer(i) => int_product *= i,
                                Expression::Float(f) => float_product *= f,
                                e => return Err(Error::CustomError(format!("invalid product argument {:?}", e)))
                            }
                        }
                    },
                    e => return Err(Error::CustomError(format!("invalid product argument {:?}", e)))
                }
            }

            if float_product == 1.0 {
                Ok(int_product.into())
            } else {
                Ok((int_product as f64 * float_product).into())
            }
        }, "multiply a list of numbers"),

        String::from("fact") => Expression::builtin("fact", |args, env| {
            super::check_exact_args_len("fact", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => {
                    if i < 0 {
                        Err(Error::CustomError("cannot take the factorial of a negative number".to_string()))
                    } else {
                        let mut result = 1;
                        for n in 1..=i {
                            result *= n;
                        }
                        Ok(result.into())
                    }
                },
                Expression::Float(f) => {
                    if f < 0.0 {
                        Err(Error::CustomError("cannot take the factorial of a negative number".to_string()))
                    } else {
                        Ok(f64::round(f64::exp(f64::ln(f) * f64::ln(f)) * f64::sqrt(2.0 * std::f64::consts::PI * f) * (1.0 + 1.0 / (12.0 * f) + 1.0 / (288.0 * f * f) - 139.0 / (51840.0 * f * f * f) - 571.0 / (2488320.0 * f * f * f * f))).into())
                    }
                },
                e => Err(Error::CustomError(format!("invalid fact argument {:?}", e)))
            }
        }, "get the factorial of a number"),

        String::from("abs") => Expression::builtin("abs", |args, env| {
            super::check_exact_args_len("abs", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok(i.abs().into()),
                Expression::Float(f) => Ok(f.abs().into()),
                e => Err(Error::CustomError(format!("invalid abs argument {:?}", e)))
            }
        }, "get the absolute value of a number"),

        String::from("floor") => Expression::builtin("floor", |args, env| {
            super::check_exact_args_len("floor", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok(i.into()),
                Expression::Float(f) => Ok(f.floor().into()),
                e => Err(Error::CustomError(format!("invalid floor argument {:?}", e)))
            }
        }, "get the floor of a number"),

        String::from("ceil") => Expression::builtin("ceil", |args, env| {
            super::check_exact_args_len("ceil", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok(i.into()),
                Expression::Float(f) => Ok(f.ceil().into()),
                e => Err(Error::CustomError(format!("invalid ceil argument {:?}", e)))
            }
        }, "get the ceiling of a number"),

        String::from("round") => Expression::builtin("round", |args, env| {
            super::check_exact_args_len("round", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok(i.into()),
                Expression::Float(f) => Ok(f.round().into()),
                e => Err(Error::CustomError(format!("invalid round argument {:?}", e)))
            }
        }, "round a number to the nearest integer"),

        String::from("trunc") => Expression::builtin("trunc", |args, env| {
            super::check_exact_args_len("trunc", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok(i.into()),
                Expression::Float(f) => Ok(f.trunc().into()),
                e => Err(Error::CustomError(format!("invalid trunc argument {:?}", e)))
            }
        }, "truncate a number"),

        String::from("sinh") => Expression::builtin("sinh", |args, env| {
            super::check_exact_args_len("sinh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).sinh().into()),
                Expression::Float(f) => Ok(f.sinh().into()),
                e => Err(Error::CustomError(format!("invalid sinh argument {:?}", e)))
            }
        }, "get the hyperbolic sine of a number"),

        String::from("cosh") => Expression::builtin("cosh", |args, env| {
            super::check_exact_args_len("cosh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).cosh().into()),
                Expression::Float(f) => Ok(f.cosh().into()),
                e => Err(Error::CustomError(format!("invalid cosh argument {:?}", e)))
            }
        }, "get the hyperbolic cosine of a number"),

        String::from("tanh") => Expression::builtin("tanh", |args, env| {
            super::check_exact_args_len("tanh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).tanh().into()),
                Expression::Float(f) => Ok(f.tanh().into()),
                e => Err(Error::CustomError(format!("invalid tanh argument {:?}", e)))
            }
        }, "get the hyperbolic tangent of a number"),

        String::from("asinh") => Expression::builtin("asinh", |args, env| {
            super::check_exact_args_len("asinh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).asinh().into()),
                Expression::Float(f) => Ok(f.asinh().into()),
                e => Err(Error::CustomError(format!("invalid asinh argument {:?}", e)))
            }
        }, "get the inverse hyperbolic sine of a number"),

        String::from("acosh") => Expression::builtin("acosh", |args, env| {
            super::check_exact_args_len("acosh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).acosh().into()),
                Expression::Float(f) => Ok(f.acosh().into()),
                e => Err(Error::CustomError(format!("invalid acosh argument {:?}", e)))
            }
        }, "get the inverse hyperbolic cosine of a number"),

        String::from("atanh") => Expression::builtin("atanh", |args, env| {
            super::check_exact_args_len("atanh", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64).atanh().into()),
                Expression::Float(f) => Ok(f.atanh().into()),
                e => Err(Error::CustomError(format!("invalid atanh argument {:?}", e)))
            }
        }, "get the inverse hyperbolic tangent of a number"),

        String::from("sinpi") => Expression::builtin("sinpi", |args, env| {
            super::check_exact_args_len("sinpi", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64 * std::f64::consts::PI).sin().into()),
                Expression::Float(f) => Ok((f * std::f64::consts::PI).sin().into()),
                e => Err(Error::CustomError(format!("invalid sinpi argument {:?}", e)))
            }
        }, "get the sine of a number times pi"),

        String::from("cospi") => Expression::builtin("cospi", |args, env| {
            super::check_exact_args_len("cospi", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64 * std::f64::consts::PI).cos().into()),
                Expression::Float(f) => Ok((f * std::f64::consts::PI).cos().into()),
                e => Err(Error::CustomError(format!("invalid cospi argument {:?}", e)))
            }
        }, "get the cosine of a number times pi"),

        String::from("tanpi") => Expression::builtin("tanpi", |args, env| {
            super::check_exact_args_len("tanpi", &args, 1)?;
            match args[0].eval(env)? {
                Expression::Integer(i) => Ok((i as f64 * std::f64::consts::PI).tan().into()),
                Expression::Float(f) => Ok((f * std::f64::consts::PI).tan().into()),
                e => Err(Error::CustomError(format!("invalid tanpi argument {:?}", e)))
            }
        }, "get the tangent of a number times pi"),

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


        String::from("log") => curry(Expression::builtin("log", |args, env| {
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
        }, "get the log of a number using a given base"), 2),


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
