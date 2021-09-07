use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::one_of,
    combinator::{eof, map, opt},
    error::{ErrorKind, ParseError},
    multi::{count, many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use std::collections::BTreeMap;

use super::{Environment, Expression, Int};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyntaxError {
    Expected {
        input: String,
        expected: String,
        found: Option<String>,
        hint: Option<String>,
    },
    Union(Box<Self>, Box<Self>),
    At(String, Box<Self>),
    CustomError(String),
    InternalError,
}

impl SyntaxError {
    fn unrecoverable<A, B, T>(
        input: T,
        expected: T,
        found: Option<T>,
        hint: Option<T>,
    ) -> IResult<A, B, Self>
    where
        T: ToString,
    {
        Err(nom::Err::Failure(Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: found.map(|x| x.to_string()),
            hint: hint.map(|x| x.to_string()),
        }))
    }

    fn expected<A, B, T>(
        input: T,
        expected: T,
        found: Option<T>,
        hint: Option<T>,
    ) -> IResult<A, B, Self>
    where
        T: ToString,
    {
        Err(nom::Err::Error(Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: found.map(|x| x.to_string()),
            hint: hint.map(|x| x.to_string()),
        }))
    }
}

impl<T> ParseError<T> for SyntaxError
where
    T: ToString,
{
    fn from_error_kind(input: T, kind: ErrorKind) -> Self {
        Self::At(
            input.to_string(),
            Box::new(Self::CustomError(format!("expected {:?}", kind))),
        )
    }

    fn append(input: T, kind: ErrorKind, other: Self) -> Self {
        Self::Union(
            Box::new(other),
            Box::new(Self::from_error_kind(input, kind)),
        )
    }

    fn from_char(input: T, expected: char) -> Self {
        Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: None,
            hint: None,
        }
    }

    fn or(self, other: Self) -> Self {
        match self {
            Self::InternalError => other,
            _ => self.clone(),
        }
    }
}

const ASCII_ALPHA: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ASCII_ALPHANUMERIC: &'static str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const ASCII_NONZERO_DIGIT: &'static str = "123456789";
const ASCII_DIGIT: &'static str = "0123456789";
const ASCII_HEX_DIGIT: &'static str = "0123456789ABCDEFabcdef";

pub fn parse_script(input: &str, require_eof: bool) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, mut exprs) = many0(alt((
        terminated(parse_if, pair(try_parse_ws, opt(tag(";")))),
        terminated(parse_for_loop, pair(try_parse_ws, opt(tag(";")))),
        terminated(parse_expression, pair(try_parse_ws, tag(";"))),
    )))(input)?;

    let (input, last) = opt(alt((
        terminated(parse_if, pair(try_parse_ws, opt(tag(";")))),
        terminated(parse_for_loop, pair(try_parse_ws, opt(tag(";")))),
        terminated(parse_expression, pair(try_parse_ws, opt(tag(";")))),
    )))(input)?;

    if let Some(expr) = last {
        exprs.push(expr);
    }

    let (input, _) = try_parse_ws(input)?;
    if require_eof {
        let (input, _) = eof(input)?;
        Ok((input, Expression::Do(exprs)))
    } else {
        Ok((input, Expression::Do(exprs)))
    }
}

fn try_parse_ws(input: &str) -> IResult<&str, (), SyntaxError> {
    let (i, _) = many0(one_of(" \r\t\n"))(input)?;
    Ok((i, ()))
}

fn parse_ws(input: &str) -> IResult<&str, (), SyntaxError> {
    let (i, _) = many1(one_of(" \r\t\n"))(input)?;
    Ok((i, ()))
}

