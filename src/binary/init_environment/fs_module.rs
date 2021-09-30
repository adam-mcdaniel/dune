use std::path::PathBuf;

use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn add_to(env: &mut Environment) {
    let mut dir_tree = b_tree_map! {};

    if let Some(home_dir) = dirs::home_dir() {
        let home_dir = home_dir.into_os_string().into_string().unwrap();
        env.set_cwd(&home_dir);

        dir_tree.insert("home".to_string(), Expression::from(home_dir.clone()));
        env.define("HOME", Expression::String(home_dir));
    }

    if let Some(desk_dir) = dirs::desktop_dir() {
        let desk_dir = desk_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("desk".to_string(), desk_dir.clone().into());
        env.define("DESK", Expression::String(desk_dir));
    }

    if let Some(docs_dir) = dirs::document_dir() {
        let docs_dir = docs_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("docs".to_string(), docs_dir.clone().into());
        env.define("DOCS", Expression::String(docs_dir));
    }

    if let Some(down_dir) = dirs::download_dir() {
        let down_dir = down_dir.into_os_string().into_string().unwrap();
        dir_tree.insert("down".to_string(), down_dir.clone().into());
        env.define("DOWN", Expression::String(down_dir));
    }

    let fs_module = b_tree_map! {
        String::from("dirs") => dir_tree.into(),
        String::from("exists") => Expression::builtin("exists", |args, env| {
            super::check_exact_args_len("exists", &args, 1)?;
            let path = PathBuf::from(env.get_cwd());

            Ok(path.join(args[0].eval(env)?.to_string()).exists().into())
        }, "check if a given file path exists"),

        String::from("isdir") => Expression::builtin("isdir", |args, env| {
            super::check_exact_args_len("isdir", &args, 1)?;
            let path = PathBuf::from(env.get_cwd());

            Ok(path.join(args[0].eval(env)?.to_string()).is_dir().into())
        }, "check if a given path is a directory"),

        String::from("isfile") => Expression::builtin("isfile", |args, env| {
            super::check_exact_args_len("isfile", &args, 1)?;
            let path = PathBuf::from(env.get_cwd());

            Ok(path.join(args[0].eval(env)?.to_string()).is_file().into())
        }, "check if a given path is a file"),

        String::from("read") => Expression::builtin("read", |args, env| {
            super::check_exact_args_len("read", &args, 1)?;
            let mut path = PathBuf::from(env.get_cwd());
            let file = args[0].eval(env)?;
            path = path.join(file.to_string());

            match std::fs::read_to_string(path) {
                Ok(contents) => Ok(contents.into()),
                Err(_) => Err(Error::CustomError(format!("could not read file {}", file)))
            }
        }, "read a file"),

        String::from("write") => Expression::builtin("write", |args, env| {
            super::check_exact_args_len("write", &args, 2)?;
            let mut path = PathBuf::from(env.get_cwd());
            let file = args[0].eval(env)?;
            path = path.join(file.to_string());
            match std::fs::write(path, args[1].eval(env)?.to_string()) {
                Ok(()) => Ok(Expression::None),
                Err(_) => Err(Error::CustomError(format!("could not write to file {}", file)))
            }
        }, "write to a file"),
    }
    .into();

    env.define("fs", fs_module);
}
