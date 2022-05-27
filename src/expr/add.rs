use super::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
pub struct Add<'a> {
    pub exprs: Vec<Expr<'a>>,
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

#[cfg(test)]
mod test_add {
    use super::super::{Add, Expr, Mul, Sym};
    use crate::{add, mul, sym};
    #[test]
    fn test_fmt() {
        let test = add![sym!("x"), sym!("y"), sym!("y")];
        assert_eq!(test.to_string().as_str(), "(x+y+y)");

        let test = add![sym!("x"), sym!("y"), mul!(sym!("x"), sym!("y"))];
        assert_eq!(test.to_string().as_str(), "(x+y+(x*y))");
    }
}
