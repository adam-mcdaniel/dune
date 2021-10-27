use super::{Environment, Error, Int};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    io::ErrorKind,
    ops::{Add, Div, Index, Mul, Rem, Sub},
    process::{Command, Stdio},
};

use terminal_size::{terminal_size, Width};

use prettytable::{
    cell,
    format::{LinePosition, LineSeparator},
    row, Table,
};

/// The maximum number of times that `eval` can recursively call itself
/// on a given expression before throwing an error. Even though
/// we could theoretically keep the tail call recursion optimization,
/// we don't really want to do this because it's better to halt.
const MAX_RECURSION_DEPTH: Option<usize> = Some(800);

impl From<Int> for Expression {
    fn from(x: Int) -> Self {
        Self::Integer(x)
    }
}

impl From<f64> for Expression {
    fn from(x: f64) -> Self {
        Self::Float(x)
    }
}

impl From<&str> for Expression {
    fn from(x: &str) -> Self {
        Self::String(x.to_string())
    }
}

impl From<String> for Expression {
    fn from(x: String) -> Self {
        Self::String(x)
    }
}

impl From<Vec<u8>> for Expression {
    fn from(x: Vec<u8>) -> Self {
        Self::Bytes(x)
    }
}

impl From<bool> for Expression {
    fn from(x: bool) -> Self {
        Self::Boolean(x)
    }
}

impl<T> From<BTreeMap<String, T>> for Expression
where
    T: Into<Self>,
{
    fn from(map: BTreeMap<String, T>) -> Self {
        Self::Map(
            map.into_iter()
                .map(|(name, item)| (name, item.into()))
                .collect::<BTreeMap<String, Self>>(),
        )
    }
}

