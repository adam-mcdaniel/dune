use rand::seq::SliceRandom;
use std::env::current_exe;

use common_macros::b_tree_map;
use dune::{Expression, VERSION};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("author") => Expression::String("Adam McDaniel (https://adam-mcdaniel.net)".to_string()),
        String::from("git") => Expression::String("https://github.com/adam-mcdaniel/dune".to_string()),
        String::from("homepage") => Expression::String("https://github.com/adam-mcdaniel/dune".to_string()),
        String::from("version") => Expression::String(VERSION.to_string()),
        String::from("path") => {
            if let Ok(path) = current_exe() {
                Expression::String(path.to_str().unwrap().to_string())
            } else {
                Expression::None
            }
        },
        String::from("suggestion") => {
            // Choose a random suggestion from the `help/suggestions.txt` file.
            let suggestions = include_str!("../help/suggestions.txt");
            let suggestions = suggestions.split("\n").collect::<Vec<&str>>();
            let suggestion = suggestions.choose(&mut rand::thread_rng()).unwrap();
            Expression::String(suggestion.to_string())
        },
        String::from("license") => Expression::String("APACHE-2.0".to_string()),
        String::from("prelude") => {
            // Home directory + .dune-prelude
            let prelude_path = if let Some(home_dir) = dirs::home_dir() {
                let prelude_path = home_dir.join(".dune-prelude");
                if prelude_path.exists() {
                    Expression::String(prelude_path.to_str().unwrap().to_string())
                } else {
                    Expression::None
                }
            } else {
                Expression::None
            };

            prelude_path
        }
    })
    .into()
}
