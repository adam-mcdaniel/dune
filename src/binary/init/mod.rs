use dune::{Builtin, Environment, Error, Expression, Int};

use common_macros::b_tree_map;

#[cfg(feature = "chess-engine")]
mod chess_module;
mod console_module;
mod dict_module;
mod err_module;
mod fmt_module;
mod fn_module;
use fn_module::curry;
mod fs_module;
mod list_module;
mod log_module;
mod math_module;
mod operator_module;
mod os_module;
mod parse_module;
mod rand_module;
mod shell_module;
mod string_module;
mod sys_module;
mod time_module;
mod widget_module;

pub fn init(env: &mut Environment) {
    let standard_module = b_tree_map! {
        "log" => log_module::get(),
        "math" => math_module::get(),
        "dict" => dict_module::get(),
        "shell" => shell_module::get(),
        "err" => err_module::get(),
        "os" => os_module::get(),
        "widget" => widget_module::get(),
        "time" => time_module::get(),
        "rand" => rand_module::get(),
        "fn" => fn_module::get(),
        "console" => console_module::get(),
        "fmt" => fmt_module::get(),
        "parse" => parse_module::get(),
        "fs" => fs_module::get(env),
        "ops" => operator_module::get(env),
        "string" => string_module::get(),
        "list" => list_module::get(),
        "sys" => sys_module::get(),
    };

    env.define_module("std", standard_module.clone());

    for (name, module) in standard_module {
        env.define(name, module);
    }

    env.define("exit", env.get("os").unwrap()["exit"].clone());
    env.define("cd", env.get("os").unwrap()["cd"].clone());
    env.define("quit", env.get("exit").unwrap());

    env.define_builtin(
        "help",
        |args, env| {
            for arg in args {
                match &arg {
                    Expression::Symbol(name) if name == "me" => {
                        println!(include_str!("../help/me.txt"));
                    }
                    Expression::Symbol(name) if name == "prelude" => {
                        println!(include_str!("../help/prelude.txt"));
                    }
                    Expression::Symbol(name) if name == "types" => {
                        println!(include_str!("../help/types.txt"));
                    }
                    Expression::Symbol(name) if name == "scripting" => {
                        println!(include_str!("../help/scripting.txt"));
                    }
                    Expression::Symbol(name) if name == "builtin" => {
                        println!(include_str!("../help/builtin.txt"));
                    }
                    Expression::Symbol(name) if name == "lib" => {
                        println!(include_str!("../help/lib.txt"));
                    }
                    Expression::Symbol(name) if name == "syntax" => {
                        println!(include_str!("../help/syntax.txt"));
                    }
                    otherwise => {
                        if let Expression::Builtin(Builtin { help, .. }) = otherwise.eval(env)? {
                            println!("{}", help)
                        }
                    }
                }
            }
            Ok(Expression::None)
        },
        "run `help me`",
    );

    env.define_builtin(
        "print",
        |args, env| {
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    print!("{} ", x)
                } else {
                    print!("{}", x)
                }
            }

            Ok(Expression::None)
        },
        "print the arguments without a newline",
    );

    env.define_builtin(
        "debug",
        |args, env| {
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    print!("{:?} ", x)
                } else {
                    println!("{:?}", x)
                }
            }

            Ok(Expression::None)
        },
        "print the debug representation of the arguments and a newline",
    );

    env.define_builtin(
        "println",
        |args, env| {
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    print!("{} ", x)
                } else {
                    println!("{}", x)
                }
            }

            Ok(Expression::None)
        },
        "print the arguments and a newline",
    );
    env.define("echo", env.get("println").unwrap());

    env.define_builtin(
        "input",
        |args, env| {
            let mut prompt = String::new();
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?;
                if i < args.len() - 1 {
                    prompt += &format!("{} ", x)
                } else {
                    prompt += &format!("{}", x)
                }
            }
            let mut rl = crate::new_editor(env);
            Ok(Expression::String(crate::readline(&prompt, &mut rl)))
        },
        "get user input",
    );

    env.define_builtin(
        "to",
        |args, env| {
            if args.len() == 2 {
                match (args[0].clone().eval(env)?, args[1].clone().eval(env)?) {
                    (Expression::Integer(m), Expression::Integer(n)) => Ok(Expression::List(
                        (m..n).map(Expression::Integer).collect::<Vec<Expression>>(),
                    )),
                    _ => Err(Error::CustomError(
                        "Arguments to range must be integers".to_string(),
                    )),
                }
            } else {
                Err(Error::CustomError(
                    "Must supply 2 arguments to range".to_string(),
                ))
            }
        },
        "get a list of integers from (inclusive) one to another (exclusive)",
    );

    env.define_builtin(
        "str",
        |args, env| Ok(Expression::String(args[0].eval(env)?.to_string())),
        "format an expression to a string",
    );

    env.define_builtin(
        "int",
        |args, env| match args[0].eval(env)? {
            Expression::Integer(x) => Ok(Expression::Integer(x)),
            Expression::Float(x) => Ok(Expression::Integer(x as Int)),
            Expression::String(x) => {
                if let Ok(n) = x.parse::<Int>() {
                    Ok(Expression::Integer(n))
                } else {
                    Err(Error::CustomError(format!(
                        "could not convert {:?} to an integer",
                        x
                    )))
                }
            }
            otherwise => Err(Error::CustomError(format!(
                "could not convert {:?} to an integer",
                otherwise
            ))),
        },
        "convert a float or string to an int",
    );

    env.define_builtin(
        "insert",
        |args, env| {
            check_exact_args_len("insert", &args, 3)?;
            let mut arr = args[0].eval(env)?;
            let idx = args[1].eval(env)?;
            let val = args[2].eval(env)?;
            match (&mut arr, &idx) {
                (Expression::Map(exprs), Expression::String(key)) => {
                    exprs.insert(key.clone(), val);
                }
                (Expression::List(exprs), Expression::Integer(i)) => {
                    if *i as usize <= exprs.len() {
                        exprs.insert(*i as usize, val);
                    } else {
                        return Err(Error::CustomError(format!(
                            "index {} out of bounds for {:?}",
                            idx, arr
                        )));
                    }
                }
                (Expression::String(s), Expression::Integer(i)) => {
                    if *i as usize <= s.len() {
                        s.insert_str(*i as usize, &val.to_string());
                    } else {
                        return Err(Error::CustomError(format!(
                            "index {} out of bounds for {:?}",
                            idx, arr
                        )));
                    }
                }
                _ => {
                    return Err(Error::CustomError(format!(
                        "cannot insert {:?} into {:?} with index {:?}",
                        val, arr, idx
                    )))
                }
            }

            Ok(arr)
        },
        "insert an item into a dictionary or list",
    );

    env.define_builtin(
        "keys",
        |args, env| match args[0].eval(env)? {
            Expression::Map(m) => Ok(m.into_keys().collect::<Vec<_>>().into()),
            otherwise => Err(Error::CustomError(format!(
                "cannot get the keys of {}",
                otherwise
            ))),
        },
        "get the list of keys in a table",
    );

    env.define_builtin(
        "vals",
        |args, env| match args[0].eval(env)? {
            Expression::Map(m) => Ok(m.into_values().collect::<Vec<_>>().into()),
            otherwise => Err(Error::CustomError(format!(
                "cannot get the values of {}",
                otherwise
            ))),
        },
        "get the list of values in a table",
    );

    env.define_builtin(
        "vars",
        |_, env| Ok(env.bindings.clone().into()),
        "get a table of the defined variables",
    );

    env.define_builtin(
        "len",
        |args, env| match args[0].eval(env)? {
            Expression::Map(m) => Ok(Expression::Integer(m.len() as Int)),
            Expression::List(list) => Ok(Expression::Integer(list.len() as Int)),
            Expression::Symbol(x) | Expression::String(x) => {
                Ok(Expression::Integer(x.chars().count() as Int))
            }
            otherwise => Err(Error::CustomError(format!(
                "cannot get length of {}",
                otherwise
            ))),
        },
        "get the length of an expression",
    );

    env.define_builtin(
        "chars",
        |args, env| match args[0].eval(env)? {
            Expression::Symbol(x) | Expression::String(x) => Ok(Expression::List(
                x.chars()
                    .map(|ch| Expression::String(ch.to_string()))
                    .collect::<Vec<Expression>>(),
            )),
            otherwise => Err(Error::CustomError(format!(
                "cannot get characters of non-string {}",
                otherwise
            ))),
        },
        "get the list of characters for a string or symbol",
    );

    env.define_builtin(
        "head",
        |args, env| match args[0].eval(env)? {
            Expression::List(x) => Ok(if x.is_empty() {
                Expression::None
            } else {
                x[0].clone()
            }),
            otherwise => Err(Error::CustomError(format!(
                "cannot get the head of a non-list {}",
                otherwise
            ))),
        },
        "get the first item in a list",
    );

    env.define_builtin(
        "tail",
        |args, env| match args[0].eval(env)? {
            Expression::List(x) => Ok(if x.is_empty() {
                vec![]
            } else {
                x[1..].to_vec()
            }
            .into()),
            otherwise => Err(Error::CustomError(format!(
                "cannot get the tail of a non-list {}",
                otherwise
            ))),
        },
        "get the last items in a list",
    );

    env.define_builtin(
        "lines",
        |args, env| match args[0].eval(env)? {
            Expression::String(x) => Ok(Expression::List(
                x.lines()
                    .map(|ch| Expression::String(ch.to_string()))
                    .collect::<Vec<Expression>>(),
            )),
            otherwise => Err(Error::CustomError(format!(
                "cannot get lines of non-string {}",
                otherwise
            ))),
        },
        "get the list of lines in a string",
    );

    env.define_builtin(
        "eval",
        |args, env| {
            let mut new_env = env.clone();
            args[0].clone().eval(env)?.eval(&mut new_env)
        },
        "evaluate an expression without changing the environment",
    );

    env.define_builtin(
        "exec",
        |args, env| args[0].clone().eval(env)?.eval(env),
        "evaluate an expression in the current environment",
    );

    env.define_builtin(
        "unbind",
        |args, env| {
            check_exact_args_len("unbind", &args, 1)?;
            match &args[0] {
                Expression::Symbol(x) | Expression::String(x) => env.undefine(x),
                _ => {
                    return Err(Error::CustomError(format!(
                        "expected string or symbol, but got {:?}",
                        args[0]
                    )))
                }
            }
            Ok(Expression::None)
        },
        "unbind a variable from the environment",
    );

    #[cfg(feature = "chess-engine")]
    env.define_builtin("chess", chess_module::chess_fn, chess_module::HELP);

    env.define_builtin(
        "report",
        |args, env| {
            let val = args[0].eval(env)?;
            match val {
                Expression::Map(_) => println!("{}", val),
                Expression::String(s) => println!("{}", s),
                Expression::None => {}
                otherwise => println!("{}", otherwise),
            }

            Ok(Expression::None)
        },
        "default function for reporting values",
    );
}

fn check_args_len(
    name: impl ToString,
    args: &[Expression],
    expected_len: std::ops::RangeFrom<usize>,
) -> Result<(), Error> {
    if expected_len.contains(&args.len()) {
        Ok(())
    } else {
        Err(Error::CustomError(format!(
            "too few arguments to function {}",
            name.to_string()
        )))
    }
}

fn check_exact_args_len(
    name: impl ToString,
    args: &[Expression],
    expected_len: usize,
) -> Result<(), Error> {
    if args.len() == expected_len {
        Ok(())
    } else {
        Err(Error::CustomError(if args.len() > expected_len {
            format!("too many arguments to function {}", name.to_string())
        } else {
            format!("too few arguments to function {}", name.to_string())
        }))
    }
}
