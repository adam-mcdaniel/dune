use nom::{
    branch::alt,
    combinator::{eof, map, opt},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, separated_list0},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use std::collections::BTreeMap;

use super::{tokens::Tokens, Environment, Expression, Int, Token, TokenKind};

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
    pub(crate) fn unrecoverable<A, B, T1, T2>(
        input: T1,
        expected: T2,
        found: Option<T2>,
        hint: Option<T2>,
    ) -> IResult<A, B, Self>
    where
        T1: ToString,
        T2: ToString,
    {
        Err(nom::Err::Failure(Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: found.map(|x| x.to_string()),
            hint: hint.map(|x| x.to_string()),
        }))
    }

    pub(crate) fn expected<A, B, T1, T2>(
        input: T1,
        expected: T2,
        found: Option<T2>,
        hint: Option<T2>,
    ) -> IResult<A, B, Self>
    where
        T1: ToString,
        T2: ToString,
    {
        Err(nom::Err::Error(Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: found.map(|x| x.to_string()),
            hint: hint.map(|x| x.to_string()),
        }))
    }

    pub(crate) fn expected_err<T1, T2>(
        input: T1,
        expected: T2,
        found: Option<T2>,
        hint: Option<T2>,
    ) -> nom::Err<Self>
    where
        T1: ToString,
        T2: ToString,
    {
        nom::Err::Error(Self::Expected {
            input: input.to_string(),
            expected: expected.to_string(),
            found: found.map(|x| x.to_string()),
            hint: hint.map(|x| x.to_string()),
        })
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
            _ => self,
        }
    }
}

#[inline]
fn kind(kind: TokenKind) -> impl Fn(Tokens<'_>) -> IResult<Tokens<'_>, &str, SyntaxError> {
    move |input: Tokens<'_>| match input.first() {
        Some(&token) if token.kind == kind => Ok((input.skip_n(1), token.text)),
        _ => Err(nom::Err::Error(SyntaxError::InternalError)),
    }
}

#[inline]
fn text<'a>(text: &'a str) -> impl Fn(Tokens<'a>) -> IResult<Tokens<'a>, Token<'a>, SyntaxError> {
    move |input: Tokens<'a>| match input.first() {
        Some(&token) if token.text == text => Ok((input.skip_n(1), token)),
        _ => Err(nom::Err::Error(SyntaxError::InternalError)),
    }
}

#[inline]
fn empty(input: Tokens<'_>) -> IResult<Tokens<'_>, (), SyntaxError> {
    if input.is_empty() {
        Ok((input, ()))
    } else {
        Err(nom::Err::Error(SyntaxError::InternalError))
    }
}

pub fn parse_script(input: &str) -> Result<Expression, nom::Err<SyntaxError>> {
    let (_, mut tokens) = super::parse_tokens(input)?;

    for &token in &tokens {
        if token.kind == TokenKind::Other {
            return Err(nom::Err::Failure(SyntaxError::CustomError(format!(
                "Illegal character `{}`",
                token.text
            ))));
        }
    }

    for window in tokens.windows(2) {
        let (a, b) = (window[0], window[1]);
        if is_symbol_like(a.kind) && is_symbol_like(b.kind) {
            return Err(nom::Err::Failure(SyntaxError::Expected {
                input: input.to_string(),
                expected: "whitespace".to_string(),
                found: Some(b.text.to_string()),
                hint: None,
            }));
        }
    }

    // remove whitespace
    tokens.retain(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::Comment));

    let (_, expr) = parse_script_tokens(Tokens(tokens.as_slice()), true)?;
    Ok(expr)
}

#[inline]
fn is_symbol_like(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Symbol
            | TokenKind::Keyword
            | TokenKind::Operator
            | TokenKind::BooleanLiteral
            | TokenKind::FloatLiteral
            | TokenKind::IntegerLiteral
    )
}

fn parse_statement(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, expr) = parse_expression(input)?;
    match (&expr, text(";")(input)) {
        (Expression::For(_, _, _), Ok((input, _))) => Ok((input, expr)),
        (Expression::For(_, _, _), Err(_)) => Ok((input, expr)),
        (Expression::If(_, _, _), Ok((input, _))) => Ok((input, expr)),
        (Expression::If(_, _, _), Err(_)) => Ok((input, expr)),

        (_, Ok((input, _))) => Ok((input, expr)),
        (_, Err(_)) => SyntaxError::expected(
            input,
            ";",
            input.first().map(|x| x.text),
            Some("try adding a semicolon"),
        ),
    }
}

fn parse_script_tokens(
    input: Tokens<'_>,
    require_eof: bool,
) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    // println!("hmm {}", input);
    let (input, mut exprs) = many0(parse_statement)(input)?;

    let (mut input, last) = opt(terminated(parse_expression, opt(text(";"))))(input)?;

    if let Some(expr) = last {
        exprs.push(expr);
    }

    if require_eof {
        input = eof(input)?.0;
    }

    Ok((input, Expression::Do(exprs)))
}

