use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};
use rand::{distributions::Uniform, prelude::SliceRandom, Rng};

pub fn get() -> Expression {
    fn int(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
        super::check_exact_args_len("int", &args, 2)?;
        match (args[0].eval(env)?, args[1].eval(env)?) {
            (Expression::Integer(l), Expression::Integer(h)) => {
                let mut rng = rand::thread_rng();
                let n = Uniform::new(l, h);
                Ok(Expression::Integer(rng.sample(n)))
            }
            (l, h) => Err(Error::CustomError(format!(
                "expected two integers, but got {} and {}",
                l, h
            ))),
        }
    }

    fn choose(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
        super::check_exact_args_len("choose", &args, 1)?;
        match args[0].eval(env)? {
            Expression::List(list) => {
                let mut rng = rand::thread_rng();
                let n = Uniform::new(0, list.len());
                Ok(list[rng.sample(n)].clone())
            }
            otherwise => Err(Error::CustomError(format!(
                "expected a list, but got {}",
                otherwise
            ))),
        }
    }

    fn shuffle(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
        super::check_exact_args_len("shuffle", &args, 1)?;
        match args[0].eval(env)? {
            Expression::List(mut list) => {
                let mut rng = rand::thread_rng();
                list.shuffle(&mut rng);
                Ok(list.into())
            }
            otherwise => Err(Error::CustomError(format!(
                "expected a list, but got {}",
                otherwise
            ))),
        }
    }

    b_tree_map! {
        String::from("int") => Expression::builtin("int", int, "get a random integer between two numbers (exclusive)"),
        String::from("choose") => Expression::builtin("choose", choose, "choose a random item in a list"),
        String::from("shuffle") => Expression::builtin("shuffle", shuffle, "shuffle a list randomly"),
    }
    .into()
}
