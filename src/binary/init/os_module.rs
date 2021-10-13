use common_macros::b_tree_map;
use dune::{Error, Expression};
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
        String::from("cd") => Expression::builtin(
            "cd",
            |args, env| match args[0].clone().eval(env)? {
                Expression::Symbol(path) | Expression::String(path) => {
                    if let Ok(new_cwd) = dunce::canonicalize(PathBuf::from(env.get_cwd()).join(path)) {
                        // It's not necessary that this succeeds, because
                        // Dune does everything relative to the `CWD` bound variable.
                        // This is mostly to reduce any unintended behavior from
                        // other libraries like `rustyline`.
                        let _ = std::env::set_current_dir(&new_cwd);
                        env.set_cwd(new_cwd.into_os_string().into_string().unwrap());
                    }
                    Ok(Expression::None)
                }
                _ => Err(Error::CustomError(format!(
                    "expected string or symbol, got {:?}",
                    args[0]
                ))),
            },
            "change directories",
        )
    })
    .into()
}
