use std::fmt::{Display, Formatter, Result};

use super::Expr;

#[derive(PartialEq, Eq)]
pub struct Mul<'a> {
    exprs: Vec<Expr<'a>>,
}

impl<'a> Mul<'a> {
    pub fn new(exprs: Vec<Expr<'a>>) -> Self {
        Mul { exprs }
    }
}

impl<'a> Display for Mul<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = self.exprs[0].to_string();
        for i in 1..self.exprs.len() {
            result = format!("{}*{}", result, self.exprs[i]);
        }
        result = format!("({})", result);
        write!(f, "{}", result)
    }
}

#[test]
fn test_mul() {
    use super::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    let mul = Mul::new(vec![Expr::Sym(x), Expr::Sym(y), Expr::Sym(y)]);
    assert_eq!(mul.to_string().as_str(), "(x*y*y)");
}
