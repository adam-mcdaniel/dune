use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get(env: &mut Environment) -> Expression {
    let mut dir_tree = b_tree_map! {};

    if let Some(home_dir) = dirs::home_dir() {
        let home_dir = home_dir.into_os_string().into_string().unwrap();
        env.set_cwd(&home_dir);
        dir_tree.insert("home".to_string(), Expression::from(home_dir.clone()));
        env.define("HOME", Expression::String(home_dir));
    }

    if let Ok(cwd) = current_dir() {
        env.set_cwd(&cwd.into_os_string().into_string().unwrap());
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
        String::from("canon") => Expression::builtin("canon", |args, env| {
            super::check_exact_args_len("canon", &args, 1)?;
            let cwd = PathBuf::from(env.get_cwd());
            let path = cwd.join(args[0].eval(env)?.to_string());

            if let Ok(canon_path) = dunce::canonicalize(&path) {
                Ok(canon_path.into_os_string().into_string().unwrap().into())
            } else {
                Err(Error::CustomError(format!("could not canonicalize path {}", path.display())))
            }
        }, "resolve, normalize, and absolutize a relative path"),
        String::from("mkdir") => Expression::builtin("mkdir", |args, env| {
            super::check_exact_args_len("mkdir", &args, 1)?;
            let cwd = PathBuf::from(env.get_cwd());
            let dir = cwd.join(args[0].eval(env)?.to_string());

            if std::fs::create_dir_all(&dir).is_err() {
                return Err(Error::CustomError(format!("could not create directory {}", dir.display())));
            }

            Ok(Expression::None)
        }, "create a directory and its parent directories"),
        String::from("rmdir") => Expression::builtin("rmdir", |args, env| {
            super::check_exact_args_len("rmdir", &args, 1)?;
            let cwd = PathBuf::from(env.get_cwd());
            let dir = cwd.join(args[0].eval(env)?.to_string());

            if std::fs::remove_dir(&dir).is_err() {
                return Err(Error::CustomError(format!("could not remove directory {}, is it empty?", dir.display())));
            }

            Ok(Expression::None)
        }, "remove an empty directory"),
        String::from("mv") => Expression::builtin("mv", |args, env| {
            super::check_exact_args_len("mv", &args, 2)?;
            let cwd = PathBuf::from(env.get_cwd());
            let src = cwd.join(args[0].eval(env)?.to_string());
            let dst = cwd.join(args[1].eval(env)?.to_string());

            move_path(&src, &dst)?;

            Ok(Expression::None)
        }, "move a source path to a destination path"),
        String::from("cp") => Expression::builtin("cp", |args, env| {
            super::check_exact_args_len("cp", &args, 2)?;
            let cwd = PathBuf::from(env.get_cwd());
            let src = cwd.join(args[0].eval(env)?.to_string());
            let dst = cwd.join(args[1].eval(env)?.to_string());

            copy_path(&src, &dst)?;

            Ok(Expression::None)
        }, "copy a source path to a destination path"),
        String::from("rm") => Expression::builtin("rm", |args, env| {
            super::check_exact_args_len("rm", &args, 1)?;
            let cwd = PathBuf::from(env.get_cwd());
            let path = cwd.join(args[0].eval(env)?.to_string());

            remove_path(&path)?;

            Ok(Expression::None)
        }, "remove a file or directory from the filesystem"),
        String::from("ls") => Expression::builtin("ls", |args, env| {
            super::check_exact_args_len("ls", &args, 1)?;
            let cwd = PathBuf::from(env.get_cwd());
            let dir = cwd.join(args[0].eval(env)?.to_string());

            list_directory(&dir)
        }, "get a directory's entries as a list of strings"),
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

            match std::fs::read_to_string(&path) {
                // First, try to read the contents as a string.
                Ok(contents) => Ok(contents.into()),
                // If that fails, try to read them as a list of bytes.
                Err(_) => match std::fs::read(&path) {
                    Ok(contents) => Ok(Expression::Bytes(contents)),
                    Err(_) => Err(Error::CustomError(format!("could not read file {}", file)))
                }
            }
        }, "read a file's contents"),

        String::from("write") => Expression::builtin("write", |args, env| {
            super::check_exact_args_len("write", &args, 2)?;
            let mut path = PathBuf::from(env.get_cwd());
            let file = args[0].eval(env)?;
            path = path.join(file.to_string());

            let contents = args[1].eval(env)?;

            // If the contents are bytes, write the bytes directly to the file.
            let result = if let Expression::Bytes(bytes) = contents {
                std::fs::write(path, bytes)
            } else {
                // Otherwise, convert the contents to a pretty string and write that.
                std::fs::write(path, contents.to_string())
            };

            match result {
                Ok(()) => Ok(Expression::None),
                Err(e) => Err(Error::CustomError(format!("could not write to file {}: {:?}", file, e)))
            }
        }, "write to a file with some contents"),
    };

    env.define_module("fs", fs_module.clone());
    Expression::Map(fs_module)
}

