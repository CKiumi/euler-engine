use super::{mul::Mul, Expr};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
pub struct Pow<'a> {
    pub body: Box<Expr<'a>>,
    pub pow: Box<Expr<'a>>,
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
}

impl<'a> Display for Pow<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let result = format!("{}^{}", self.body, self.pow);
        write!(f, "{}", result)
    }
}

#[test]
fn test_pow() {
    use super::{Add, Num, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let pow = Pow::new(Expr::Sym(x), Expr::Sym(y));
    assert_eq!(pow.to_string(), "x^y");

    let pow = Pow::new(
        Expr::Add(Add::new(vec![Expr::Sym(x), Expr::Sym(y)])),
        Expr::Add(Add::new(vec![Expr::Sym(x), Expr::Sym(y)])),
    );
    assert_eq!(pow.to_string(), "(x+y)^(x+y)");

    let pow = Pow::new(
        Expr::Add(Add::new(vec![Expr::Sym(x), Expr::Sym(y)])),
        Expr::Num(Num::new(2)),
    );
    assert_eq!(pow.to_mul().to_string(), "(x+y)*(x+y)");
}
