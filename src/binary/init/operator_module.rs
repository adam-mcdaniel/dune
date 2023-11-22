use dune::{Environment, Error, Expression};
use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn get(env: &mut Environment) -> Expression {
    let mut tmp = Environment::new();

    // This is a special form that takes a list of expressions
    // and interprets them as a commands.
    // It pipes the result of each command to the next one.
    tmp.define_builtin("|", pipe_builtin, "pipe input through a list of commands");

    tmp.define_builtin(
        "+",
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
        "add two expressions",
    );

    tmp.define_builtin(
        "-",
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
        "subtract two expressions",
    );

    tmp.define_builtin(
        "*",
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
        "multiply two expressions",
    );

    tmp.define_builtin(
        "//",
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
        "divide two expressions",
    );

    tmp.define_builtin(
        "%",
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
        "get the remainder of two expressions",
    );

    tmp.define_builtin(
        "&&",
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

    tmp.define_builtin(
        "||",
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

    tmp.define_builtin(
        "!",
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

    tmp.define_builtin(
        "==",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].eval(env)? == args[1].eval(env)?,
            ))
        },
        "compare two values for equality",
    );

    tmp.define_builtin(
        "!=",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].eval(env)? != args[1].eval(env)?,
            ))
        },
        "compare two values for inequality",
    );

    tmp.define_builtin(
        "<",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? < args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    tmp.define_builtin(
        "<=",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? <= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    tmp.define_builtin(
        ">",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? > args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    tmp.define_builtin(
        ">=",
        |args, env| {
            Ok(Expression::Boolean(
                args[0].clone().eval(env)? >= args[1].clone().eval(env)?,
            ))
        },
        "determine the order of two values",
    );

    tmp.define_builtin(
        "@",
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

    let mut new_tmp = env.clone();
    for (name, val) in &tmp.bindings {
        new_tmp.define(name, val.clone());
    }

    tmp.define("<<", crate::parse("fs@read").unwrap().eval(&mut new_tmp).unwrap());
    tmp.define(">>", crate::parse("file -> contents -> fs@write file contents").unwrap().eval(&mut new_tmp).unwrap());
    tmp.define(">>>", crate::parse("file -> contents -> fs@append file contents").unwrap().eval(&mut new_tmp).unwrap());
    
    let bindings = tmp.bindings.clone();

    for (name, val) in &bindings {
        env.define(name, val.clone());
    }

    Expression::Map(tmp.bindings)
}

fn pipe_builtin(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    if args.len() <= 1 {
        return Err(Error::CustomError(
            "pipe requires at least two arguments".to_string(),
        ));
    }
    // The accumulator for the result of the pipe.
    // This is mainly used if the user pipes into a command
    // that is a function or macro like so:
    //
    // $ "Hello" | (x -> x + " world!") | echo
    let mut result_of_last_cmd = Expression::None;
    // The buffer of the STDOUT of the last command.
    let mut buf = vec![];

    // A dummy value to hold where `expr_to_command` stores its resulting command.
    let mut x = Command::new("dummy");

    // For every command, pipe in the previous output buffer,
    // and get the result.
    for (i, expr) in args.iter().enumerate() {
        // Is this the first command to be executed?
        let is_first = i == 0;
        // Is this the last command to be executed?
        let is_last = i == args.len() - 1;

        // If the expression can be interpreted as a command (a call to a program),
        // then execute it and get its standard output (using the last expression's
        // result as the standard input).
        match expr_to_command(&mut x, expr, env)? {
            // If the expression is a command:
            Some(mut cmd) => {
                if is_first {
                    // If this is the first command, we inherit the current STDIN.
                    cmd = cmd.stdin(Stdio::inherit());
                } else {
                    // Otherwise, we use the piped STDOUT of the previous command.
                    cmd = cmd.stdin(Stdio::piped());
                }

                if is_last {
                    // If this is the last command, we inherit the current STDOUT.
                    cmd = cmd.stdout(Stdio::inherit());
                } else {
                    // Otherwise, we collect the stdout and pipe it to the next command.
                    cmd = cmd.stdout(Stdio::piped());
                }

                // Try to execute the command.
                if let Ok(mut child_handler) = cmd.spawn() {
                    // If we need to pipe in STDIN:
                    if !is_first {
                        // Attempt to grab the STDIN of the process from the handler.
                        if let Some(stdin) = child_handler.stdin.as_mut() {
                            // Write the contents of the previous command's STDOUT
                            // to the process's STDIN.
                            if stdin.write_all(&buf).is_err() {
                                return Err(Error::CustomError(format!(
                                    "error when piping into process `{}`",
                                    expr
                                )));
                            }

                            // Zero out all of our information about the previous command.
                            buf = vec![];
                            result_of_last_cmd = Expression::None;

                            // Flush the STDIN buffer to force the process to read it.
                            stdin.flush().unwrap();
                        } else {
                            return Err(Error::CustomError(format!(
                                "error when piping into process `{}`",
                                expr
                            )));
                        }
                    }

                    if is_last {
                        // If this is the last command in the pipe, then simply
                        // wait for it to finish without piping in any input.
                        if child_handler.wait().is_err() {
                            return Err(Error::CustomError(format!(
                                "error when waiting for process `{}`",
                                expr
                            )));
                        }
                    } else {
                        // If it is not the last command, then we need
                        // collect its standard output for the next process
                        // in the pipe.

                        // Attempt to grab the STDOUT of the process from the handler.
                        if let Ok(output) = child_handler.wait_with_output() {
                            // Store the contents of the STDOUT into the buffer
                            // for the next process.
                            buf = output.stdout.clone();

                            // Store the result of the command into the accumulator.
                            result_of_last_cmd = if let Ok(s) = String::from_utf8(output.stdout) {
                                // If the command returned valid UTF-8,
                                // then store it as a string.
                                Expression::String(s)
                            } else {
                                // Otherwise, store it as a list of bytes to
                                // prevent the loss of any data.
                                Expression::Bytes(buf.clone())
                            };
                        } else {
                            return Err(Error::CustomError(format!(
                                "error when waiting for process `{}`",
                                expr
                            )));
                        }
                    }
                } else {
                    // We couldn't spawn the command.
                    return Err(Error::CustomError(format!(
                        "could not spawn process `{}`",
                        expr
                    )));
                }
            }
            // If the expression is not a command, then
            // treat this as an application of that expression to the result
            // of the last command.
            None => {
                result_of_last_cmd = if is_first {
                    // If this is the first command, don't pipe in anything.
                    expr.clone()
                } else {
                    // If this is any other command, pipe in the result of the last command (via application).
                    Expression::Apply(Box::new(expr.clone()), vec![result_of_last_cmd])
                }
                .eval(env)?;

                if let Expression::Bytes(ref bytes) = result_of_last_cmd {
                    // If the result of the last command was some bytes,
                    // then store the bytes directly into the stdin buffer for the next command.
                    buf = bytes.clone();
                } else {
                    // Otherwise, just convert the result of this command into a string,
                    // and store that into the stdin buffer for the next command.
                    buf = result_of_last_cmd.to_string().into_bytes();
                }
            }
        }
    }
    // Return the accumulated Dune expression.
    Ok(result_of_last_cmd)
}

/// Interpret a Dune expression as a program to be executed.
/// If the expression can be executed as a program, then it will
/// be stored in the `cmd` parameter, and the function will return
/// `Ok(Some(cmd))`.
///
/// Otherwise, the program will return `Ok(None)`.
fn expr_to_command<'a>(
    cmd: &'a mut Command,
    expr: &Expression,
    env: &mut Environment,
) -> Result<Option<&'a mut Command>, Error> {
    let bindings = env
        .bindings
        .clone()
        .into_iter()
        .map(|(k, v)| (k, v.to_string()))
        // This is to prevent environment variables from getting too large.
        // This causes some strange bugs on Linux: mainly it becomes
        // impossible to execute any program because `the argument
        // list is too long`.
        .filter(|(_, s)| s.len() <= 1024);

    Ok(match expr {
        // If the command is quoted or in parentheses, try to get the inner command.
        Expression::Group(expr) | Expression::Quote(expr) => expr_to_command(cmd, expr, env)?,
        // If the command is an undefined symbol with some arguments.
        Expression::Apply(f, args) => match **f {
            Expression::Symbol(ref name) | Expression::String(ref name) => {
                let cmd_name = match env.get(name) {
                    // If the symbol is an alias, then execute the alias.
                    Some(Expression::Symbol(alias)) => alias,
                    // If the symbol is bound to something like `5`, this isn't a command.
                    Some(_) => return Ok(None),
                    // If the symbol is not bound, then it is a command.
                    None => name.clone(),
                };
                *cmd = Command::new(cmd_name);
                Some(
                    cmd.current_dir(env.get_cwd()).envs(bindings).args(
                        args.iter()
                            .filter(|&x| x != &Expression::None)
                            .map(|x| Ok(format!("{}", x.eval(env)?)))
                            .collect::<Result<Vec<String>, Error>>()?,
                    ),
                )
            }
            Expression::Quote(ref program) => {
                match *program.clone() {
                    Expression::String(cmd_name) | Expression::Symbol(cmd_name) => {
                        *cmd = Command::new(cmd_name);
                        Some(
                            cmd.current_dir(env.get_cwd()).envs(bindings).args(
                                args.iter()
                                    .filter(|&x| x != &Expression::None)
                                    .map(|x| Ok(format!("{}", x.eval(env)?)))
                                    .collect::<Result<Vec<String>, Error>>()?,
                            ),
                        )
                    }
                    _ => None
                }
            }
            _ => None,
        },

        // If the command is an undefined symbol, or an alias.
        Expression::Symbol(name) => match env.get(name) {
            // If the symbol is an alias, then execute the alias.
            Some(Expression::Symbol(name)) => {
                *cmd = Command::new(name);
                Some(cmd.current_dir(env.get_cwd()).envs(bindings))
            }
            // If the symbol is bound to something like `5`, this isn't a command.
            Some(_) => None,
            // If the symbol is not defined, use the symbol as the program name.
            None => {
                *cmd = Command::new(name);
                Some(cmd.current_dir(env.get_cwd()).envs(bindings))
            }
        },

        // Other types of expressions cannot be commands.
        _ => None,
    })
}