fn parse_keyword(input: &str) -> IResult<&str, &str, SyntaxError> {
    alt((
        tag("False"),
        tag("True"),
        tag("None"),
        tag("then"),
        tag("else"),
        tag("let"),
        tag("for"),
        tag("if"),
        tag("in"),
        tag("to"),
        tag("->"),
        tag("~>"),
        alt((
            tag("=="),
            tag(">="),
            tag("<="),
            tag("&&"),
            tag("||"),
            tag("//"),
            tag("<"),
            tag(">"),
            tag("+"),
            tag("-"),
            tag("'"),
            tag("@"),
        )),
    ))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, String, SyntaxError> {
    match tuple((parse_keyword, parse_ws))(input) {
        Ok(_) => SyntaxError::expected(
            input,
            "symbol",
            Some("keyword"),
            Some("try using an unreserved symbol"),
        ),
        Err(_) => {
            let old_input = input;

            let (input, head) = alt((one_of(ASCII_ALPHA), one_of("_+-.~\\/?&<>$%#^=")))(input)?;

            let (input, tail) = many0(alt((
                one_of(ASCII_ALPHANUMERIC),
                one_of("_+-.~\\/?&<>$%#^="),
            )))(input)?;

            let mut result = String::from(head);

            for ch in tail {
                result.push(ch);
            }

            if let Ok((i, _)) = parse_keyword(&result) {
                if i.is_empty() {
                    return SyntaxError::unrecoverable(
                        old_input,
                        "symbol",
                        Some("keyword"),
                        Some("try using an unreserved symbol"),
                    );
                }
            }

            Ok((input, result))
        }
    }
}

fn parse_digits(input: &str) -> IResult<&str, String, SyntaxError> {
    alt((
        map(tag("0"), |x: &str| x.to_string()),
        map(
            tuple((one_of(ASCII_NONZERO_DIGIT), many0(one_of(ASCII_DIGIT)))),
            |(head, tail)| {
                let mut result = String::from(head);
                for ch in tail {
                    result.push(ch)
                }
                result
            },
        ),
    ))(input)
}

fn parse_integer(input: &str) -> IResult<&str, Int, SyntaxError> {
    let (input, is_positive) = map(opt(tag("-")), |x| x.is_none())(input)?;

    let sign = if is_positive { 1 } else { -1 };

    match parse_digits(input) {
        Ok((input, digits)) => match digits.parse::<Int>() {
            Ok(n) => Ok((input, sign * n)),
            Err(_) => SyntaxError::expected(input, "integer", None, None),
        },
        _ => SyntaxError::expected(input, "integer", None, None),
    }
}

fn parse_float(input: &str) -> IResult<&str, f64, SyntaxError> {
    let (input, is_positive) = map(opt(tag("-")), |x| x.is_none())(input)?;

    let sign = if is_positive { 1.0 } else { -1.0 };

    match parse_digits(input) {
        Ok((input, first_digits)) => {
            let (input, _) = tag(".")(input)?;

            match parse_digits(input) {
                Ok((input, last_digits)) => {
                    match format!("{}.{}", first_digits, last_digits).parse::<f64>() {
                        Ok(n) => Ok((input, sign * n)),
                        Err(_) => SyntaxError::unrecoverable(
                            input.to_string(),
                            "float".to_string(),
                            Some(format!("{}.{}", first_digits, last_digits)),
                            Some("valid floats can be written like 1.0 or 5.23".to_string()),
                        ),
                    }
                }
                _ => SyntaxError::expected(
                    input,
                    "float",
                    None,
                    Some("valid floats can be written like 1.0 or 5.23"),
                ),
            }
        }

        _ => SyntaxError::expected(
            input,
            "float",
            None,
            Some("valid floats can be written like 1.0 or 5.23"),
        ),
    }
}

fn parse_boolean(input: &str) -> IResult<&str, bool, SyntaxError> {
    match tag::<&str, &str, SyntaxError>("True")(input) {
        Ok((input, _)) => Ok((input, true)),
        Err(_) => match tag::<&str, &str, SyntaxError>("False")(input) {
            Ok((input, _)) => Ok((input, false)),
            Err(_) => SyntaxError::expected(input, "bool", None, None),
        },
    }
}

fn parse_none(input: &str) -> IResult<&str, (), SyntaxError> {
    match tag::<&str, &str, SyntaxError>("None")(input) {
        Ok((input, _)) => Ok((input, ())),
        Err(_) => match tag::<&str, &str, SyntaxError>("()")(input) {
            Ok((input, _)) => Ok((input, ())),
            Err(_) => SyntaxError::expected(input, "()", None, None),
        },
    }
}
fn parse_quote(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("'")(input)?;

    map(parse_expression_prec_four, |x| {
        Expression::Quote(Box::new(x))
    })(input)
}

fn parse_not(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("!")(input)?;

    map(parse_expression_prec_four, |x| {
        Expression::Apply(Box::new(Expression::Symbol("not".to_string())), vec![x])
    })(input)
}

fn parse_string(input: &str) -> IResult<&str, String, SyntaxError> {
    let old_input = input;
    let (input, _) = tag("\"")(input)?;

    let (input, _) = parse_string_inner(input)?;

    let (input, _) = alt((tag("\""), eof))(input)?;

    Ok((
        input,
        snailquote::unescape(&old_input[0..old_input.len() - input.len()]).unwrap(),
    ))
}

fn parse_string_inner(mut input: &str) -> IResult<&str, String, SyntaxError> {
    let old_input = input;

    loop {
        match input.chars().next() {
            Some('\"') | None => break,
            Some('\\') => {
                match parse_escape(input) {
                    Ok((i, _)) => input = i,
                    Err(_) => return SyntaxError::unrecoverable(
                        input,
                        "string",
                        Some("invalid escape"),
                        Some(
                            "escape codes can be one of: \\\", \\\\, \\/, \\b, \\f, \\n, \\r, \\t ",
                        ),
                    ),
                }
            }
            Some(ch) => {
                input = &input[ch.len_utf8()..];
            }
        }
    }

    let result = old_input[0..old_input.len() - input.len()].to_string();

    Ok((input, result))
}

fn parse_escape(input: &str) -> IResult<&str, (), SyntaxError> {
    let (input, _) = tag("\\")(input)?;
    let (input, _) = alt((
        tag("\""),
        tag("\\"),
        tag("/"),
        tag("b"),
        tag("f"),
        tag("n"),
        tag("r"),
        tag("t"),
        map(count(one_of(ASCII_HEX_DIGIT), 4), |_| ""),
    ))(input)?;
    Ok((input, ()))
}

fn parse_assign(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("let")(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, symbol) = match parse_symbol(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "symbol",
                None,
                Some("try using a valid symbol such as `x` in `let x = 5`"),
            )
        }
    };
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = match tag::<&str, &str, SyntaxError>("=")(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`=`",
                None,
                Some("let expressions must use an `=` sign"),
            )
        }
    };
    let (input, expr) = parse_expression(input)?;
    Ok((input, Expression::Assign(symbol, Box::new(expr))))
}

