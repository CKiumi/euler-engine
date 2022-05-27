mod add;
mod mul;
pub mod sym;
use std::fmt::{Display, Formatter, Result};

use add::Add;
use mul::Mul;
use sym::Sym;

#[derive(PartialEq, Eq)]
pub enum Expr<'a> {
    Sym(Sym<'a>),
    Add(Add<'a>),
    Mul(Mul<'a>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Sym(sym) => write!(f, "{}", sym),
            Expr::Add(add) => write!(f, "{}", add),
            Expr::Mul(mul) => write!(f, "{}", mul),
        }
    }
}
