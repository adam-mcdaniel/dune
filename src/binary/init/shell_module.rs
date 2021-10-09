use std::env::current_exe;

use common_macros::b_tree_map;
use dune::{Expression, VERSION};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("author") => Expression::String("Adam McDaniel (https://adam-mcdaniel.net)".to_string()),
        String::from("git") => Expression::String("https://github.com/adam-mcdaniel/dune".to_string()),
        String::from("version") => Expression::String(VERSION.to_string()),
        String::from("path") => {
            if let Ok(path) = current_exe() {
                Expression::String(path.to_str().unwrap().to_string())
            } else {
                Expression::None
            }
        }
    })
    .into()
}
