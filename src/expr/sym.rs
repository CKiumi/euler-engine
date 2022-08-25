use super::{Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Sym<'a> {
    symbol: &'a str,
    sub: &'a str,
}

impl<'a> ToExpr<'a> for Sym<'a> {
    fn to_expr(self) -> Expr<'a> {
        Expr::Sym(self)
    }
}

impl<'a> Sym<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Sym { symbol, sub: "" }
    }

    pub fn set_sub(&mut self, sub: &'a str) -> Self {
        self.sub = sub;
        *self
    }
}

impl<'a> Display for Sym<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.sub.is_empty() {
            write!(f, "{}", self.symbol)
        } else {
            write!(f, "{}_{{{}}}", self.symbol, self.sub)
        }
    }
}

#[test]
fn test_sym() {
    let x = Sym::new("x");
    let y = Sym::new("y");

    assert_eq!((x + y).to_string(), "x+y");
    assert_eq!((x + y + x).to_string(), "x+y+x");
    assert_eq!((x ^ y).to_string(), "x^{y}");
    assert_eq!(((x + y) ^ y).to_string(), "(x+y)^{y}");
}
