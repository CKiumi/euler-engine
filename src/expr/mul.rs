use super::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
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
    use super::{Mul, Sym};
    use crate::{mul, sym};
    let mul = mul![sym!("x"), sym!("y"), sym!("y")];
    assert_eq!(mul.to_string().as_str(), "(x*y*y)");
}
