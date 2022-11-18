use std::fmt::{Display, Formatter, Result};

use crate::{Expr, Num};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Tensor {
    pub exprs: Vec<Expr>,
}

impl Tensor {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Tensor { exprs }
    }
}

impl Display for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let handle_add = |expr: &Expr| match expr {
            Expr::Add(_) => format!("({expr})"),
            _ => expr.to_string(),
        };
        let exprs = &self.exprs;
        if exprs.len() == 1 {
            return write!(f, "{}", exprs[0]);
        }
        let mut result = match exprs[0] {
            Expr::Num(num) if num == Num::new(-1) => {
                format!("-{}", handle_add(&exprs[1]))
            }
            _ => format!("{}⊗{}", handle_add(&exprs[0]), handle_add(&exprs[1])),
        };
        for expr in &exprs[2..] {
            result = format!("{result}⊗{}", handle_add(expr))
        }
        write!(f, "{}", &result)
    }
}
