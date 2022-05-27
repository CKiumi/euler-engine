use std::fmt::{Display, Formatter, Result};

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

#[test]
fn test_sym() {
    use super::Expr;
    use crate::sym;
    assert_eq!(sym!("x").to_string().as_str(), "x");
    assert_eq!((sym!("x") + sym!("x")).to_string().as_str(), "(x+x)");
}
