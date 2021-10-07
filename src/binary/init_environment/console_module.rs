use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("write") => Expression::builtin("write", write, "write text to a specific position in the console"),
        String::from("title") => Expression::builtin("title", title, "set the title of the console"),
        String::from("clear") => Expression::builtin("clear", clear, "clear the console"),
    })
    .into()
}

fn write(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("write", &args, 3)?;
    print!(
        "\x1b[s\x1b[{line};{column}H\x1b[{line};{column}f{content}\x1b[u",
        line = args[1].eval(env)?,
        column = args[0].eval(env)?,
        content = args[2].eval(env)?
    );
    Ok(Expression::None)
}

fn title(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("title", &args, 1)?;
    print!("\x1b]2;{}\x1b[0m", args[0].eval(env)?);
    Ok(Expression::None)
}

fn clear(args: Vec<Expression>, _env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("clear", &args, 1)?;
    print!("\x1b[2J\x1b[H");
    Ok(Expression::None)
}