#[inline]
pub fn no_terminating_punctuation(input: Tokens<'_>) -> IResult<Tokens<'_>, (), SyntaxError> {
    if let Some(token) = input.first() {
        if token.kind == TokenKind::Punctuation
            && matches!(token.text, ";" | "," | "=" | "]" | ")" | "}" | "|")
        {
            SyntaxError::expected(input, "a non-terminating punctuation", None, None)
        } else {
            Ok((input, ()))
        }
    } else {
        Ok((input, ()))
    }
}

#[inline]
fn parse_symbol(input: Tokens<'_>) -> IResult<Tokens<'_>, String, SyntaxError> {
    map(kind(TokenKind::Symbol), |t| t.to_string())(input)
}

fn parse_integer(input: Tokens<'_>) -> IResult<Tokens<'_>, Int, SyntaxError> {
    let (new_input, num) = kind(TokenKind::IntegerLiteral)(input)?;
    let num = num
        .parse::<Int>()
        .map_err(|_| SyntaxError::expected_err(input, "integer", Some(num), None))?;
    Ok((new_input, num))
}

fn parse_float(input: Tokens<'_>) -> IResult<Tokens<'_>, f64, SyntaxError> {
    let (new_input, num) = kind(TokenKind::FloatLiteral)(input)?;
    let num = num.parse::<f64>().map_err(|_| {
        SyntaxError::expected_err(
            input,
            "float",
            Some(num),
            Some("valid floats can be written like 1.0 or 5.23"),
        )
    })?;
    Ok((new_input, num))
}

#[inline]
fn parse_boolean(input: Tokens<'_>) -> IResult<Tokens<'_>, bool, SyntaxError> {
    map(kind(TokenKind::BooleanLiteral), |s| s == "True")(input)
}

fn parse_none(input: Tokens<'_>) -> IResult<Tokens<'_>, (), SyntaxError> {
    if let Ok((input, _)) = text("None")(input) {
        Ok((input, ()))
    } else if input.0.len() >= 2 && input[0].text == "(" && input[1].text == ")" {
        Ok((input.skip_n(2), ()))
    } else {
        SyntaxError::expected(input, "None or ()", None, None)
    }
}

fn parse_quote(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("'")(input)?;

    map(parse_expression_prec_two, |x| {
        Expression::Quote(Box::new(x))
    })(input)
}

fn parse_not(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("!")(input)?;

    map(parse_expression_prec_four, |x| {
        Expression::Apply(Box::new(Expression::Symbol("__not__".to_string())), vec![x])
    })(input)
}

#[inline]
fn parse_string(input: Tokens<'_>) -> IResult<Tokens<'_>, String, SyntaxError> {
    let (input, string) = kind(TokenKind::StringLiteral)(input)?;
    Ok((input, snailquote::unescape(string).unwrap()))
}

fn parse_assign(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("let")(input)?;

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
    let (input, _) = match text("=")(input) {
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

fn parse_group(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("(")(input)?;
    let (input, expr) = parse_expression(input)?;

    let (input, _) = match alt((map(text(")"), |_| ()), empty))(input) {
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

fn parse_list(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("[")(input)?;
    let (input, expr_list) = separated_list0(text(","), parse_expression)(input)?;
    let (input, _) = match text("]")(input) {
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

fn parse_map(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("{")(input)?;
    let (input, expr_map) = separated_list0(
        text(","),
        separated_pair(parse_symbol, text("="), parse_expression),
    )(input)?;
    let (input, _) = match text("}")(input) {
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

fn parse_for_loop(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("for")(input)?;
    let (input, symbol) = match parse_symbol(input) {
        Ok(result) => result,
        Err(_) => {
            return SyntaxError::unrecoverable(
                input,
                "symbol",
                None,
                Some("try using a valid symbol such as `x` in `for x in 0 to 10 {}`"),
            )
        }
    };

    let (input, _) = match text("in")(input) {
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

    let (input, list) = match alt((parse_range, parse_expression_prec_four))(input) {
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

fn parse_if(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("if")(input)?;

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

    let (input, t) = match parse_expression_prec_four(input) {
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
        text("else"),
        alt((parse_if, parse_expression_prec_four)),
    ))(input)?;

    let result = Expression::If(
        Box::new(cond),
        Box::new(t),
        Box::new(match maybe_e {
            Some(expr) => Expression::Group(Box::new(expr)),
            None => Expression::None,
        }),
    );

    Ok((input, result))
}

fn parse_callable(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, arg) = parse_symbol(input)?;
    let (input, fn_type) = alt((text("->"), text("~>")))(input)?;
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
        match fn_type.text {
            "->" => Expression::Lambda(arg, Box::new(body), Environment::new()),
            "~>" => Expression::Macro(arg, Box::new(body)),
            _ => unreachable!(),
        },
    ))
}

fn parse_block(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("{")(input)?;
    let (input, expr) = parse_script_tokens(input, false)?;

    if let Ok((input, _)) = text("}")(input) {
        Ok((input, expr))
    } else {
        SyntaxError::unrecoverable(
            input,
            "`}`",
            Some("no matching `}`"),
            Some("try adding a matching `}` to the end of your block"),
        )
    }
}

#[inline]
fn parse_apply(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, f) = parse_expression_prec_two(input)?;
    let (input, args) = many1(parse_expression_prec_five)(input)?;

    Ok((input, Expression::Apply(Box::new(f), args)))
}

fn parse_expression(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;

    let expr_parser = parse_expression_prec_seven;

    let (input, head) = expr_parser(input)?;

    let (input, list) = many0(pair(alt((text("|"), text(">>"))), expr_parser))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    let mut args = vec![head];
    for (op, item) in list {
        args.push(match op.text {
            "|" => item,
            ">>" => Expression::Apply(
                Box::new(Expression::Symbol("redirect-out".to_string())),
                vec![item],
            ),
            _ => unreachable!(),
        })
    }

    Ok((
        input,
        Expression::Group(Box::new(Expression::Apply(
            Box::new(Expression::Symbol("__pipe__".to_string())),
            args,
        ))),
    ))
}

fn parse_expression_prec_seven(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    alt((
        parse_for_loop,
        parse_if,
        parse_assign,
        parse_callable,
        parse_apply,
        parse_expression_prec_six,
    ))(input)
}

fn parse_expression_prec_six(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    let expr_parser = parse_expression_prec_five;

    let (input, mut head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(alt((text("&&"), text("||"))), expr_parser))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    list.reverse();

    while let Some((op, item)) = list.pop() {
        let op_fun = Expression::Symbol(
            match op.text {
                "&&" => "__and__",
                "||" => "__or__",
                _ => unreachable!(),
            }
            .to_string(),
        );

        head = Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, item],
        )));
    }

    Ok((input, head))
}

fn parse_range(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    let (input, from) = parse_expression_prec_four(input)?;
    let (input, _) = text("to")(input)?;

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

fn parse_expression_prec_five(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    let expr_parser = parse_expression_prec_four;

    let (input, mut head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(
        alt((
            text("=="),
            text("!="),
            text(">="),
            text("<="),
            text(">"),
            text("<"),
        )),
        expr_parser,
    ))(input)?;

    if list.is_empty() {
        if let Ok((input, _)) = text("to")(input) {
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

            return Ok((
                input,
                Expression::Apply(
                    Box::new(Expression::Symbol("range".to_string())),
                    vec![head, to],
                ),
            ));
        } else if let Ok(result) = parse_not(input) {
            return Ok(result);
        } else {
            return Ok((input, head));
        }
    }

    list.reverse();

    while let Some((op, item)) = list.pop() {
        let op_fun = Expression::Symbol(
            match op.text {
                "==" => "__eq__",
                "!=" => "__neq__",
                ">=" => "__gte__",
                "<=" => "__lte__",
                ">" => "__gt__",
                "<" => "__lt__",
                _ => unreachable!(),
            }
            .to_string(),
        );

        head = Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, item],
        )));
    }

    Ok((input, head))
}

fn parse_expression_prec_four(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    let expr_parser = parse_expression_prec_three;

    let (input, mut head) = expr_parser(input)?;

    let (input, mut list) = many0(pair(alt((text("+"), text("-"))), expr_parser))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    list.reverse();

    while let Some((op, item)) = list.pop() {
        let op_fun = Expression::Symbol(
            match op.text {
                "+" => "__add__",
                "-" => "__sub__",
                _ => unreachable!(),
            }
            .to_string(),
        );

        head = Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, item],
        )));
    }

    Ok((input, head))
}

fn parse_expression_prec_three(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;
    let expr_parser = parse_expression_prec_two;

    let (input, mut head) = expr_parser(input)?;

    let (input, mut list) =
        many0(pair(alt((text("*"), text("//"), text("%"))), expr_parser))(input)?;

    if list.is_empty() {
        return Ok((input, head));
    }

    list.reverse();

    while let Some((op, item)) = list.pop() {
        let op_fun = Expression::Symbol(
            match op.text {
                "*" => "__mul__",
                "//" => "__div__",
                "%" => "__rem__",
                _ => unreachable!(),
            }
            .to_string(),
        );

        head = Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, item],
        )));
    }

    Ok((input, head))
}

fn parse_expression_prec_two(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    no_terminating_punctuation(input)?;

    let (input, head) = parse_expression_prec_one(input)?;
    let (input, args) = many0(preceded(text("@"), parse_expression_prec_one))(input)?;

    if args.is_empty() {
        return Ok((input, head));
    }

    let mut result = vec![head];
    result.extend(args.into_iter());

    Ok((
        input,
        Expression::Apply(Box::new(Expression::Symbol("__idx__".to_string())), result),
    ))
}

fn parse_expression_prec_one(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    alt((
        parse_group,
        parse_quote,
        parse_map,
        parse_block,
        parse_list,
        map(parse_boolean, Expression::Boolean),
        map(parse_none, |_| Expression::None),
        map(parse_float, Expression::Float),
        map(parse_integer, Expression::Integer),
        map(parse_string, Expression::String),
        map(parse_symbol, Expression::Symbol),
    ))(input)
}
