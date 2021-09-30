pub type Int = i64;

mod expr;
pub use expr::*;

mod env;
pub use env::*;

mod error;
pub use error::*;

mod parser;
pub use parser::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
