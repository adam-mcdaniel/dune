use super::{Environment, Error, Int};
use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    io::ErrorKind,
    ops::{Add, Div, Index, Mul, Rem, Sub},
    process::Command,
};

use prettytable::{row, cell, Table, format::{LinePosition, LineSeparator}};

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

#[derive(Clone)]
pub enum Expression {
    Group(Box<Self>),

    Symbol(String),
    // An integer literal
    Integer(Int),
    // A floating point number literal
    Float(f64),
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
    //
    // The first argument is the name of the function, the second is the
    // function pointer for executing the function. The third is the
    // help string for the function.
    Builtin(
        String,
        fn(Vec<Self>, &mut Environment) -> Result<Self, Error>,
        String,
    ),

    Quote(Box<Self>),
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Quote(inner) => write!(f, "'{:?}", inner),
            Self::Group(inner) => write!(f, "({:?})", inner),
            Self::Symbol(name) => write!(f, "{}", name),
            Self::Integer(i) => write!(f, "{}", *i),
            Self::Float(n) => write!(f, "{}", *n),
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
            Self::Builtin(name, _, _) => write!(f, "builtin@{}", name),
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
            // Self::Map(exprs) => write!(
            //     f,
            //     "{{{}}}",
            //     exprs
            //         .iter()
            //         .map(|(k, e)| format!("{}: {:?}", k, e))
            //         .collect::<Vec<String>>()
            //         .join(", ")
            // ),
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
                for (key, val) in exprs {
                    if let Self::Builtin(_, _, help) = &val {
                        t.add_row(row!(key, format!("{}", val), help));
                    } else {
                        t.add_row(row!(key, format!("{}", val)));
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
            Self::Builtin(name, _, _) => write!(f, "builtin@{}", name),
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol(a), Self::Symbol(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Map(a), Self::Map(b)) => a == b,
            (Self::None, Self::None) => true,
            (Self::Lambda(a, b, c), Self::Lambda(x, y, z)) => a == x && b == y && c == z,
            (Self::Macro(a, b), Self::Macro(c, d)) => a == c && b == d,
            (Self::Builtin(a, _, _), Self::Builtin(b, _, _)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
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
        Self::Builtin(name.to_string(), body, help.to_string())
    }

    pub fn new(x: impl Into<Self>) -> Self {
        x.into()
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Integer(i) => *i != 0,
            Self::Float(f) => *f != 0.0,
            Self::String(s) => !s.is_empty(),
            Self::Boolean(b) => *b,
            Self::List(exprs) => !exprs.is_empty(),
            Self::Map(exprs) => !exprs.is_empty(),
            Self::Lambda(_, _, _) => true,
            Self::Macro(_, _) => true,
            Self::Builtin(_, _, _) => true,
            _ => false,
        }
    }

    fn get_used_symbols(&self) -> Vec<String> {
        match self {
            Self::Symbol(name) => vec![name.clone()],
            Self::None
            | Self::Integer(_)
            | Self::Float(_)
            | Self::String(_)
            | Self::Boolean(_)
            | Self::Builtin(_, _, _) => vec![],

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
                for (_, expr) in exprs {
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
        self.clone().eval_mut(env)
    }

    fn eval_mut(mut self, mut env: &mut Environment) -> Result<Self, Error> {
        loop {
            match self {
                Self::Quote(inner) => return Ok(*inner),
                Self::Group(inner) => return inner.eval_mut(env),

                Self::Symbol(name) => {
                    return Ok(match env.get(&name) {
                        Some(expr) => expr,
                        None => Self::Symbol(name.clone()),
                    })
                }

                Self::Assign(name, expr) => {
                    let x = expr.eval_mut(env)?;
                    env.define(&name, x.clone());
                    return Ok(x);
                }

                Self::For(name, list, body) => {
                    if let Expression::List(items) = list.clone().eval_mut(env)? {
                        return Ok(Self::List(
                            items
                                .into_iter()
                                .map(|item| {
                                    env.define(&name, item);
                                    body.clone().eval_mut(env)
                                })
                                .collect::<Result<Vec<Self>, Error>>()?,
                        ));
                    } else {
                        return Err(Error::ForNonList(*list));
                    }
                }

                Self::If(cond, true_expr, false_expr) => {
                    return if cond.eval_mut(env)?.is_truthy() {
                        true_expr
                    } else {
                        false_expr
                    }
                    .eval_mut(env)
                }

                Self::Apply(ref f, ref args) => match f.clone().eval_mut(env)? {
                    Self::Symbol(name) | Self::String(name) => {
                        let bindings = env
                            .bindings
                            .iter()
                            .map(|(k, v)| (k.clone(), format!("{}", v)))
                            .collect::<BTreeMap<String, String>>();

                        match Command::new(&name)
                            .current_dir(env.get_cwd())
                            .args(
                                args.iter()
                                    .filter(|x| x.clone() != &Self::None)
                                    .map(|x| Ok(format!("{}", x.clone().eval_mut(env)?)))
                                    .collect::<Result<Vec<String>, Error>>()?,
                            )
                            .envs(bindings)
                            .status()
                        {
                            Ok(_) => return Ok(Self::None),
                            Err(e) => {
                                return Err(match e.kind() {
                                    ErrorKind::NotFound => {
                                        Error::CustomError(format!("\"{}\" not found", name))
                                    }
                                    ErrorKind::PermissionDenied => Error::CustomError(format!(
                                        "permission to execute \"{}\" denied",
                                        name
                                    )),
                                    _ => Error::CommandFailed(name, args.clone()),
                                })
                            }
                        }
                    }

                    Self::Lambda(param, body, old_env) if args.len() == 1 => {
                        let mut new_env = old_env.clone();
                        new_env.define(&param, args[0].clone().eval_mut(env)?);
                        return body.eval_mut(&mut new_env);
                    }

                    Self::Lambda(param, body, old_env) if args.len() > 1 => {
                        let mut new_env = old_env.clone();
                        new_env.define(&param, args[0].clone().eval_mut(env)?);
                        self =
                            Self::Apply(Box::new(body.eval_mut(&mut new_env)?), args[1..].to_vec());
                    }

                    Self::Macro(param, body) if args.len() == 1 => {
                        let x = args[0].clone().eval_mut(env)?;
                        env.define(&param, x);
                        self = *body;
                    }

                    Self::Macro(param, body) if args.len() > 1 => {
                        let x = args[0].clone().eval_mut(env)?;
                        env.define(&param, x);
                        self = Self::Apply(Box::new(body.eval_mut(&mut env)?), args[1..].to_vec());
                    }

                    Self::Builtin(_, g, _) => {
                        return g(args.clone(), env);
                    }

                    _ => return Err(Error::CannotApply(*f.clone(), args.clone())),
                },

                // // Apply a function or macro to an argument
                Self::Lambda(param, body, captured) => {
                    let mut tmp_env = captured.clone();
                    tmp_env.define(&param, Expression::None);
                    for symbol in body.get_used_symbols() {
                        if symbol != param && !captured.is_defined(&symbol) {
                            if let Some(val) = env.get(&symbol) {
                                tmp_env.define(&symbol, val)
                            }
                        }
                    }
                    return Ok(Self::Lambda(param.clone(), body.clone(), tmp_env));
                }

                Self::List(exprs) => {
                    return Ok(Self::List(
                        exprs
                            .into_iter()
                            .map(|x| x.eval_mut(env))
                            .collect::<Result<Vec<Self>, Error>>()?,
                    ))
                }
                Self::Map(exprs) => {
                    return Ok(Self::Map(
                        exprs
                            .into_iter()
                            .map(|(n, x)| Ok((n.clone(), x.eval_mut(env)?)))
                            .collect::<Result<BTreeMap<String, Self>, Error>>()?,
                    ))
                }
                Self::Do(exprs) => {
                    if exprs.is_empty() {
                        return Ok(Self::None);
                    }

                    for expr in &exprs[..exprs.len() - 1] {
                        expr.clone().eval_mut(env)?;
                    }
                    self = exprs[exprs.len() - 1].clone();
                }
                Self::None
                | Self::Integer(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::String(_)
                | Self::Macro(_, _)
                | Self::Builtin(_, _, _) => return Ok(self.clone()),
            }
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
            (Self::Map(m), Self::Symbol(name)) => match m.get(&name) {
                Some(val) => val,
                None => &Self::None,
            },
            (Self::Map(m), Self::String(name)) => match m.get(&name) {
                Some(val) => val,
                None => &Self::None,
            },
            (Self::List(list), Self::Integer(n)) if list.len() > n as usize => &list[n as usize],
            _ => &Self::None,
        }
    }
}
