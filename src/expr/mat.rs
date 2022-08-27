use super::ToExpr;
use crate::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Mat {
    pub elems: Vec<Vec<Expr>>,
}

impl Mat {
    pub fn new(elems: Vec<Vec<Expr>>) -> Self {
        Mat { elems }
    }
}

impl ToExpr for Mat {
    fn to_expr(self) -> Expr {
        Expr::Mat(self)
    }
}

impl Display for Mat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = "mat([".to_string();
        for row in &self.elems {
            result.push('[');
            for elem in row {
                result = format!("{result}{elem}, ");
            }
            result.pop();
            result.pop();
            result.push_str("], ");
        }
        result.pop();
        result.pop();
        write!(f, "{result}])")
    }
}
