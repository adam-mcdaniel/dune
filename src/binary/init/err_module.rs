use common_macros::b_tree_map;
use dune::{Error, Expression, Environment, Int};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("try") => Expression::builtin("try", try_builtin,
            "try an expression or apply an error handler to an error"),
        String::from("codes") => Error::codes()
    })
    .into()
}


fn try_builtin(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    // Try to evaluate the first argument, if it fails, apply the second argument to the error
    // message.
    if args.len() != 2 {
        return Err(Error::CustomError(
            "try requires exactly two arguments: an expression to try to evaluate, and an error handler that takes an error as an argument".to_string(),
        ));
    }

    match args[0].eval(env) {
        Err(err) => {
            let handler = args[1].clone();
            
            Expression::Apply(
                Box::new(handler),
                vec![
                    Expression::Map(b_tree_map! {
                        String::from("message") => Expression::String(err.to_string()),
                        String::from("code") => Expression::Integer(Int::from(err.code())),
                        String::from("expression") => Expression::Quote(Box::new(args[0].clone()))
                    }),
                ],
            )
            .eval(env)
        },
        result => result
    }
}