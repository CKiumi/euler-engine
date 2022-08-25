use std::fmt::{Display, Formatter, Result};

use crate::Expr;

use super::ToExpr;
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Debug, Eq)]
pub struct Num {
    pub num: i32,
}

impl Num {
    pub fn new(num: i32) -> Self {
        Num { num }
    }
}

impl ToExpr for Num {
    fn to_expr(self) -> Expr {
        Expr::Num(self)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.num)
    }
}

#[test]
fn test_num() {
    use super::test_util::*;
    use super::Expr;
    let x = Expr::Num(Num::new(1));
    let y = Expr::Num(Num::new(2));
    asrt(x + y, "1+2");
    asrt(Num::new(3) ^ Num::new(3), "3^{3}");
}