fn parse_group(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, expr) = parse_expression(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = match tag::<&str, &str, SyntaxError>(")")(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`)`",
                Some("no matching parentheses"),
                Some("try adding a matching `)` to the end of your expression"),
            )
        }
    };

    Ok((input, Expression::Group(Box::new(expr))))
}

fn parse_list(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, expr_list) = separated_list0(tag(","), parse_expression)(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = match tag::<&str, &str, SyntaxError>("]")(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`]`",
                Some("no matching `]`"),
                Some("try adding a matching `]` to the end of your list"),
            )
        }
    };

    Ok((input, Expression::List(expr_list)))
}

fn parse_map(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, expr_map) = separated_list0(
        tag(","),
        separated_pair(
            delimited(try_parse_ws, parse_symbol, try_parse_ws),
            tag(":"),
            parse_expression,
        ),
    )(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = match tag::<&str, &str, SyntaxError>("}")(input) {
        Ok(result) => result,
        Err(_) if expr_map.is_empty() => {
            return SyntaxError::expected(
                input,
                "`}`",
                Some("no matching `}`"),
                Some("try adding a matching `}` to the end of your map"),
            )
        }
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`}`",
                Some("no matching `}`"),
                Some("try adding a matching `}` to the end of your map"),
            )
        }
    };

    Ok((
        input,
        Expression::Map(
            expr_map
                .into_iter()
                .collect::<BTreeMap<String, Expression>>(),
        ),
    ))
}

fn parse_for_loop(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("for")(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, symbol) = parse_symbol(input)?;
    let (input, _) = try_parse_ws(input)?;

    let (input, _) = match tag::<&str, &str, SyntaxError>("in")(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`in` keyword",
                None,
                Some("try writing a for loop in the format of `for i in 0 to 10 {}`"),
            )
        }
    };

    let (input, list) = match parse_expression_prec_five(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "iterable expression",
                None,
                Some("try adding an iterable expression such as `0 to 10` to your for loop"),
            )
        }
    };
    let (input, _) = try_parse_ws(input)?;
    let (input, body) =
        match parse_block(input) {
            Ok(result) => result,
            Err(_) => return SyntaxError::unrecoverable(
                input,
                "block",
                None,
                Some(
                    "try adding a block, such as `{ print \"hello!\"}` to the end of your for loop",
                ),
            ),
        };

    Ok((
        input,
        Expression::For(symbol, Box::new(list), Box::new(body)),
    ))
}

