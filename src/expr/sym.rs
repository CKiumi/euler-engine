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
    let x = Sym::new("x");
    let y = Sym::new("y");

    assert_eq!((x + y).to_string(), "(x+y)");
    assert_eq!((x + y + x).to_string(), "(x+y+x)");
    assert_eq!((x ^ y).to_string(), "x^y");
    assert_eq!(((x + y) ^ y).to_string(), "(x+y)^y");
}
