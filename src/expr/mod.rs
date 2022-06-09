mod add;
mod mul;
mod num;
mod ops;
mod pow;
pub mod sym;
pub use add::Add;
pub use mul::Mul;
pub use num::Num;
pub use pow::Pow;
use std::fmt::{Display, Formatter, Result};
pub use sym::Sym;

#[derive(PartialEq, Eq, Clone)]
pub enum Expr<'a> {
    Num(Num),
    Sym(Sym<'a>),
    Add(Add<'a>),
    Mul(Mul<'a>),
    Pow(Pow<'a>),
}

impl<'a> Expr<'a> {
    fn collect(&self) -> Self {
        match self {
            Expr::Add(add) => Expr::Add(add.collect()),
            Expr::Mul(mul) => Expr::Mul(mul.collect()),
            x => x.clone(),
        }
    }

    /// Pow(x,y)->(x,y) otherwise expr->(expr,1)
    fn detach_pow(&self) -> (Self, Self) {
        match self {
            Expr::Pow(pow) => (*pow.body.clone(), *pow.pow.clone()),
            _ => (self.clone(), Expr::Num(Num::new(1))),
        }
    }

    /// Multi(num,x1,...,xn)->(num,Multi(x1,...,xn)) otherwise expr->(1,expr)
    pub fn detach_coeff(&self) -> (Num, Mul<'a>) {
        match self {
            Expr::Mul(mul) => match mul.exprs[0] {
                Expr::Num(x) => (x, Mul::new(mul.exprs[1..].to_vec())),
                _ => (Num::new(1), Mul::new(mul.exprs.clone())),
            },
            x => (Num::new(1), Mul::new(vec![x.clone()])),
        }
    }
}

impl<'a> std::ops::Add<Expr<'a>> for Expr<'a> {
    type Output = Expr<'a>;
    fn add(self, _rhs: Expr<'a>) -> Expr {
        match (self, _rhs) {
            (Expr::Num(x), Expr::Num(y)) => Expr::Num(x + y),
            (Expr::Num(x), Expr::Sym(y)) => Expr::Add(x + y),
            (Expr::Num(x), Expr::Add(y)) => Expr::Add(x + y),
            (Expr::Num(x), Expr::Mul(y)) => Expr::Add(x + y),
            (Expr::Num(x), Expr::Pow(y)) => Expr::Add(x + y),
            (Expr::Sym(x), Expr::Num(y)) => Expr::Add(x + y),
            (Expr::Sym(x), Expr::Sym(y)) => Expr::Add(x + y),
            (Expr::Sym(x), Expr::Add(y)) => Expr::Add(x + y),
            (Expr::Sym(x), Expr::Mul(y)) => Expr::Add(x + y),
            (Expr::Sym(x), Expr::Pow(y)) => Expr::Add(x + y),
            (Expr::Add(x), Expr::Num(y)) => Expr::Add(x + y),
            (Expr::Add(x), Expr::Sym(y)) => Expr::Add(x + y),
            (Expr::Add(x), Expr::Add(y)) => Expr::Add(x + y),
            (Expr::Add(x), Expr::Mul(y)) => Expr::Add(x + y),
            (Expr::Add(x), Expr::Pow(y)) => Expr::Add(x + y),
            (Expr::Mul(x), Expr::Num(y)) => Expr::Add(x + y),
            (Expr::Mul(x), Expr::Sym(y)) => Expr::Add(x + y),
            (Expr::Mul(x), Expr::Add(y)) => Expr::Add(x + y),
            (Expr::Mul(x), Expr::Mul(y)) => Expr::Add(x + y),
            (Expr::Mul(x), Expr::Pow(y)) => Expr::Add(x + y),
            (Expr::Pow(x), Expr::Num(y)) => Expr::Add(x + y),
            (Expr::Pow(x), Expr::Sym(y)) => Expr::Add(x + y),
            (Expr::Pow(x), Expr::Add(y)) => Expr::Add(x + y),
            (Expr::Pow(x), Expr::Mul(y)) => Expr::Add(x + y),
            (Expr::Pow(x), Expr::Pow(y)) => Expr::Add(x + y),
        }
    }
}

impl<'a> std::ops::Mul<Expr<'a>> for Expr<'a> {
    type Output = Expr<'a>;
    fn mul(self, rhs: Expr<'a>) -> Self::Output {
        match (self, rhs) {
            (Expr::Num(x), Expr::Num(y)) => Expr::Num(x * y),
            (Expr::Num(x), Expr::Sym(y)) => Expr::Mul(x * y),
            (Expr::Num(x), Expr::Add(y)) => Expr::Mul(x * y),
            (Expr::Num(x), Expr::Mul(y)) => Expr::Mul(x * y),
            (Expr::Num(x), Expr::Pow(y)) => Expr::Mul(x * y),
            (Expr::Sym(x), Expr::Num(y)) => Expr::Mul(x * y),
            (Expr::Sym(x), Expr::Sym(y)) => Expr::Mul(x * y),
            (Expr::Sym(x), Expr::Add(y)) => Expr::Mul(x * y),
            (Expr::Sym(x), Expr::Mul(y)) => Expr::Mul(x * y),
            (Expr::Sym(x), Expr::Pow(y)) => Expr::Mul(x * y),
            (Expr::Add(x), Expr::Num(y)) => Expr::Mul(x * y),
            (Expr::Add(x), Expr::Sym(y)) => Expr::Mul(x * y),
            (Expr::Add(x), Expr::Add(y)) => Expr::Mul(x * y),
            (Expr::Add(x), Expr::Mul(y)) => Expr::Mul(x * y),
            (Expr::Add(x), Expr::Pow(y)) => Expr::Mul(x * y),
            (Expr::Mul(x), Expr::Num(y)) => Expr::Mul(x * y),
            (Expr::Mul(x), Expr::Sym(y)) => Expr::Mul(x * y),
            (Expr::Mul(x), Expr::Add(y)) => Expr::Mul(x * y),
            (Expr::Mul(x), Expr::Mul(y)) => Expr::Mul(x * y),
            (Expr::Mul(x), Expr::Pow(y)) => Expr::Mul(x * y),
            (Expr::Pow(x), Expr::Num(y)) => Expr::Mul(x * y),
            (Expr::Pow(x), Expr::Sym(y)) => Expr::Mul(x * y),
            (Expr::Pow(x), Expr::Add(y)) => Expr::Mul(x * y),
            (Expr::Pow(x), Expr::Mul(y)) => Expr::Mul(x * y),
            (Expr::Pow(x), Expr::Pow(y)) => Expr::Mul(x * y),
        }
    }
}

impl<'a> std::ops::BitXor<Expr<'a>> for Expr<'a> {
    type Output = Expr<'a>;
    fn bitxor(self, rhs: Expr<'a>) -> Self::Output {
        match (self, rhs) {
            (Expr::Num(x), Expr::Num(y)) => Expr::Num(x ^ y),
            (Expr::Num(x), Expr::Sym(y)) => Expr::Pow(x ^ y),
            (Expr::Num(x), Expr::Add(y)) => Expr::Pow(x ^ y),
            (Expr::Num(x), Expr::Mul(y)) => Expr::Pow(x ^ y),
            (Expr::Num(x), Expr::Pow(y)) => Expr::Pow(x ^ y),
            (Expr::Sym(x), Expr::Num(y)) => Expr::Pow(x ^ y),
            (Expr::Sym(x), Expr::Sym(y)) => Expr::Pow(x ^ y),
            (Expr::Sym(x), Expr::Add(y)) => Expr::Pow(x ^ y),
            (Expr::Sym(x), Expr::Mul(y)) => Expr::Pow(x ^ y),
            (Expr::Sym(x), Expr::Pow(y)) => Expr::Pow(x ^ y),
            (Expr::Add(x), Expr::Num(y)) => Expr::Pow(x ^ y),
            (Expr::Add(x), Expr::Sym(y)) => Expr::Pow(x ^ y),
            (Expr::Add(x), Expr::Add(y)) => Expr::Pow(x ^ y),
            (Expr::Add(x), Expr::Mul(y)) => Expr::Pow(x ^ y),
            (Expr::Add(x), Expr::Pow(y)) => Expr::Pow(x ^ y),
            (Expr::Mul(x), Expr::Num(y)) => Expr::Pow(x ^ y),
            (Expr::Mul(x), Expr::Sym(y)) => Expr::Pow(x ^ y),
            (Expr::Mul(x), Expr::Add(y)) => Expr::Pow(x ^ y),
            (Expr::Mul(x), Expr::Mul(y)) => Expr::Pow(x ^ y),
            (Expr::Mul(x), Expr::Pow(y)) => Expr::Pow(x ^ y),
            (Expr::Pow(x), Expr::Num(y)) => Expr::Pow(x ^ y),
            (Expr::Pow(x), Expr::Sym(y)) => Expr::Pow(x ^ y),
            (Expr::Pow(x), Expr::Add(y)) => Expr::Pow(x ^ y),
            (Expr::Pow(x), Expr::Mul(y)) => Expr::Pow(x ^ y),
            (Expr::Pow(x), Expr::Pow(y)) => Expr::Pow(x ^ y),
        }
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Sym(sym) => write!(f, "{}", sym),
            Expr::Add(add) => write!(f, "{}", add),
            Expr::Mul(mul) => write!(f, "{}", mul),
            Expr::Num(num) => write!(f, "{}", num),
            Expr::Pow(pow) => write!(f, "{}", pow),
        }
    }
}

#[test]
fn test_expr() {
    use super::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    let pow = Expr::Pow(x ^ y);
    assert_eq!(pow.detach_pow().0.to_string(), "x");
    assert_eq!(pow.detach_pow().1.to_string(), "y");
    assert_eq!(Expr::Sym(x).detach_pow().0.to_string(), "x");
    assert_eq!(Expr::Sym(x).detach_pow().1.to_string(), "1");
}
