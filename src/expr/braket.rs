use super::{Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Ket {
    pub inner: String,
}

impl Ket {
    pub fn new<T: AsRef<str>>(inner: T) -> Self {
        Ket {
            inner: inner.as_ref().to_string(),
        }
    }
}

impl ToExpr for Ket {
    fn to_expr(self) -> Expr {
        Expr::Ket(self)
    }
}

impl Display for Ket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "|{}>", self.inner)
    }
}

#[test]
fn test_ket() {
    println!("{}", Ket::new("000"));
}
