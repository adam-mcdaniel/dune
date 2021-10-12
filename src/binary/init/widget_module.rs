use common_macros::b_tree_map;
use dune::{Environment, Error, Expression};

pub fn get() -> Expression {
    (b_tree_map! {
        String::from("create") => Expression::builtin("create", create, "create a text widget"),
        String::from("joinx") => Expression::builtin("joinx", joinx, "join two widgets horizontally"),
        String::from("joiny") => Expression::builtin("joiny", joiny, "join two widgets vertically")
    })
    .into()
}

fn create(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_exact_args_len("create", &args, 4)?;
    let title = args[0].eval(env)?.to_string();
    let title_len = title.chars().count();

    let text_width = match args[2].eval(env)? {
        Expression::Integer(n) if n > 4 => n,
        otherwise => {
            return Err(Error::CustomError(format!(
                "expected width argument to be integer greater than 4, but got {}",
                otherwise
            )))
        }
    } as usize
        - 2;

    let text = textwrap::fill(&args[1].eval(env)?.to_string(), text_width);

    let widget_height = match args[3].eval(env)? {
        Expression::Integer(n) if n >= 3 => n,
        otherwise => {
            return Err(Error::CustomError(format!(
                "expected height argument to be an integer greater than 2, but got {}",
                otherwise
            )))
        }
    } as usize;

    if text_width < title_len {
        Err(Error::CustomError(String::from(
            "width is less than title length",
        )))
    } else {
        let mut left_border_half =
            "─".repeat(((text_width - title_len) as f64 / 2.0).round() as usize);
        let right_border_half = left_border_half.clone();
        let left_len = left_border_half.chars().count();
        if (left_len * 2 + title_len + 2) > text_width + 2 {
            left_border_half.pop();
        }

        let mut result = format!(
            "┌{left_side}{}{right_side}┐\n",
            title,
            left_side = left_border_half,
            right_side = right_border_half
        );
        let width = result.chars().count() - 1;

        let mut lines = 1;
        let mut i = 0;
        for ch in text.replace('\r', "").chars() {
            if i == 0 {
                result.push(' ');
                i += 1;
            }

            if ch == '\n' {
                lines += 1;
                result += &" ".repeat(width - i);
                i = width;
            } else {
                result.push(ch);
            }

            if lines == widget_height - 1 {
                break;
            }

            if i >= width - 1 {
                result += "\n";
                i = 0;
            } else {
                i += 1;
            }
        }

        result += &" ".repeat(width - i);

        while result.lines().count() < widget_height - 1 {
            result += "\n";
            result += &" ".repeat(width);
        }

        result += &format!(
            "\n└{left_side}{}{right_side}┘",
            "─".repeat(title_len),
            left_side = left_border_half,
            right_side = right_border_half
        );

        Ok(result.into())
    }
}

fn joinx(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_args_len("joinx", &args, 2..)?;

    let mut string_args = vec![];
    let mut height = 0;
    for (i, arg) in args.iter().enumerate() {
        match arg.eval(env)? {
            Expression::String(s) => {
                let lines = s.lines().map(ToString::to_string).collect::<Vec<String>>();
                string_args.push(lines.clone());

                height = string_args[0].len();

                if height != lines.len() {
                    return Err(Error::CustomError(format!(
                        "Heights of horizontally added widgets must be equal, \
                            first widget height={}, {}th widget height={}",
                        height,
                        i,
                        lines.len()
                    )));
                }
            }
            otherwise => {
                return Err(Error::CustomError(format!(
                    "expected string, but got {}",
                    otherwise
                )))
            }
        }
    }

    let mut result = String::new();

    for line_n in 0..height {
        for arg in &string_args {
            result += &arg[line_n].replace('\r', "");
        }
        result += "\n";
    }

    Ok(result.into())
}

fn joiny(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    super::check_args_len("joiny", &args, 2..)?;

    let mut string_args = vec![];
    for (i, arg) in args.iter().enumerate() {
        match arg.eval(env)? {
            Expression::String(s) => {
                string_args.push(s.trim().to_string());

                let width = string_args[0].lines().next().unwrap().chars().count();
                let this_width = string_args[i].lines().next().unwrap().chars().count();
                if width != this_width {
                    return Err(Error::CustomError(format!(
                        "Widths of vertically added widgets must be equal, \
                            first widget height={}, {}th widget height={}",
                        width, i, this_width
                    )));
                }
            }
            otherwise => {
                return Err(Error::CustomError(format!(
                    "expected string, but got {}",
                    otherwise
                )))
            }
        }
    }

    Ok(string_args
        .into_iter()
        .map(|x| x.replace('\r', ""))
        .collect::<Vec<_>>()
        .join("\n")
        .into())
}
