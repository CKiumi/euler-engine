use std::fmt::{Display, Formatter, Result};

use crate::Expr;

use super::ToExpr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum FuncName {
    Sin,
    Cos,
    Tan,
    Re,
    Im,
}

impl Display for FuncName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FuncName::Sin => write!(f, "sin"),
            FuncName::Cos => write!(f, "cos"),
            FuncName::Tan => write!(f, "tan"),
            FuncName::Re => write!(f, "Re"),
            FuncName::Im => write!(f, "Im"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Func {
    pub name: FuncName,
    pub args: Box<Expr>,
}

impl Func {
    pub fn new(name: FuncName, args: Expr) -> Self {
        Func {
            name,
            args: Box::new(args),
        }
    }
}

impl ToExpr for Func {
    fn to_expr(self) -> Expr {
        Expr::Func(self)
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let args = self.args.to_string();
        write!(f, "{}({})", self.name, args)
    }
}