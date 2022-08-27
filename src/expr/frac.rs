use std::fmt::{Display, Formatter, Result};

use crate::Expr;

use super::ToExpr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Frac {
    pub numer: Box<Expr>,
    pub denom: Box<Expr>,
}

impl Frac {
    pub fn new(numer: Expr, denom: Expr) -> Self {
        Frac {
            numer: Box::new(numer),
            denom: Box::new(denom),
        }
    }
}

impl ToExpr for Frac {
    fn to_expr(self) -> Expr {
        Expr::Frac(self)
    }
}

impl Display for Frac {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "frac({})({})", self.numer, self.denom)
    }
}
