use std::{
    fmt::{Display, Formatter, Result},
    ops,
};

use super::Add;
use super::Expr;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sym<'a> {
    symbol: &'a str,
}

impl<'a> Sym<'a> {
    pub fn new(symbol: &'a str) -> Self {
        Sym { symbol }
    }
}

impl<'a> Display for Sym<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.symbol)
    }
}

impl<'a> ops::Add<Sym<'a>> for Sym<'a> {
    type Output = Add<'a>;
    fn add(self, _rhs: Sym<'a>) -> Add<'a> {
        Add::new(vec![Expr::Sym(self), Expr::Sym(_rhs)])
    }
}

#[test]
fn test_sym() {
    let x = Sym::new("x");
    assert_eq!(x.to_string().as_str(), "x");
    assert_eq!((x + x).to_string().as_str(), "(x+x)");
}
