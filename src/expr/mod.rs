mod add;
mod mul;
mod num;
pub mod sym;
use add::Add;
use mul::Mul;
use num::Num;
use std::fmt::{Display, Formatter, Result};
use sym::Sym;

#[derive(PartialEq, Eq)]
pub enum Expr<'a> {
    Num(Num),
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
            Expr::Num(num) => write!(f, "{}", num),
        }
    }
}
