use detached_str::StrSlice;
use nom::{
    branch::alt,
    combinator::{eof, map, opt},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, separated_list0},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use std::collections::BTreeMap;

use crate::{
    tokens::{Input, Tokens},
    Diagnostic, Environment, Expression, Int, Token, TokenKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxError {
    TokenizationErrors(Box<[Diagnostic]>),
    Expected {
        input: StrSlice,
        expected: &'static str,
        found: Option<String>,
        hint: Option<&'static str>,
    },
    ExpectedChar {
        expected: char,
        at: Option<StrSlice>,
    },
    NomError {
        kind: nom::error::ErrorKind,
        at: Option<StrSlice>,
        cause: Option<Box<SyntaxError>>,
    },
    InternalError,
}

impl SyntaxError {
    pub(crate) fn unrecoverable(
        input: StrSlice,
        expected: &'static str,
        found: Option<String>,
        hint: Option<&'static str>,
    ) -> nom::Err<SyntaxError> {
        nom::Err::Failure(Self::Expected {
            input,
            expected,
            found,
            hint,
        })
    }

    pub(crate) fn expected(
        input: StrSlice,
        expected: &'static str,
        found: Option<String>,
        hint: Option<&'static str>,
    ) -> nom::Err<SyntaxError> {
        nom::Err::Error(Self::Expected {
            input,
            expected,
            found,
            hint,
        })
    }
}

impl ParseError<Tokens<'_>> for SyntaxError {
    fn from_error_kind(input: Tokens<'_>, kind: ErrorKind) -> Self {
        Self::NomError {
            kind,
            at: input.get(0).map(|t| t.range),
            cause: None,
        }
    }

    fn append(input: Tokens<'_>, kind: ErrorKind, other: Self) -> Self {
        Self::NomError {
            kind,
            at: input.get(0).map(|t| t.range),
            cause: Some(Box::new(other)),
        }
    }

    fn from_char(input: Tokens<'_>, expected: char) -> Self {
        Self::ExpectedChar {
            expected,
            at: input.get(0).map(|t| t.range),
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
fn kind(kind: TokenKind) -> impl Fn(Tokens<'_>) -> IResult<Tokens<'_>, StrSlice, SyntaxError> {
    move |input: Tokens<'_>| match input.first() {
        Some(&token) if token.kind == kind => Ok((input.skip_n(1), token.range)),
        _ => Err(nom::Err::Error(SyntaxError::InternalError)),
    }
}

#[inline]
fn text<'a>(text: &'a str) -> impl Fn(Tokens<'a>) -> IResult<Tokens<'a>, Token, SyntaxError> {
    move |input: Tokens<'a>| match input.first() {
        Some(&token) if token.text(input) == text => Ok((input.skip_n(1), token)),
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
    let str = input.into();
    let tokenization_input = Input::new(&str);
    let (mut token_vec, mut diagnostics) = super::parse_tokens(tokenization_input);

    diagnostics.retain(|d| d != &Diagnostic::Valid);
    if !diagnostics.is_empty() {
        return Err(nom::Err::Failure(SyntaxError::TokenizationErrors(
            diagnostics.into_boxed_slice(),
        )));
    }

    let tokens = Tokens {
        str: &str,
        slice: token_vec.as_slice(),
    };

    for window in tokens.slice.windows(2) {
        let (a, b) = (window[0], window[1]);
        if is_symbol_like(a.kind)
            && is_symbol_like(b.kind)
            && a.text(tokens) != "@"
            && b.text(tokens) != "@"
        {
            return Err(nom::Err::Failure(SyntaxError::Expected {
                input: a.range.join(b.range),
                expected: "whitespace",
                found: Some(b.text(tokens).to_string()),
                hint: None,
            }));
        }
    }

    // remove whitespace
    token_vec.retain(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::Comment));

    let (_, expr) = parse_script_tokens(
        Tokens {
            str: &str,
            slice: token_vec.as_slice(),
        },
        true,
    )?;
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
        (_, Err(_)) => Err(SyntaxError::expected(
            input.get_str_slice(),
            ";",
            None,
            Some("try adding a semicolon"),
        )),
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
        input = eof(input)
            .map_err(|_: nom::Err<SyntaxError>| {
                SyntaxError::expected(input.get_str_slice(), "end of input", None, None)
            })?
            .0;
    }

    Ok((input, Expression::Do(exprs)))
}

#[inline]
pub fn no_terminating_punctuation(input: Tokens<'_>) -> IResult<Tokens<'_>, (), SyntaxError> {
    if let Some(token) = input.first() {
        if token.kind == TokenKind::Punctuation
            && matches!(token.text(input), ";" | "," | "=" | "]" | ")" | "}" | "|")
        {
            Err(SyntaxError::expected(
                input.get_str_slice(),
                "a non-terminating punctuation",
                None,
                None,
            ))
        } else {
            Ok((input, ()))
        }
    } else {
        Ok((input, ()))
    }
}

#[inline]
fn parse_symbol(input: Tokens<'_>) -> IResult<Tokens<'_>, String, SyntaxError> {
    map(kind(TokenKind::Symbol), |t| t.to_str(input.str).to_string())(input)
}

fn parse_integer(input: Tokens<'_>) -> IResult<Tokens<'_>, Int, SyntaxError> {
    let (input, num) = kind(TokenKind::IntegerLiteral)(input)?;
    let num = num.to_str(input.str).parse::<Int>().map_err(|e| {
        SyntaxError::unrecoverable(
            num,
            "integer",
            Some(format!("error: {}", e.to_string())),
            None,
        )
    })?;
    Ok((input, num))
}

fn parse_float(input: Tokens<'_>) -> IResult<Tokens<'_>, f64, SyntaxError> {
    let (input, num) = kind(TokenKind::FloatLiteral)(input)?;
    let num = num.to_str(input.str).parse::<f64>().map_err(|e| {
        SyntaxError::unrecoverable(
            num,
            "float",
            Some(format!("error: {}", e.to_string())),
            Some("valid floats can be written like 1.0 or 5.23"),
        )
    })?;
    Ok((input, num))
}

#[inline]
fn parse_boolean(input: Tokens<'_>) -> IResult<Tokens<'_>, bool, SyntaxError> {
    map(kind(TokenKind::BooleanLiteral), |s| {
        s.to_str(input.str) == "True"
    })(input)
}

fn parse_none(input: Tokens<'_>) -> IResult<Tokens<'_>, (), SyntaxError> {
    if let Ok((input, _)) = text("None")(input) {
        Ok((input, ()))
    } else if input.len() >= 2 && input[0].text(input) == "(" && input[1].text(input) == ")" {
        Ok((input.skip_n(2), ()))
    } else {
        Err(SyntaxError::expected(
            input.get_str_slice(),
            "None or ()",
            None,
            None,
        ))
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
        Expression::Apply(Box::new(Expression::Symbol("!".to_string())), vec![x])
    })(input)
}

#[inline]
fn parse_string(input: Tokens<'_>) -> IResult<Tokens<'_>, String, SyntaxError> {
    let (input, string) = kind(TokenKind::StringLiteral)(input)?;
    Ok((
        input,
        snailquote::unescape(string.to_str(input.str)).unwrap(),
    ))
}

fn parse_assign(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("let")(input)?;

    let (input, symbol) = alt((parse_symbol, parse_operator))(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "symbol",
            None,
            Some("try using a valid symbol such as `x` in `let x = 5`"),
        )
    })?;
    let (input, _) = text("=")(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "`=`",
            None,
            Some("let expressions must use an `=` sign"),
        )
    })?;
    let (input, expr) = parse_expression(input)?;
    Ok((input, Expression::Assign(symbol, Box::new(expr))))
}

fn parse_group(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("(")(input)?;
    let (input, expr) = parse_expression(input)?;

    let (input, _) = alt((map(text(")"), |_| ()), empty))(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "`)`",
            Some("no matching parentheses".into()),
            Some("try adding a matching `)` to the end of your expression"),
        )
    })?;

    Ok((input, Expression::Group(Box::new(expr))))
}

fn parse_list(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("[")(input)?;
    let (input, expr_list) = separated_list0(text(","), parse_expression)(input)?;
    let (input, _) = text("]")(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "`]`",
            Some("no matching `]`".into()),
            Some("try adding a matching `]` to the end of your list"),
        )
    })?;

    Ok((input, Expression::List(expr_list)))
}

