use std::{
    fmt::{Display, Formatter, Result},
    ops,
};

use super::add::Add;

#[derive(Clone, Copy)]
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
        Add::new(vec![self, _rhs])
    }
}

#[test]
fn test_sym() {
    let x = Sym::new("x");
    println!("{x}");
    println!("{}", x + x);
}