impl<T> From<Vec<T>> for Expression
where
    T: Into<Self>,
{
    fn from(list: Vec<T>) -> Self {
        Self::List(
            list.into_iter()
                .map(|item| item.into())
                .collect::<Vec<Self>>(),
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum Expression {
    Group(Box<Self>),

    Symbol(String),
    // An integer literal
    Integer(Int),
    // A floating point number literal
    Float(f64),
    // A list of bytes
    Bytes(Vec<u8>),
    // A string literal
    String(String),
    // A boolean literal
    Boolean(bool),
    // A list of expressions
    List(Vec<Self>),
    // A map of expressions
    Map(BTreeMap<String, Self>),
    None,

    // Assign an expression to a variable
    Assign(String, Box<Self>),

    // Control flow
    For(String, Box<Self>, Box<Self>),

    // Control flow
    If(Box<Self>, Box<Self>, Box<Self>),

    // Apply a function or macro to an argument
    Apply(Box<Self>, Vec<Self>),

    Lambda(String, Box<Self>, Environment),
    Macro(String, Box<Self>),
    Do(Vec<Self>),
    // A builtin function.
    Builtin(Builtin),

    Quote(Box<Self>),
}

#[derive(Clone)]
pub struct Builtin {
    /// name of the function
    pub name: String,
    /// function pointer for executing the function
    pub body: fn(Vec<Expression>, &mut Environment) -> Result<Expression, Error>,
    /// help string
    pub help: String,
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin@{}", self.name)
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "builtin@{}", self.name)
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Default for Expression {
    fn default() -> Self {
        Expression::None
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quote(inner) => write!(f, "'{:?}", inner),
            Self::Group(inner) => write!(f, "({:?})", inner),
            Self::Symbol(name) => write!(f, "{}", name),
            Self::Integer(i) => write!(f, "{}", *i),
            Self::Float(n) => write!(f, "{}", *n),
            Self::Bytes(b) => write!(f, "b{:?}", b),
            Self::String(s) => write!(f, "{:?}", s),
            Self::Boolean(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Self::List(exprs) => write!(
                f,
                "[{}]",
                exprs
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),

            Self::Map(exprs) => write!(
                f,
                "{{{}}}",
                exprs
                    .iter()
                    .map(|(k, e)| format!("{}: {:?}", k, e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),

            Self::None => write!(f, "None"),
            Self::Lambda(param, body, _) => write!(f, "{} -> {:?}", param, body),
            Self::Macro(param, body) => write!(f, "{} ~> {:?}", param, body),
            Self::For(name, list, body) => write!(f, "for {} in {:?} {:?}", name, list, body),
            Self::Do(exprs) => write!(
                f,
                "{{ {} }}",
                exprs
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join("; ")
            ),

            Self::Assign(name, expr) => write!(f, "let {} = {:?}", name, expr),
            Self::If(cond, true_expr, false_expr) => {
                write!(f, "if {:?} {:?} else {:?}", cond, true_expr, false_expr)
            }
            Self::Apply(g, args) => write!(
                f,
                "{:?} {}",
                g,
                args.iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Builtin(builtin) => fmt::Debug::fmt(builtin, f),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quote(inner) => write!(f, "'{:?}", inner),
            Self::Group(inner) => write!(f, "({:?})", inner),
            Self::Symbol(name) => write!(f, "{}", name),
            Self::Integer(i) => write!(f, "{}", *i),
            Self::Float(n) => write!(f, "{}", *n),
            Self::Bytes(b) => write!(f, "b{:?}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Self::List(exprs) => write!(
                f,
                "[{}]",
                exprs
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Map(exprs) => {
                let mut t = Table::new();
                let fmt = t.get_format();
                fmt.padding(1, 1);
                fmt.borders('│');
                fmt.column_separator('│');
                fmt.separator(LinePosition::Top, LineSeparator::new('═', '╤', '╒', '╕'));
                fmt.separator(LinePosition::Title, LineSeparator::new('═', '╪', '╞', '╡'));
                fmt.separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'));
                fmt.separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'));

                let width = match terminal_size() {
                    Some((Width(width), _)) => Some(width as usize),
                    _ => None,
                };

                for (key, val) in exprs {
                    match &val {
                        Self::Builtin(Builtin { help, .. }) => {
                            t.add_row(row!(
                                key,
                                format!("{}", val),
                                match width {
                                    Some(w) => textwrap::fill(help, w / 6),
                                    None => help.to_string(),
                                }
                            ));
                        }
                        Self::Map(_) => {
                            t.add_row(row!(key, format!("{}", val)));
                        }
                        _ => {
                            let formatted = format!("{}", val);
                            t.add_row(row!(
                                key,
                                match width {
                                    Some(w) => textwrap::fill(&formatted, w / 5),
                                    None => formatted,
                                }
                            ));
                        }
                    }
                }
                write!(f, "{}", t)
            }

            Self::None => write!(f, "None"),
            Self::Lambda(param, body, _) => write!(f, "{} -> {:?}", param, body),
            Self::Macro(param, body) => write!(f, "{} ~> {:?}", param, body),
            Self::For(name, list, body) => write!(f, "for {} in {:?} {:?}", name, list, body),
            Self::Do(exprs) => write!(
                f,
                "{{ {} }}",
                exprs
                    .iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join("; ")
            ),

            Self::Assign(name, expr) => write!(f, "let {} = {:?}", name, expr),
            Self::If(cond, true_expr, false_expr) => {
                write!(f, "if {:?} {:?} else {:?}", cond, true_expr, false_expr)
            }
            Self::Apply(g, args) => write!(
                f,
                "{:?} {}",
                g,
                args.iter()
                    .map(|e| format!("{:?}", e))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Self::Builtin(builtin) => fmt::Display::fmt(builtin, f),
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Bytes(a), Self::Bytes(b)) => a.partial_cmp(b),
            (Self::List(a), Self::List(b)) => a.partial_cmp(b),
            (Self::Map(a), Self::Map(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Expression {
    pub fn builtin(
        name: impl Into<String>,
        body: fn(Vec<Self>, &mut Environment) -> Result<Self, Error>,
        help: impl Into<String>,
    ) -> Self {
        Self::Builtin(Builtin {
            name: name.into(),
            body,
            help: help.into(),
        })
    }

    pub fn new(x: impl Into<Self>) -> Self {
        x.into()
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Integer(i) => *i != 0,
            Self::Float(f) => *f != 0.0,
            Self::String(s) => !s.is_empty(),
            Self::Bytes(b) => !b.is_empty(),
            Self::Boolean(b) => *b,
            Self::List(exprs) => !exprs.is_empty(),
            Self::Map(exprs) => !exprs.is_empty(),
            Self::Lambda(_, _, _) => true,
            Self::Macro(_, _) => true,
            Self::Builtin(_) => true,
            _ => false,
        }
    }

    fn get_used_symbols(&self) -> Vec<&str> {
        let mut result = vec![];
        self.add_to_used_symbols(&mut result);
        result
    }

    fn add_to_used_symbols<'a>(&'a self, buf: &mut Vec<&'a str>) {
        match self {
            Self::Symbol(name) => {
                buf.push(name);
            }
            Self::None
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bytes(_)
            | Self::String(_)
            | Self::Boolean(_)
            | Self::Builtin(_) => {}

            Self::For(_, list, body) => {
                list.add_to_used_symbols(buf);
                body.add_to_used_symbols(buf);
            }

            Self::Do(exprs) | Self::List(exprs) => {
                for expr in exprs {
                    expr.add_to_used_symbols(buf);
                }
            }
            Self::Map(exprs) => {
                for expr in exprs.values() {
                    expr.add_to_used_symbols(buf);
                }
            }

            Self::Group(inner) | Self::Quote(inner) => {
                inner.add_to_used_symbols(buf);
            }
            Self::Lambda(_, body, _) => body.add_to_used_symbols(buf),
            Self::Macro(_, body) => body.add_to_used_symbols(buf),

            Self::Assign(_, expr) => expr.add_to_used_symbols(buf),
            Self::If(cond, t, e) => {
                cond.add_to_used_symbols(buf);
                t.add_to_used_symbols(buf);
                e.add_to_used_symbols(buf);
            }
            Self::Apply(g, args) => {
                g.add_to_used_symbols(buf);
                for expr in args {
                    expr.add_to_used_symbols(buf);
                }
            }
        }
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Self, Error> {
        self.clone().eval_mut(env, 0)
    }

    pub fn eval_capturing(self, env: &mut Environment, depth: usize) -> Result<Self, Error> {
        env.capture_stdio(|env| self.eval_mut(env, depth))
    }

    fn eval_mut(mut self, env: &mut Environment, mut depth: usize) -> Result<Self, Error> {
        loop {
            if let Some(max_depth) = MAX_RECURSION_DEPTH {
                if depth > max_depth {
                    return Err(Error::RecursionDepth(self));
                }
            }

            match self {
                Self::Quote(inner) => return Ok(*inner),
                Self::Group(inner) => return inner.eval_capturing(env, depth + 1),

                Self::Symbol(name) => {
                    return Ok(match env.get(&name) {
                        Some(expr) => expr,
                        None => Self::Symbol(name),
                    })
                }

                Self::Assign(name, expr) => {
                    let x = expr.eval_capturing(env, depth + 1)?;
                    env.define(&name, x);
                    return Ok(Self::None);
                }

                Self::For(name, list, body) => {
                    if let Expression::List(items) =
                        (*list).clone().eval_capturing(env, depth + 1)?
                    {
                        return Ok(Self::List(
                            items
                                .into_iter()
                                .map(|item| {
                                    env.define(&name, item);
                                    (*body).clone().eval_mut(env, depth + 1)
                                })
                                .collect::<Result<Vec<Self>, Error>>()?,
                        ));
                    } else {
                        return Err(Error::ForNonList(*list));
                    }
                }

                Self::If(cond, true_expr, false_expr) => {
                    return if cond.eval_capturing(env, depth + 1)?.is_truthy() {
                        true_expr
                    } else {
                        false_expr
                    }
                    .eval_mut(env, depth + 1)
                }

                Self::Apply(ref f, mut args) => {
                    match (**f).clone().eval_mut(env, depth + 1)? {
                        Self::Symbol(name) | Self::String(name) => {
                            let bindings = env
                                .bindings
                                .iter()
                                .map(|(k, v)| (k.clone(), v.to_string()))
                                // This is to prevent environment variables from getting too large.
                                // This causes some strange bugs on Linux: mainly it becomes
                                // impossible to execute any program because `the argument
                                // list is too long`.
                                .filter(|(_, s)| s.len() <= 1024)
                                .collect::<BTreeMap<String, String>>();

                            let mut command = Command::new(&name);
                            command
                                .current_dir(env.get_cwd())
                                .args(
                                    args.iter()
                                        .filter(|&x| x != &Self::None)
                                        .map(|x| {
                                            Ok(x.clone()
                                                .eval_capturing(env, depth + 1)?
                                                .to_string())
                                        })
                                        .collect::<Result<Vec<String>, Error>>()?,
                                )
                                .envs(bindings);

                            let captured = env.is_capturing_stdio();
                            if captured {
                                command.stdout(Stdio::piped()).stderr(Stdio::inherit());
                            } else {
                                command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
                            }

                            match command.output() {
                                Ok(output) => {
                                    if captured {
                                        let stdout =
                                            String::from_utf8_lossy(&output.stdout).to_string();
                                        return Ok(stdout.into());
                                    } else {
                                        return Ok(Self::None);
                                    }
                                }
                                Err(e) => {
                                    return Err(match e.kind() {
                                        ErrorKind::NotFound => Error::CustomError(format!(
                                            "program \"{}\" not found",
                                            name
                                        )),
                                        ErrorKind::PermissionDenied => Error::CustomError(format!(
                                            "permission to execute \"{}\" denied",
                                            name
                                        )),
                                        _ => Error::CommandFailed(name, args),
                                    })
                                }
                            }
                        }

                        Self::Lambda(param, body, mut new_env) if !args.is_empty() => {
                            let first = args.remove(0);
                            new_env.set_cwd(env.get_cwd());
                            new_env.define(&param, first.eval_capturing(env, depth + 1)?);
                            let result = body.eval_mut(&mut new_env, depth + 1)?;

                            if !args.is_empty() {
                                self = Self::Apply(Box::new(result), args);
                            } else {
                                return Ok(result);
                            }
                        }

                        Self::Macro(param, body) if !args.is_empty() => {
                            let first = args.remove(0);
                            let x = first.eval_mut(env, depth + 1)?;
                            env.define(&param, x);
                            self = if args.is_empty() {
                                *body
                            } else {
                                Self::Apply(
                                    Box::new(body.eval_mut(env, depth + 1)?),
                                    args[1..].to_vec(),
                                )
                            };
                        }

                        Self::Builtin(Builtin { body, .. }) => {
                            return body(args, env);
                        }

                        _ => return Err(Error::CannotApply(*f.clone(), args)),
                    }
                }

                // // Apply a function or macro to an argument
                Self::Lambda(param, body, captured) => {
                    let mut tmp_env = captured.clone();
                    tmp_env.define(&param, Expression::None);
                    tmp_env.set_cwd(env.get_cwd());
                    for symbol in body.get_used_symbols() {
                        if symbol != param && !captured.is_defined(symbol) {
                            if let Some(val) = env.get(symbol) {
                                tmp_env.define(symbol, val)
                            }
                        }
                    }
                    return Ok(Self::Lambda(param, body, tmp_env));
                }

                Self::List(exprs) => {
                    return Ok(Self::List(
                        exprs
                            .into_iter()
                            .map(|x| x.eval_mut(env, depth + 1))
                            .collect::<Result<Vec<Self>, Error>>()?,
                    ))
                }
                Self::Map(exprs) => {
                    return Ok(Self::Map(
                        exprs
                            .into_iter()
                            .map(|(n, x)| Ok((n, x.eval_mut(env, depth + 1)?)))
                            .collect::<Result<BTreeMap<String, Self>, Error>>()?,
                    ))
                }
                Self::Do(mut exprs) => {
                    if exprs.is_empty() {
                        return Ok(Self::None);
                    }

                    for expr in &exprs[..exprs.len() - 1] {
                        expr.clone().eval_mut(env, depth + 1)?;
                    }
                    self = exprs.pop().unwrap();
                }
                Self::None
                | Self::Integer(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Bytes(_)
                | Self::String(_)
                | Self::Macro(_, _)
                | Self::Builtin(_) => return Ok(self),
            }
            depth += 1;
        }
    }
}

impl Add for Expression {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(m), Self::Integer(n)) => match m.checked_add(n) {
                Some(i) => Self::Integer(i),
                None => Self::None,
            },
            (Self::Integer(m), Self::Float(n)) => Self::Float(m as f64 + n),
            (Self::Float(m), Self::Integer(n)) => Self::Float(m + n as f64),
            (Self::Float(m), Self::Float(n)) => Self::Float(m + n),
            (Self::String(m), Self::String(n)) => Self::String(m + &n),
            (Self::Bytes(mut a), Self::Bytes(b)) => {
                a.extend(b);
                Self::Bytes(a)
            }
            (Self::List(mut a), Self::List(b)) => {
                a.extend(b);
                Self::List(a)
            }
            _ => Self::None,
        }
    }
}

impl Sub for Expression {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(m), Self::Integer(n)) => match m.checked_sub(n) {
                Some(i) => Self::Integer(i),
                None => Self::None,
            },
            (Self::Integer(m), Self::Float(n)) => Self::Float(m as f64 - n),
            (Self::Float(m), Self::Integer(n)) => Self::Float(m - n as f64),
            (Self::Float(m), Self::Float(n)) => Self::Float(m - n),
            (Self::Map(mut m), Self::String(n)) => match m.remove_entry(&n) {
                Some((_, val)) => val,
                None => Self::None,
            },
            (Self::List(mut m), Self::Integer(n)) if m.len() > n as usize => m.remove(n as usize),
            _ => Self::None,
        }
    }
}

impl Mul for Expression {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(m), Self::Integer(n)) => match m.checked_mul(n) {
                Some(i) => Self::Integer(i),
                None => Self::None,
            },
            (Self::Integer(m), Self::Float(n)) => Self::Float(m as f64 * n),
            (Self::Float(m), Self::Integer(n)) => Self::Float(m * n as f64),
            (Self::Float(m), Self::Float(n)) => Self::Float(m * n),
            (Self::String(m), Self::Integer(n)) | (Self::Integer(n), Self::String(m)) => {
                Self::String(m.repeat(n as usize))
            }
            (Self::List(m), Self::Integer(n)) | (Self::Integer(n), Self::List(m)) => {
                let mut result = vec![];
                for _ in 0..n {
                    result.extend(m.clone());
                }
                Self::List(result)
            }
            _ => Self::None,
        }
    }
}

impl Div for Expression {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(m), Self::Integer(n)) => match m.checked_div(n) {
                Some(i) => Self::Integer(i),
                None => Self::None,
            },
            (Self::Integer(m), Self::Float(n)) => Self::Float(m as f64 / n),
            (Self::Float(m), Self::Integer(n)) => Self::Float(m / n as f64),
            (Self::Float(m), Self::Float(n)) => Self::Float(m / n),
            _ => Self::None,
        }
    }
}

impl Rem for Expression {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (Self::Integer(m), Self::Integer(n)) => Self::Integer(m % n),
            _ => Self::None,
        }
    }
}

impl<T> Index<T> for Expression
where
    T: Into<Self>,
{
    type Output = Self;

    fn index(&self, idx: T) -> &Self {
        match (self, idx.into()) {
            (Self::Map(m), Self::Symbol(name)) | (Self::Map(m), Self::String(name)) => {
                match m.get(&name) {
                    Some(val) => val,
                    None => &Self::None,
                }
            }

            (Self::List(list), Self::Integer(n)) if list.len() > n as usize => &list[n as usize],
            _ => &Self::None,
        }
    }
}