fn parse_map(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("{")(input)?;
    let (input, expr_map) = separated_list0(
        text(","),
        separated_pair(
            alt((parse_symbol, parse_operator)),
            text("="),
            parse_expression,
        ),
    )(input)?;
    let (input, _) = text("}")(input).map_err(|_| {
        if expr_map.is_empty() {
            SyntaxError::expected(
                input.get_str_slice(),
                "`}`",
                Some("no matching `}`".into()),
                Some("try adding a matching `}` to the end of your map"),
            )
        } else {
            SyntaxError::unrecoverable(
                input.get_str_slice(),
                "`}`",
                Some("no matching `}`".into()),
                Some("try adding a matching `}` to the end of your map"),
            )
        }
    })?;

    let expr_map = expr_map
        .into_iter()
        .collect::<BTreeMap<String, Expression>>();

    Ok((input, Expression::Map(expr_map)))
}

fn parse_for_loop(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("for")(input)?;
    let (input, symbol) = parse_symbol(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "symbol",
            None,
            Some("try using a valid symbol such as `x` in `for x in 0 to 10 {}`"),
        )
    })?;

    let (input, _) = text("in")(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "`in` keyword",
            None,
            Some("try writing a for loop in the format of `for i in 0 to 10 {}`"),
        )
    })?;

    let (input, list) = alt((parse_range, parse_expression_prec_four))(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "iterable expression",
            None,
            Some("try adding an iterable expression such as `0 to 10` to your for loop"),
        )
    })?;
    let (input, body) = parse_block(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "block",
            None,
            Some("try adding a block, such as `{ print \"hello!\"}` to the end of your for loop"),
        )
    })?;

    Ok((
        input,
        Expression::For(symbol, Box::new(list), Box::new(body)),
    ))
}

