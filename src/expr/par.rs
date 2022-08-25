use super::{Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Par {
    pub inner: Box<Expr>,
}

impl ToExpr for Par {
    fn to_expr(self) -> Expr {
        Expr::Par(self)
    }
}

impl Par {
    pub fn new(inner: Expr) -> Self {
        Par {
            inner: Box::new(inner),
        }
    }

    pub fn from<T: ToExpr>(inner: T) -> Self {
        Par {
            inner: Box::new(inner.to_expr()),
        }
    }
}

impl Display for Par {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.inner)
    }
}

#[test]
fn test_paren() {
    use super::test_util::*;
    asrt(Par::from(x() + y()), "(x+y)");
}
