use super::{mul::Mul, Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Pow {
    pub body: Box<Expr>,
    pub pow: Box<Expr>,
}

impl ToExpr for Pow {
    fn to_expr(self) -> Expr {
        Expr::Pow(self)
    }
}

impl Pow {
    pub fn to_mul(&self) -> Mul {
        match *self.pow {
            Expr::Num(x) if x.num > 0 => Mul::new(vec![*self.body.clone(); x.num as usize]),
            _ => panic!("Tried to convert pow to mul with non-integer pow"),
        }
    }
}

impl Pow {
    pub fn new(body: Expr, pow: Expr) -> Self {
        Pow {
            body: Box::new(body),
            pow: Box::new(pow),
        }
    }

    pub fn from<E: ToExpr, F: ToExpr>(body: E, pow: F) -> Self {
        Pow {
            body: Box::new(body.to_expr()),
            pow: Box::new(pow.to_expr()),
        }
    }
}

impl Display for Pow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Expr::Sym(_) | Expr::Num(_) | Expr::Par(_) = *self.body {
            write!(f, "{}", format_args!("{}^{{{}}}", self.body, self.pow))
        } else {
            write!(f, "{}", format_args!("({})^{{{}}}", self.body, self.pow))
        }
    }
}

#[test]
fn test_pow() {
    use super::test_util::*;
    use super::{Add, Num};
    let pow = Pow::from(x(), y());
    assert_eq!(pow.to_string(), "x^{y}");

    let pow = Pow::from(
        Add::new(vec![Expr::Sym(x()), Expr::Sym(y())]),
        Add::new(vec![Expr::Sym(x()), Expr::Sym(y())]),
    );
    assert_eq!(pow.to_string(), "(x+y)^{x+y}");
    let pow = Pow::from(Add::new(vec![Expr::Sym(x()), Expr::Sym(y())]), Num::new(2));
    assert_eq!(pow.to_mul().to_string(), "(x+y)*(x+y)");
}
