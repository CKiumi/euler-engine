use super::{Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Sym {
    pub symbol: String,
    pub sub: String,
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
    asrt(x() + y(), "x+y");
    asrt(x() + y() + x(), "x+y+x");
    asrt(x() ^ y(), "x^{y}");
    asrt((x() + y()) ^ y(), "(x+y)^{y}");
}
