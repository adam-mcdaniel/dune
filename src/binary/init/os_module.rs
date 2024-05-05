use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};
use std::path::PathBuf;

pub fn get() -> Expression {
    let os = os_info::get();
    let os_type = os.os_type();

    (b_tree_map! {
        String::from("name") => Expression::from(crate::get_os_name(&os_type)),
        String::from("family") => crate::get_os_family(&os_type).into(),
        String::from("version") => os.version().to_string().into(),
        String::from("exit") => Expression::builtin(
            "exit",
            |args, env| {
                if args.is_empty() {
                    std::process::exit(0);
                } else if let Expression::Integer(n) = args[0].clone().eval(env)? {
                    std::process::exit(n as i32);
                } else {
                    Err(Error::CustomError(format!(
                        "expected integer but got `{:?}`",
                        args[0]
                    )))
                }
            },
            "exit the shell",
        ),
        String::from("cd") => Expression::builtin("cd", cd, "change directories"),
    })
    .into()
}

fn cd(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, dune::Error> {
    super::check_exact_args_len("cd", &args, 1)?;

    match args[0].eval(env)? {
        Expression::Symbol(path) | Expression::String(path) => {
            let abs_path = PathBuf::from(env.get_cwd()).join(path);

            let new_cwd = dunce::canonicalize(&abs_path).map_err(|e| {
                dune::Error::CustomError(match format!("{:?}", e.kind()).as_str() {
                    "NotFound" => {
                        format!("the directory {:?} does not exist", abs_path)
                    }
                    "NotADirectory" => {
                        format!("a path segment in {:?} is not a directory", abs_path)
                    }
                    _ => format!(
                        "could not change to directory {:?}\n  reason: {}",
                        abs_path, e
                    ),
                })
            })?;

            std::env::set_current_dir(&new_cwd).map_err(|e| {
                dune::Error::CustomError(match format!("{:?}", e.kind()).as_str() {
                    "PermissionDenied" => {
                        format!("you don't have permission to read directory {:?}", new_cwd)
                    }
                    "NotADirectory" => {
                        format!("{:?} is not a directory", new_cwd)
                    }
                    _ => format!(
                        "could not change directory to {:?}\n  reason: {}",
                        new_cwd, e
                    ),
                })
            })?;

            env.set_cwd(new_cwd.into_os_string().into_string().unwrap());
            Ok(Expression::None)
        }
        

        other => {
            // Try to convert the argument to a string
            let path = other.to_string();
            cd(vec![Expression::String(path)], env)
        }
    }
}
