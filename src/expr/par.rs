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
    use crate::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    assert_eq!(Par::new(Expr::Add(x + y)).to_string(), "(x+y)");
}
