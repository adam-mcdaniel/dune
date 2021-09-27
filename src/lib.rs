pub type Int = i64;

mod expr;
pub use expr::*;

mod env;
pub use env::*;

mod error;
pub use error::*;

mod parser;
pub use parser::*;

pub const VERSION: &'static str = "0.1.3";