fn parse_if(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("if")(input)?;

    let (input, cond) = parse_expression_prec_six(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "condition expression",
            None,
            Some("try adding a condition expression to your if statement"),
        )
    })?;

    let (input, t) = parse_expression_prec_four(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "then expression",
            None,
            Some("try adding an expression to the end of your if statement"),
        )
    })?;

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
    let (input, body) = parse_expression(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "an expression",
            None,
            Some("try writing a lambda or macro like `x -> x + 1` or `y ~> let x = y`"),
        )
    })?;
    Ok((
        input,
        match fn_type.text(input) {
            "->" => Expression::Lambda(arg, Box::new(body), Environment::new()),
            "~>" => Expression::Macro(arg, Box::new(body)),
            _ => unreachable!(),
        },
    ))
}

fn parse_block(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, _) = text("{")(input)?;
    let (input, expr) = parse_script_tokens(input, false)?;

    let (input, _) = text("}")(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "`}`",
            Some("no matching `}`".into()),
            Some("try adding a matching `}` to the end of your block"),
        )
    })?;
    Ok((input, expr))
}

fn parse_apply(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, f) = alt((parse_expression_prec_two, parse_operator_as_symbol))(input)?;
    let (input, args) = many1(parse_expression_prec_five)(input)?;

    Ok((input, Expression::Apply(Box::new(f), args)))
}

fn parse_apply_operator(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    let (input, f) = parse_operator_as_symbol(input)?;
    let (input, args) = many0(parse_expression_prec_five)(input)?;

    if args.is_empty() {
        Ok((input, f))
    } else {
        Ok((input, Expression::Apply(Box::new(f), args)))
    }
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
        args.push(match op.text(input) {
            "|" => item,
            ">>" => Expression::Apply(Box::new(Expression::Symbol(">>".to_string())), vec![item]),
            _ => unreachable!(),
        })
    }

    Ok((
        input,
        Expression::Group(Box::new(Expression::Apply(
            Box::new(Expression::Symbol("|".to_string())),
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
        parse_apply_operator,
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
        let op_fun = Expression::Symbol(op.text(input).to_string());

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

    let (input, to) = parse_expression_prec_four(input).map_err(|_| {
        SyntaxError::unrecoverable(
            input.get_str_slice(),
            "a valid range expression",
            None,
            Some("try writing an expression like `0 to 10`"),
        )
    })?;

    Ok((
        input,
        Expression::Apply(
            Box::new(Expression::Symbol("to".to_string())),
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
            let (input, to) = parse_expression_prec_four(input).map_err(|_| {
                SyntaxError::unrecoverable(
                    input.get_str_slice(),
                    "a valid range expression",
                    None,
                    Some("try writing an expression like `0 to 10`"),
                )
            })?;

            return Ok((
                input,
                Expression::Apply(
                    Box::new(Expression::Symbol("to".to_string())),
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
        let op_fun = Expression::Symbol(op.text(input).to_string());

        head = Expression::Group(Box::new(Expression::Apply(
            Box::new(op_fun.clone()),
            vec![head, item],
        )));
    }

    Ok((input, head))
}

fn parse_operator(input: Tokens<'_>) -> IResult<Tokens<'_>, String, SyntaxError> {
    map(kind(TokenKind::Operator), |t| {
        t.to_str(input.str).to_string()
    })(input)
}

fn parse_operator_as_symbol(input: Tokens<'_>) -> IResult<Tokens<'_>, Expression, SyntaxError> {
    map(parse_operator, Expression::Symbol)(input)
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
        let op_fun = Expression::Symbol(op.text(input).to_string());

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
        let op_fun = Expression::Symbol(op.text(input).to_string());

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
    let (input, args) = many0(preceded(
        text("@"),
        alt((parse_expression_prec_one, parse_operator_as_symbol)),
    ))(input)?;

    if args.is_empty() {
        return Ok((input, head));
    }

    let mut result = vec![head];
    result.extend(args.into_iter());

    Ok((
        input,
        Expression::Apply(Box::new(Expression::Symbol("@".to_string())), result),
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
