use std::fmt::{Display, Formatter, Result};

use super::Expr;

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
    use super::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    let pow = Pow::new(Expr::Sym(x), Expr::Sym(y));
    assert_eq!(pow.to_string().as_str(), "x^y");
    let pow = Pow::new(Expr::Add(x + y), Expr::Add(x + y));
    assert_eq!(pow.to_string().as_str(), "(x+y)^(x+y)");
}
