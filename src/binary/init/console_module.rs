use common_macros::b_tree_map;
use dune::{Int, Environment, Error, Expression};
use terminal_size::{terminal_size, Width, Height};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("width") => Expression::builtin("width", width, "get the width of the console"),
        String::from("height") => Expression::builtin("height", height, "get the height of the console"),
        String::from("write") => Expression::builtin("write", write, "write text to a specific position in the console"),
        String::from("title") => Expression::builtin("title", title, "set the title of the console"),
        String::from("clear") => Expression::builtin("clear", clear, "clear the console"),
    })
    .into()
}

fn width(_: Vec<Expression>, _: &mut Environment) -> Result<Expression, Error> {
    Ok(match terminal_size() {
        Some((Width(w), _)) => (w as Int).into(),
        _ => Expression::None,
    })
}

fn height(_: Vec<Expression>, _: &mut Environment) -> Result<Expression, Error> {
    Ok(match terminal_size() {
        Some((_, Height(h))) => (h as Int).into(),
        _ => Expression::None,
    })
}

fn write(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("write", &args, 3)?;
    match (args[0].eval(env)?, args[1].eval(env)?, args[2].eval(env)?) {
        (Expression::Integer(x), Expression::Integer(y), Expression::String(text)) => {
            print!(
                "\x1b[s\x1b[{line};{column}H\x1b[{line};{column}f{content}\x1b[u",
                line = x,
                column = y,
                content = text
            );
        }
        (x, y, text) => {
            return Err(Error::CustomError(format!(
                "expected int, int, and a string, but got: `{:?}`, `{:?}`, and `{:?}`",
                x, y, text
            )))
        }
    }
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
