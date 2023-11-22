use super::Int;
use common_macros::b_tree_map;
use dune::{Error, Expression};
use lazy_static::lazy_static;
use std::sync::RwLock;
lazy_static! {
    static ref LOG_LEVEL: RwLock<Int> = RwLock::new(0);
}

const NONE: Int = 0;
const ERROR: Int = 1;
const WARN: Int = 2;
const INFO: Int = 3;
const DEBUG: Int = 4;
const TRACE: Int = 5;

fn is_log_level_enabled(level: Int) -> bool {
    if level <= NONE {
        return true;
    }
    *LOG_LEVEL.read().unwrap() >= level
}

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("level") => Expression::Map(b_tree_map! {
            String::from("none") => Expression::Integer(NONE),
            String::from("trace") => Expression::Integer(TRACE),
            String::from("debug") => Expression::Integer(DEBUG),
            String::from("info") => Expression::Integer(INFO),
            String::from("warn") => Expression::Integer(WARN),
            String::from("error") => Expression::Integer(ERROR),
        }),

        String::from("set-level") => Expression::builtin("set-level", |args, _env| {
            super::check_exact_args_len("set-level", &args, 1)?;
            let level = args[0].clone().eval(_env)?;
            if let Expression::Integer(level) = level {
                *LOG_LEVEL.write().unwrap() = level;
                return Ok(Expression::None)
            } else {
                return Err(Error::CustomError(format!("expected an integer, found {}", level)))
            }
        }, "set the log level"),

        String::from("get-level") => Expression::builtin("get-level", |args, _env| {
            super::check_exact_args_len("get-level", &args, 0)?;
            return Ok(Expression::Integer(*LOG_LEVEL.read().unwrap()))
        }, "get the log level"),

        String::from("disable") => Expression::builtin("disable", |args, _env| {
            super::check_exact_args_len("disable", &args, 0)?;
            *LOG_LEVEL.write().unwrap() = NONE;
            return Ok(Expression::None)
        }, "disable logging"),

        String::from("enabled?") => Expression::builtin("enabled?", |args, env| {
            super::check_exact_args_len("enabled?", &args, 1)?;
            let level = args[0].clone().eval(env)?;
            if let Expression::Integer(level) = level {
                return Ok(Expression::Boolean(is_log_level_enabled(level)))
            } else {
                return Err(Error::CustomError(format!("expected an integer, found {}", level)))
            }
        }, "check if a log level is enabled"),

        String::from("info") => Expression::builtin("info", |args, env| {
            if !is_log_level_enabled(INFO) {
                return Ok(Expression::None)
            }

            // Like `echo`, but with green formatting and an `[INFO]` prefix on new lines.
            let mut last_was_newline = true;
            let prefix = "\x1b[92m[INFO]\x1b[m ";
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                // Split into lines and print each line with a green `[INFO]` prefix.
                let lines = x.split('\n').collect::<Vec<&str>>();
                if lines.len() > 1 {
                    for (i, line) in lines.iter().enumerate() {
                        if i < lines.len() - 1 {
                            println!("{}{}", prefix, line);
                        } else if line.len() > 0 {
                            print!("{}{}", prefix, line);
                        }
                    }
                } else {
                    if last_was_newline {
                        print!("{}", prefix);
                    }
                    if i < args.len() - 1 {
                        print!("{} ", x)
                    } else {
                        println!("{}", x)
                    }
                }

                last_was_newline = x.ends_with('\n');
            }

            Ok(Expression::None)
        }, "print a message to the console with green formatting and an `[INFO]` prefix on new lines"),

        String::from("warn") => Expression::builtin("warn", |args, env| {
            if !is_log_level_enabled(WARN) {
                return Ok(Expression::None)
            }

            // Like `echo`, but with green formatting and an `[INFO]` prefix on new lines.
            let mut last_was_newline = true;
            let prefix = "\x1b[93m[WARN]\x1b[m ";
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                // Split into lines and print each line with a green `[INFO]` prefix.
                let lines = x.split('\n').collect::<Vec<&str>>();
                if lines.len() > 1 {
                    for (i, line) in lines.iter().enumerate() {
                        if i < lines.len() - 1 {
                            println!("{}{}", prefix, line);
                        } else if line.len() > 0 {
                            print!("{}{}", prefix, line);
                        }
                    }
                } else {
                    if last_was_newline {
                        print!("{}", prefix);
                    }
                    if i < args.len() - 1 {
                        print!("{} ", x)
                    } else {
                        println!("{}", x)
                    }
                }

                last_was_newline = x.ends_with('\n');
            }

            Ok(Expression::None)
        }, "print a message to the console with yellow formatting and an `[WARN]` prefix on new lines"),

        String::from("debug") => Expression::builtin("debug", |args, env| {
            if !is_log_level_enabled(DEBUG) {
                return Ok(Expression::None)
            }


            // Like `echo`, but with green formatting and an `[INFO]` prefix on new lines.
            let mut last_was_newline = true;
            let prefix = "\x1b[94m[DEBUG]\x1b[m ";
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                // Split into lines and print each line with a green `[INFO]` prefix.
                let lines = x.split('\n').collect::<Vec<&str>>();
                if lines.len() > 1 {
                    for (i, line) in lines.iter().enumerate() {
                        if i < lines.len() - 1 {
                            println!("{}{}", prefix, line);
                        } else if line.len() > 0 {
                            print!("{}{}", prefix, line);
                        }
                    }
                } else {
                    if last_was_newline {
                        print!("{}", prefix);
                    }
                    if i < args.len() - 1 {
                        print!("{} ", x)
                    } else {
                        println!("{}", x)
                    }
                }

                last_was_newline = x.ends_with('\n');
            }

            Ok(Expression::None)
        }, "print a message to the console with blue formatting and an `[DEBUG]` prefix on new lines"),

        String::from("error") => Expression::builtin("error", |args, env| {
            if !is_log_level_enabled(ERROR) {
                return Ok(Expression::None)
            }

            // Like `echo`, but with green formatting and an `[INFO]` prefix on new lines.
            let mut last_was_newline = true;
            let prefix = "\x1b[91m[ERROR]\x1b[m ";
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                // Split into lines and print each line with a green `[INFO]` prefix.
                let lines = x.split('\n').collect::<Vec<&str>>();
                if lines.len() > 1 {
                    for (i, line) in lines.iter().enumerate() {
                        if i < lines.len() - 1 {
                            println!("{}{}", prefix, line);
                        } else if line.len() > 0 {
                            print!("{}{}", prefix, line);
                        }
                    }
                } else {
                    if last_was_newline {
                        print!("{}", prefix);
                    }
                    if i < args.len() - 1 {
                        print!("{} ", x)
                    } else {
                        println!("{}", x)
                    }
                }

                last_was_newline = x.ends_with('\n');
            }
            Ok(Expression::None)
        }, "print a message to the console with red formatting and an `[ERROR]` prefix on new lines"),

        String::from("trace") => Expression::builtin("trace", |args, env| {
            if !is_log_level_enabled(TRACE) {
                return Ok(Expression::None)
            }

            // Like `echo`, but with green formatting and an `[INFO]` prefix on new lines.
            let mut last_was_newline = true;
            let prefix = "\x1b[95m[TRACE]\x1b[m ";
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                // Split into lines and print each line with a green `[INFO]` prefix.
                let lines = x.split('\n').collect::<Vec<&str>>();
                if lines.len() > 1 {
                    for (i, line) in lines.iter().enumerate() {
                        if i < lines.len() - 1 {
                            println!("{}{}", prefix, line);
                        } else if line.len() > 0 {
                            print!("{}{}", prefix, line);
                        }
                    }
                } else {
                    if last_was_newline {
                        print!("{}", prefix);
                    }
                    if i < args.len() - 1 {
                        print!("{} ", x)
                    } else {
                        println!("{}", x)
                    }
                }

                last_was_newline = x.ends_with('\n');
            }

            Ok(Expression::None)
        }, "print a message to the console with magenta formatting and an `[TRACE]` prefix on new lines"),

        String::from("echo") => Expression::builtin("echo", |args, env| {
            // Like `echo`, but with no formatting.
            for (i, arg) in args.iter().enumerate() {
                let x = arg.clone().eval(env)?.to_string();
                if i < args.len() - 1 {
                    print!("{} ", x)
                } else {
                    println!("{}", x)
                }
            }

            Ok(Expression::None)
        }, "print a message to the console with no formatting"),
    }).into()
}
