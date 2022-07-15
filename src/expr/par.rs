use super::Expr;
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Par<'a> {
    pub inner: Box<Expr<'a>>,
}

impl<'a> Par<'a> {
    pub fn new(inner: Expr<'a>) -> Self {
        Par {
            inner: Box::new(inner),
        }
    }
}

impl<'a> Display for Par<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.inner)
    }
}

#[test]
fn test_paren() {
    use crate::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    assert_eq!(Par::new(Expr::Add(x + y)).to_string(), "(x+y)");
}
