use super::{mul::Mul, Expr, ToExpr};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Pow<'a> {
    pub body: Box<Expr<'a>>,
    pub pow: Box<Expr<'a>>,
}

impl<'a> ToExpr<'a> for Pow<'a> {
    fn to_expr(self) -> Expr<'a> {
        Expr::Pow(self)
    }
}

impl<'a> Pow<'a> {
    pub fn to_mul(&self) -> Mul {
        match *self.pow {
            Expr::Num(x) if x.num > 0 => Mul::new(vec![*self.body.clone(); x.num as usize]),
            _ => panic!("Tried to convert pow to mul with non-integer pow"),
        }
    }
}

impl<'a> Pow<'a> {
    pub fn new(body: Expr<'a>, pow: Expr<'a>) -> Self {
        Pow {
            body: Box::new(body),
            pow: Box::new(pow),
        }
    }

    pub fn from<E: ToExpr<'a>, F: ToExpr<'a>>(body: E, pow: F) -> Self {
        Pow {
            body: Box::new(body.to_expr()),
            pow: Box::new(pow.to_expr()),
        }
    }
}

impl<'a> Display for Pow<'a> {
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
    use super::{Add, Num, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let pow = Pow::from(x, y);
    assert_eq!(pow.to_string(), "x^{y}");

    let pow = Pow::from(
        Add::new(vec![Expr::Sym(x), Expr::Sym(y)]),
        Add::new(vec![Expr::Sym(x), Expr::Sym(y)]),
    );
    assert_eq!(pow.to_string(), "(x+y)^{x+y}");
    let pow = Pow::from(Add::new(vec![Expr::Sym(x), Expr::Sym(y)]), Num::new(2));
    assert_eq!(pow.to_mul().to_string(), "(x+y)*(x+y)");
}
