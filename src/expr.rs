use super::{Environment, Error, Int};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    io::ErrorKind,
    ops::{Add, Div, Index, Mul, Rem, Sub},
    process::Command,
};

use terminal_size::{terminal_size, Width};

use prettytable::{
    format::{LinePosition, LineSeparator},
    row, Cell, Row, Table,
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

impl From<Environment> for Expression {
    fn from(env: Environment) -> Self {
        Self::Map(
            env.bindings
                .into_iter()
                .map(|(name, item)| (name, item))
                .collect::<BTreeMap<String, Self>>(),
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
        let specified_width = f.width().unwrap_or(
            terminal_size()
                .map(|(Width(w), _)| w as usize)
                .unwrap_or(120),
        );
        // let width = match terminal_size() {
        //     Some((Width(width), _)) => Some(width as usize),
        //     _ => None,
        // }

        match self {
            Self::Quote(inner) => write!(f, "'{:?}", inner),
            Self::Group(inner) => write!(f, "({:?})", inner),
            Self::Symbol(name) => write!(f, "{}", name),
            Self::Integer(i) => write!(f, "{}", *i),
            Self::Float(n) => write!(f, "{}", *n),
            Self::Bytes(b) => write!(f, "b{:?}", b),
            Self::String(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{}", if *b { "True" } else { "False" }),
            Self::List(exprs) => {
                // Create a table with one column
                let mut t = Table::new();
                let fmt = t.get_format();
                fmt.padding(1, 1);
                fmt.borders('┃');
                fmt.column_separator('┃');
                fmt.separator(LinePosition::Top, LineSeparator::new('━', '┳', '┏', '┓'));
                fmt.separator(LinePosition::Title, LineSeparator::new('━', '╋', '┣', '┫'));
                fmt.separator(LinePosition::Intern, LineSeparator::new('━', '╋', '┣', '┫'));
                fmt.separator(LinePosition::Bottom, LineSeparator::new('━', '┻', '┗', '┛'));

                let mut row = vec![];
                let mut total_len = 1;
                for expr in exprs {
                    let formatted = format!("{}", expr);
                    // Get the length of the first line
                    if formatted.contains('\n') {
                        let first_line_len = formatted.lines().next().unwrap().len();
                        total_len += first_line_len + 1;
                    } else {
                        total_len += formatted.len() + 1;
                    }
                    row.push(formatted);
                }
                if total_len > specified_width {
                    return write!(f, "{:?}", self);
                }
                let row = Row::new(row.into_iter().map(|x| Cell::new(&x)).collect::<Vec<_>>());
                t.add_row(row);

                write!(f, "{}", t)
            }
            Self::Map(exprs) => {
                let mut t = Table::new();
                let fmt = t.get_format();
                fmt.padding(1, 1);
                // Set width to be 2/3
                fmt.borders('│');
                fmt.column_separator('│');
                fmt.separator(LinePosition::Top, LineSeparator::new('═', '╤', '╒', '╕'));
                fmt.separator(LinePosition::Title, LineSeparator::new('═', '╪', '╞', '╡'));
                fmt.separator(LinePosition::Intern, LineSeparator::new('─', '┼', '├', '┤'));
                fmt.separator(LinePosition::Bottom, LineSeparator::new('─', '┴', '└', '┘'));

                for (key, val) in exprs {
                    match &val {
                        Self::Builtin(Builtin { help, .. }) => {
                            t.add_row(row!(
                                key,
                                format!("{}", val),
                                textwrap::fill(help, specified_width / 6)
                            ));
                        }
                        Self::Map(_) => {
                            t.add_row(row!(key, format!("{:specified_width$}", val)));
                        }
                        Self::List(_) => {
                            let w = specified_width - key.len() - 3;
                            let formatted = format!("{:w$}", val);
                            t.add_row(row!(key, textwrap::fill(&formatted, w),));
                        }
                        _ => {
                            // Format the value to the width of the terminal / 5
                            let formatted = format!("{:?}", val);
                            let w = specified_width / 3;
                            t.add_row(row!(key, textwrap::fill(&formatted, w),));
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
        name: impl ToString,
        body: fn(Vec<Self>, &mut Environment) -> Result<Self, Error>,
        help: impl ToString,
    ) -> Self {
        Self::Builtin(Builtin {
            name: name.to_string(),
            body,
            help: help.to_string(),
        })
    }

    pub fn new(x: impl Into<Self>) -> Self {
        x.into()
    }

    pub fn apply(self, args: Vec<Self>) -> Self {
        Self::Apply(Box::new(self), args)
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

    pub fn flatten(args: Vec<Self>) -> Vec<Self> {
        let mut result = vec![];
        for arg in args {
            match arg {
                Self::List(exprs) => result.extend(Self::flatten(exprs)),
                Self::Group(expr) => result.extend(Self::flatten(vec![*expr])),
                _ => result.push(arg),
            }
        }
        result
    }

    fn get_used_symbols(&self) -> Vec<String> {
        match self {
            Self::Symbol(name) => vec![name.clone()],
            Self::None
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Bytes(_)
            | Self::String(_)
            | Self::Boolean(_)
            | Self::Builtin(_) => vec![],

            Self::For(_, list, body) => {
                let mut result = vec![];
                result.extend(list.get_used_symbols());
                result.extend(body.get_used_symbols());
                result
            }

            Self::Do(exprs) | Self::List(exprs) => {
                let mut result = vec![];
                for expr in exprs {
                    result.extend(expr.get_used_symbols())
                }
                result
            }
            Self::Map(exprs) => {
                let mut result = vec![];
                for expr in exprs.values() {
                    result.extend(expr.get_used_symbols())
                }
                result
            }

            Self::Group(inner) | Self::Quote(inner) => inner.get_used_symbols(),
            Self::Lambda(_, body, _) => body.get_used_symbols(),
            Self::Macro(_, body) => body.get_used_symbols(),

            Self::Assign(_, expr) => expr.get_used_symbols(),
            Self::If(cond, t, e) => {
                let mut result = vec![];
                result.extend(cond.get_used_symbols());
                result.extend(t.get_used_symbols());
                result.extend(e.get_used_symbols());
                result
            }
            Self::Apply(g, args) => {
                let mut result = g.get_used_symbols();
                for expr in args {
                    result.extend(expr.get_used_symbols())
                }
                result
            }
        }
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Self, Error> {
        self.clone().eval_mut(env, 0)
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
                Self::Group(inner) => return inner.eval_mut(env, depth + 1),

                Self::Symbol(name) => {
                    return Ok(match env.get(&name) {
                        Some(expr) => expr,
                        None => Self::Symbol(name.clone()),
                    })
                }

                Self::Assign(name, expr) => {
                    let x = expr.eval_mut(env, depth + 1)?;
                    env.define(&name, x);
                    return Ok(Self::None);
                }

                Self::For(name, list, body) => {
                    if let Expression::List(items) = list.clone().eval_mut(env, depth + 1)? {
                        let mut results = vec![];
                        for item in items {
                            env.define(&name, item);
                            results.push(body.clone().eval_mut(env, depth + 1)?);
                        }
                        return Ok(Self::List(results));
                        // return Ok(Self::List(
                        //     items
                        //         .into_iter()
                        //         .map(|item| {
                        //             env.define(&name, item);
                        //             body.clone().eval_mut(env, depth + 1)
                        //         })
                        //         .collect::<Result<Vec<Self>, Error>>()?,
                        // ));
                    } else {
                        return Err(Error::ForNonList(*list));
                    }
                }

                Self::If(cond, true_expr, false_expr) => {
                    return if cond.eval_mut(env, depth + 1)?.is_truthy() {
                        true_expr
                    } else {
                        false_expr
                    }
                    .eval_mut(env, depth + 1)
                }

                Self::Apply(ref f, ref args) => match f.clone().eval_mut(env, depth + 1)? {
                    Self::Symbol(name) | Self::String(name) => {
                        let bindings = env
                            .bindings
                            .clone()
                            .into_iter()
                            .map(|(k, v)| (k, v.to_string()))
                            // This is to prevent environment variables from getting too large.
                            // This causes some strange bugs on Linux: mainly it becomes
                            // impossible to execute any program because `the argument
                            // list is too long`.
                            .filter(|(_, s)| s.len() <= 1024)
                            .collect::<BTreeMap<String, String>>();

                        let mut cmd_args = vec![];
                        for arg in args {
                            for flattened_arg in
                                Self::flatten(vec![arg.clone().eval_mut(env, depth + 1)?])
                            {
                                match flattened_arg {
                                    Self::String(s) => cmd_args.push(s),
                                    Self::Bytes(b) => {
                                        cmd_args.push(String::from_utf8_lossy(&b).to_string())
                                    }
                                    Self::None => continue,
                                    _ => cmd_args.push(format!("{}", flattened_arg)),
                                }
                            }
                        }

                        match Command::new(&name)
                            .current_dir(env.get_cwd())
                            .args(
                                cmd_args, // Self::flatten(args.clone()).iter()
                                         //     .filter(|&x| x != &Self::None)
                                         //     // .map(|x| Ok(format!("{}", x.clone().eval_mut(env, depth + 1)?)))
                                         //     .collect::<Result<Vec<String>, Error>>()?,
                            )
                            .envs(bindings)
                            .status()
                        {
                            Ok(_) => return Ok(Self::None),
                            Err(e) => {
                                return Err(match e.kind() {
                                    ErrorKind::NotFound => Error::ProgramNotFound(name),
                                    ErrorKind::PermissionDenied => {
                                        Error::PermissionDenied(self.clone())
                                    }
                                    _ => Error::CommandFailed(name, args.clone()),
                                })
                            }
                        }
                    }

                    Self::Lambda(param, body, old_env) if args.len() == 1 => {
                        let mut new_env = old_env;
                        new_env.set_cwd(env.get_cwd());
                        new_env.define(&param, args[0].clone().eval_mut(env, depth + 1)?);
                        return body.eval_mut(&mut new_env, depth + 1);
                    }

                    Self::Lambda(param, body, old_env) if args.len() > 1 => {
                        let mut new_env = old_env.clone();
                        new_env.set_cwd(env.get_cwd());
                        new_env.define(&param, args[0].clone().eval_mut(env, depth + 1)?);
                        self = Self::Apply(
                            Box::new(body.eval_mut(&mut new_env, depth + 1)?),
                            args[1..].to_vec(),
                        );
                    }

                    Self::Macro(param, body) if args.len() == 1 => {
                        let x = args[0].clone().eval_mut(env, depth + 1)?;
                        env.define(&param, x);
                        self = *body;
                    }

                    Self::Macro(param, body) if args.len() > 1 => {
                        let x = args[0].clone().eval_mut(env, depth + 1)?;
                        env.define(&param, x);
                        self = Self::Apply(
                            Box::new(body.eval_mut(env, depth + 1)?),
                            args[1..].to_vec(),
                        );
                    }

                    Self::Builtin(Builtin { body, .. }) => {
                        return body(args.clone(), env);
                    }

                    _ => return Err(Error::CannotApply(*f.clone(), args.clone())),
                },

                // // Apply a function or macro to an argument
                Self::Lambda(param, body, captured) => {
                    let mut tmp_env = captured.clone();
                    tmp_env.define(&param, Expression::None);
                    tmp_env.set_cwd(env.get_cwd());
                    for symbol in body.get_used_symbols() {
                        if symbol != param && !captured.is_defined(&symbol) {
                            if let Some(val) = env.get(&symbol) {
                                tmp_env.define(&symbol, val)
                            }
                        }
                    }
                    return Ok(Self::Lambda(param.clone(), body, tmp_env));
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
                Self::Do(exprs) => {
                    if exprs.is_empty() {
                        return Ok(Self::None);
                    }

                    for expr in &exprs[..exprs.len() - 1] {
                        expr.clone().eval_mut(env, depth + 1)?;
                    }
                    self = exprs[exprs.len() - 1].clone();
                }
                Self::None
                | Self::Integer(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Bytes(_)
                | Self::String(_)
                | Self::Macro(_, _)
                | Self::Builtin(_) => return Ok(self.clone()),
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