/// Copy one path to another path.
fn copy_path(src: &Path, dst: &Path) -> Result<(), Error> {
    if src == dst {
        return Ok(());
    }

    // If the destination exists, simply throw an error.
    if dst.exists() {
        return Err(Error::CustomError(format!(
            "destination {} already exists",
            dst.display()
        )));
    }

    // If the source is a directory, recursively copy the directory.
    if src.is_dir() {
        // Create the destination directory and all of its parents.
        if std::fs::create_dir_all(dst).is_err() {
            return Err(Error::CustomError(format!(
                "could not create directory {}",
                dst.display()
            )));
        }

        // Get the entries of the source directory
        if let Ok(entries) = std::fs::read_dir(src) {
            for entry in entries {
                // For every valid entry, copy it to the destination recursively.
                if let Ok(entry) = entry {
                    // Get the source file's new path relative to the destination
                    let path = entry.path();
                    let dst_path = dst.join(entry.file_name());
                    // Copy the path to its destination.
                    copy_path(&path, &dst_path)?;
                } else {
                    // If an entry is not valid, throw an error.
                    return Err(Error::CustomError(format!(
                        "could not read directory {}",
                        src.display()
                    )));
                }
            }
        } else {
            // If we cannot read the entries of the source directory, throw an error.
            return Err(Error::CustomError(format!(
                "could not create directory {}",
                dst.display()
            )));
        }
    // If the directory is a file, try to copy it.
    } else if std::fs::copy(src, dst).is_err() {
        // If copying the file fails, throw an error.
        return Err(Error::CustomError(format!(
            "could not copy file {} to {}",
            src.display(),
            dst.display()
        )));
    }
    Ok(())
}

/// Moves one path to another path.
fn move_path(src: &Path, dst: &Path) -> Result<(), Error> {
    if src == dst {
        return Ok(());
    }

    // If the destination exists, simply throw an error.
    if dst.exists() {
        return Err(Error::CustomError(format!(
            "destination {} already exists",
            dst.display()
        )));
    }

    // Attempt to rename the source to the destination.
    if std::fs::rename(src, dst).is_err() {
        return Err(Error::CustomError(format!(
            "could not move {} to {}",
            src.display(),
            dst.display()
        )));
    }

    Ok(())
}

/// Removes a file or directory from the file system.
fn remove_path(path: &Path) -> Result<(), Error> {
    if path.is_dir() {
        if std::fs::remove_dir_all(path).is_err() {
            return Err(Error::CustomError(format!(
                "could not remove directory {}",
                path.display()
            )));
        }
    } else if std::fs::remove_file(path).is_err() {
        return Err(Error::CustomError(format!(
            "could not remove file {}",
            path.display()
        )));
    }

    Ok(())
}

/// Returns the paths of entries in a directory as a list of strings.
fn list_directory(dir: &Path) -> Result<Expression, Error> {
    if dir.is_dir() {
        // The list of paths (as strings) in the directory we will return.
        let mut result = vec![];

        // Read the directory's items
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                // For every valid entry in the directory,
                // add it's filename as a string to the result list.
                if let Ok(entry) = entry {
                    let file_name_osstring = entry.file_name();
                    result.push(match file_name_osstring.into_string() {
                        Ok(file_name) => file_name,
                        // If we cannot directly convert the filename to a string,
                        // it's probably an invalid UTF-8 string.
                        // In this case, we remove the invalid bytes and try again.
                        Err(file_name) => file_name.to_string_lossy().to_string(),
                    });
                } else {
                    // If an entry is invalid, throw an error.
                    return Err(Error::CustomError(format!(
                        "could not read entries in {}",
                        dir.display()
                    )));
                }
            }
        } else {
            // If we cannot read the directory's entries, throw an error.
            return Err(Error::CustomError(format!(
                "could not read directory {}",
                dir.display()
            )));
        }

        // Return the list of paths as a list.
        Ok(result.into())
    } else if dir.is_file() {
        // If the path is a file, return the file's name as a string in a list.
        return Ok(Expression::List(vec![format!("{}", dir.display()).into()]));
    } else {
        // Otherwise, the path is neither a file nor a directory, so throw an error.
        return Err(Error::CustomError(format!(
            "{} does not exist",
            dir.display()
        )));
    }
}