fn parse_if(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("if")(input)?;

    let (input, cond) = match parse_expression_prec_six(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "condition expression",
                None,
                Some("try adding a condition expression to your if statement"),
            )
        }
    };

    let (input, t) = match alt((parse_block, parse_expression_prec_four))(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "then expression",
                None,
                Some("try adding an expression to the end of your if statement"),
            )
        }
    };

    let (input, maybe_e) = opt(preceded(
        pair(try_parse_ws, tag("else")),
        alt((parse_block, parse_expression_prec_four)),
    ))(input)?;
    Ok((
        input,
        Expression::If(
            Box::new(cond),
            Box::new(t),
            Box::new(match maybe_e {
                Some(expr) => expr,
                None => Expression::None,
            }),
        ),
    ))
}

fn parse_callable(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, arg) = parse_symbol(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, fn_type) = alt((tag("->"), tag("~>")))(input)?;
    let (input, body) = match parse_expression(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "an expression",
                None,
                Some("try writing a lambda or macro like `x -> x + 1` or `y ~> let x = y`"),
            )
        }
    };
    Ok((
        input,
        match fn_type {
            "->" => Expression::Lambda(arg, Box::new(body), Environment::new()),
            "~>" => Expression::Macro(arg, Box::new(body)),
            _ => unreachable!(),
        },
    ))
}

fn parse_block(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("{")(input)?;

    let (input, expr) = parse_script(input, false)?;

    let (input, _) = try_parse_ws(input)?;
    let (input, _) = match tag::<&str, &str, SyntaxError>("}")(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "`}`",
                Some("no matching `}`"),
                Some("try adding a matching `}` to the end of your block"),
            )
        }
    };

    Ok((input, expr))
}

fn parse_apply(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, f) = parse_expression_prec_three(input)?;
    let (input, args) = many1(parse_expression_prec_five)(input)?;

    Ok((input, Expression::Apply(Box::new(f), args)))
}

fn parse_expression(input: &str) -> IResult<&str, Expression, SyntaxError> {
    alt((
        parse_for_loop,
        parse_if,
        parse_assign,
        parse_callable,
        parse_apply,
        parse_expression_prec_six,
    ))(input)
}

fn parse_expression_prec_six(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let expr_parser = parse_expression_prec_five;

    let (input, head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(
        delimited(parse_ws, alt((tag("&&"), tag("||"))), parse_ws),
        expr_parser,
    ))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    let (mut op, mut result) = list.pop().unwrap();
    while !list.is_empty() {
        if let Some((next_op, item)) = list.pop() {
            let op_fun = Expression::Symbol(
                match op {
                    "&&" => "and",
                    "||" => "or",
                    _ => unreachable!(),
                }
                .to_string(),
            );

            result = Expression::Group(Box::new(Expression::Apply(
                Box::new(op_fun.clone()),
                vec![item, result],
            )));

            op = next_op;
        }
    }

    let op_fun = Expression::Symbol(
        match op {
            "&&" => "and",
            "||" => "or",
            _ => unreachable!(),
        }
        .to_string(),
    );

    Ok((
        input,
        Expression::Apply(Box::new(op_fun.clone()), vec![head, result]),
    ))
}

fn parse_range(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, from) = parse_expression_prec_four(input)?;
    let (input, _) = try_parse_ws(input)?;
    let (input, _) = tag("to")(input)?;

    let (input, to) = match parse_expression_prec_four(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "a valid range expression",
                None,
                Some("try writing an expression like `0 to 10`"),
            )
        }
    };

    Ok((
        input,
        Expression::Apply(
            Box::new(Expression::Symbol("range".to_string())),
            vec![from, to],
        ),
    ))
}

