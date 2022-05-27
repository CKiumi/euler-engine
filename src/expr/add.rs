use std::fmt::{Display, Formatter, Result};

use super::Expr;

#[derive(PartialEq, Eq)]
pub struct Add<'a> {
    exprs: Vec<Expr<'a>>,
}

impl<'a> Add<'a> {
    pub fn new(exprs: Vec<Expr<'a>>) -> Self {
        Add { exprs }
    }
}

impl<'a> Display for Add<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = self.exprs[0].to_string();
        for i in 1..self.exprs.len() {
            result = format!("{}+{}", result, self.exprs[i]);
        }
        result = format!("({})", result);
        write!(f, "{}", result)
    }
}

#[test]
fn test_add() {
    use super::{Mul, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let add = Add::new(vec![Expr::Sym(x), Expr::Sym(y), Expr::Sym(y)]);
    let mul = Mul::new(vec![Expr::Sym(x), Expr::Sym(y)]);
    assert_eq!(add.to_string().as_str(), "(x+y+y)");
    let add = Add::new(vec![Expr::Sym(x), Expr::Sym(y), Expr::Mul(mul)]);
    assert_eq!(add.to_string().as_str(), "(x+y+(x*y))");
}
