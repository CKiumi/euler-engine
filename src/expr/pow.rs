use super::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
pub struct Pow<'a> {
    body: Box<Expr<'a>>,
    pow: Box<Expr<'a>>,
}

impl<'a> Pow<'a> {
    pub fn new(body: Expr<'a>, pow: Expr<'a>) -> Self {
        Pow {
            body: Box::new(body),
            pow: Box::new(pow),
        }
    }
}

impl<'a> Display for Pow<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let result = format!("{}^{}", self.body, self.pow);
        write!(f, "{}", result)
    }
}

#[test]
fn test_pow() {
    use super::{Add, Sym};
    use crate::{add, sym};
    let test = Pow::new(sym!("x"), sym!("y"));
    assert_eq!(test.to_string().as_str(), "x^y");
    let pow = Pow::new(add![sym!("x"), sym!("y")], add![sym!("x"), sym!("y")]);
    assert_eq!(pow.to_string().as_str(), "(x+y)^(x+y)");
}
