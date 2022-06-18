use std::{
    io::Write,
    process::{Command, Stdio},
};

use dune::{Environment, Error, Expression};

pub fn add_to(env: &mut Environment) {
    // This is a special form that takes a list of expressions
    // and interprets them as a commands.
    // It pipes the result of each command to the next one.
    env.define_builtin("|", pipe_builtin, "pipe input through a list of commands");
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
        match expr_to_command(expr, env)? {
            // If the expression is a command:
            Some(mut cmd) => {
                if is_first {
                    // If this is the first command, we inherit the current STDIN.
                    cmd.stdin(Stdio::inherit());
                } else {
                    // Otherwise, we use the piped STDOUT of the previous command.
                    cmd.stdin(Stdio::piped());
                }

                if is_last {
                    // If this is the last command, we inherit the current STDOUT.
                    cmd.stdout(Stdio::inherit());
                } else {
                    // Otherwise, we collect the stdout and pipe it to the next command.
                    cmd.stdout(Stdio::piped());
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
fn expr_to_command(expr: &Expression, env: &mut Environment) -> Result<Option<Command>, Error> {
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
        Expression::Group(expr) | Expression::Quote(expr) => expr_to_command(expr, env)?,
        // If the command is an undefined symbol with some arguments.
        Expression::Apply(f, args) => match &**f {
            Expression::Symbol(name) => {
                let cmd_name: &str = match env.get_ref(name) {
                    // If the symbol is an alias, then execute the alias.
                    Some(Expression::Symbol(alias)) => alias,
                    // If the symbol is bound to something like `5`, this isn't a command.
                    Some(_) => return Ok(None),
                    // If the symbol is not bound, then it is a command.
                    None => name,
                };
                let mut cmd = Command::new(cmd_name);
                cmd.current_dir(env.get_cwd()).envs(bindings).args(
                    args.iter()
                        .filter(|&x| x != &Expression::None)
                        .map(|x| Ok(x.eval(env)?.to_string()))
                        .collect::<Result<Vec<String>, Error>>()?,
                );
                Some(cmd)
            }
            _ => None,
        },

        // If the command is an undefined symbol, or an alias.
        Expression::Symbol(name) => match env.get_ref(name) {
            // If the symbol is an alias, then execute the alias.
            Some(Expression::Symbol(name)) => {
                let mut cmd = Command::new(name);
                cmd.current_dir(env.get_cwd()).envs(bindings);
                Some(cmd)
            }
            // If the symbol is bound to something like `5`, this isn't a command.
            Some(_) => None,
            // If the symbol is not defined, use the symbol as the program name.
            None => {
                let mut cmd = Command::new(name);
                cmd.current_dir(env.get_cwd()).envs(bindings);
                Some(cmd)
            }
        },

        // Other types of expressions cannot be commands.
        _ => None,
    })
}
