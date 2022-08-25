use super::{Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Sym {
    symbol: String,
    sub: String,
}

impl ToExpr for Sym {
    fn to_expr(self) -> Expr {
        Expr::Sym(self)
    }
}

impl Sym {
    pub fn new<T: AsRef<str>>(symbol: T) -> Self {
        Sym {
            symbol: symbol.as_ref().to_string(),
            sub: "".to_owned(),
        }
    }

    pub fn set_sub(&mut self, sub: String) {
        self.sub = sub;
    }
}

impl Display for Sym {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.sub.is_empty() {
            write!(f, "{}", self.symbol)
        } else {
            write!(f, "{}_{{{}}}", self.symbol, self.sub)
        }
    }
}

#[test]
fn test_sym() {
    use super::test_util::*;

    assert_eq!((x() + y()).to_string(), "x+y");
    assert_eq!((x() + y() + x()).to_string(), "x+y+x");
    assert_eq!((x() ^ y()).to_string(), "x^{y}");
    assert_eq!(((x() + y()) ^ y()).to_string(), "(x+y)^{y}");
}
