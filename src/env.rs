use super::{Error, Expression};
use std::collections::BTreeMap;

const CWD_ENV_VAR: &str = "CWD";

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Environment {
    pub bindings: BTreeMap<String, Expression>,
    parent: Option<Box<Self>>,
    capture_stdio: bool,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: BTreeMap::new(),
            parent: None,
            capture_stdio: false,
        }
    }

    pub fn get_cwd(&self) -> &str {
        match self.get_ref(CWD_ENV_VAR) {
            Some(Expression::String(path)) => path,
            _ => "/",
        }
    }

    pub fn set_cwd(&mut self, cwd: impl Into<String>) {
        self.define(CWD_ENV_VAR, Expression::String(cwd.into()));
    }

    pub fn get(&self, name: &str) -> Option<Expression> {
        match self.bindings.get(name) {
            Some(expr) => Some(expr.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }

    pub fn capture_stdio<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let capture_stdio = std::mem::replace(&mut self.capture_stdio, true);
        let result = f(self);
        self.capture_stdio = capture_stdio;
        result
    }

    pub fn is_capturing_stdio(&self) -> bool {
        self.capture_stdio
    }

    pub fn get_ref(&self, name: &str) -> Option<&Expression> {
        match self.bindings.get(name) {
            Some(expr) => Some(expr),
            None => match &self.parent {
                Some(parent) => parent.get_ref(name),
                None => None,
            },
        }
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
            || if let Some(ref parent) = self.parent {
                parent.is_defined(name)
            } else {
                false
            }
    }

    pub fn undefine(&mut self, name: &str) {
        self.bindings.remove(name);
    }

    pub fn define(&mut self, name: &str, expr: Expression) {
        self.bindings.insert(name.to_string(), expr);
    }

    pub fn define_builtin(
        &mut self,
        name: impl Into<String>,
        builtin: fn(Vec<Expression>, &mut Environment) -> Result<Expression, Error>,
        help: impl Into<String>,
    ) {
        let name: String = name.into();
        self.define(
            &name.clone(),
            Expression::builtin(name, builtin, help.into()),
        )
    }

    pub fn set_parent(&mut self, parent: Self) {
        self.parent = Some(Box::new(parent));
    }
}
