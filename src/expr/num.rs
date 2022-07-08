use std::fmt::{Display, Formatter, Result};

use crate::Expr;
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct Num {
    pub num: i32,
}

impl Num {
    pub fn new(num: i32) -> Self {
        Num { num }
    }
}

pub trait AsExpr {
    fn as_expr<'a>(&self) -> Expr<'a>;
}

impl AsExpr for Num {
    fn as_expr<'a>(&self) -> Expr<'a> {
        Expr::Num(*self)
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.num)
    }
}

#[test]
fn test_num() {
    use super::Expr;
    let x = Expr::Num(Num::new(1));
    let y = Expr::Num(Num::new(2));
    assert_eq!((x + y).to_string(), "1+2");
    assert_eq!((Num::new(3) ^ Num::new(3)).to_string(), "27");
}
