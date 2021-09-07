use super::{Expression, SyntaxError};

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    CannotApply(Expression, Vec<Expression>),
    SymbolNotDefined(String),
    CommandFailed(String, Vec<Expression>),
    ForNonList(Expression),
    CustomError(String),
    SyntaxError(SyntaxError),
}

use core::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CannotApply(expr, args) => {
                write!(f, "cannot apply `{:?}` to the arguments {:?}", expr, args)
            }
            Self::SymbolNotDefined(name) => {
                write!(f, "symbol \"{}\" not defined", name)
            }
            Self::CommandFailed(name, args) => {
                write!(
                    f,
                    "command `{:?}` failed",
                    Expression::Apply(Box::new(Expression::Symbol(name.clone())), args.clone())
                )
            }
            Self::ForNonList(nonlist) => {
                write!(f, "attempted to iterate over non-list `{:?}`", nonlist)
            }
            Self::CustomError(e) => {
                write!(f, "{}", e)
            }
            Self::SyntaxError(e) => {
                write!(f, "{:?}", e)
            }
        }
    }
}