fn parse_expression_prec_five(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let expr_parser = parse_expression_prec_four;

    if let Ok(result) = parse_range(input) {
        return Ok(result);
    }

    if let Ok(result) = parse_not(input) {
        return Ok(result);
    }

    let (input, head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(
        delimited(
            parse_ws,
            alt((
                tag("=="),
                tag("!="),
                tag(">="),
                tag("<="),
                tag(">"),
                tag("<"),
            )),
            parse_ws,
        ),
        expr_parser,
    ))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    let (mut op, mut result) = list.pop().unwrap();
    while !list.is_empty() {
        if let Some((next_op, item)) = list.pop() {
            let op_fun = Expression::Symbol(
                match op {
                    "==" => "eq",
                    "!=" => "neq",
                    ">=" => "gte",
                    "<=" => "lte",
                    ">" => "gt",
                    "<" => "lt",
                    _ => unreachable!(),
                }
                .to_string(),
            );

            result = Expression::Group(Box::new(Expression::Apply(
                Box::new(op_fun.clone()),
                vec![item, result],
            )));

            op = next_op;
        }
    }

    let op_fun = Expression::Symbol(
        match op {
            "==" => "eq",
            "!=" => "neq",
            ">=" => "gte",
            "<=" => "lte",
            ">" => "gt",
            "<" => "lt",
            _ => unreachable!(),
        }
        .to_string(),
    );

    Ok((
        input,
        Expression::Apply(Box::new(op_fun.clone()), vec![head, result]),
    ))
}

fn parse_expression_prec_four(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let expr_parser = parse_expression_prec_three;

    let (input, head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(
        delimited(parse_ws, alt((tag("+"), tag("-"))), parse_ws),
        expr_parser,
    ))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    let (mut op, mut result) = list.pop().unwrap();
    while !list.is_empty() {
        if let Some((next_op, item)) = list.pop() {
            let op_fun = Expression::Symbol(
                match op {
                    "+" => "add",
                    "-" => "sub",
                    _ => unreachable!(),
                }
                .to_string(),
            );

            result = Expression::Group(Box::new(Expression::Apply(
                Box::new(op_fun.clone()),
                vec![item, result],
            )));

            op = next_op;
        }
    }

    let op_fun = Expression::Symbol(
        match op {
            "+" => "add",
            "-" => "sub",
            _ => unreachable!(),
        }
        .to_string(),
    );

    Ok((
        input,
        Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, result],
        ))),
    ))
}

fn parse_expression_prec_three(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let expr_parser = parse_expression_prec_two;

    let (input, head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(
        delimited(parse_ws, alt((tag("*"), tag("//"), tag("%"))), parse_ws),
        expr_parser,
    ))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    let (mut op, mut result) = list.pop().unwrap();
    while !list.is_empty() {
        if let Some((next_op, item)) = list.pop() {
            let op_fun = Expression::Symbol(
                match op {
                    "*" => "mul",
                    "//" => "div",
                    "%" => "rem",
                    _ => unreachable!(),
                }
                .to_string(),
            );

            result = Expression::Group(Box::new(Expression::Apply(
                Box::new(op_fun.clone()),
                vec![item, result],
            )));

            op = next_op;
        }
    }

    let op_fun = Expression::Symbol(
        match op {
            "*" => "mul",
            "//" => "div",
            "%" => "rem",
            _ => unreachable!(),
        }
        .to_string(),
    );

    Ok((
        input,
        Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, result],
        ))),
    ))
}

fn parse_expression_prec_two(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, head) = parse_expression_prec_one(input)?;
    let (input, args) = many0(preceded(
        tag("@"),
        alt((
            map(parse_symbol, |name| Expression::Symbol(name)),
            map(parse_integer, |n| Expression::Integer(n)),
        )),
    ))(input)?;

    if args.is_empty() {
        return Ok((input, head));
    }

    let mut result = vec![head];
    result.extend(args.into_iter());

    Ok((
        input,
        Expression::Apply(Box::new(Expression::Symbol("index".to_string())), result),
    ))
}

fn parse_expression_prec_one(input: &str) -> IResult<&str, Expression, SyntaxError> {
    let (input, _) = try_parse_ws(input)?;
    alt((
        parse_group,
        parse_quote,
        parse_map,
        parse_block,
        parse_list,
        map(parse_none, |_| Expression::None),
        map(parse_float, |n| Expression::Float(n)),
        map(parse_integer, |n| Expression::Integer(n)),
        map(parse_boolean, |b| Expression::Boolean(b)),
        map(parse_string, |x| Expression::String(x)),
        map(parse_symbol, |x| Expression::Symbol(x)),
    ))(input)
}
