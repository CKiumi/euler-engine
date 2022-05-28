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
    /// Pow(x,y)->(x,y) otherwise expr->(expr,1)
    fn collect(&self) -> Self {
        match self {
            // Expr::Add(add) => Expr::Add(add.collect()),
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
        match self {
            Expr::Num(x) => match _rhs {
                Expr::Num(y) => Expr::Num(x + y),
                Expr::Add(add) => Expr::Add(x + add),
                _ => Expr::Add(Add::new(vec![Expr::Num(x), _rhs])),
            },
            Expr::Add(add1) => match _rhs {
                Expr::Add(add2) => Expr::Add(Add::new(vec![add1.exprs, add2.exprs].concat())),
                _rhs => Expr::Add(Add::new(vec![add1.exprs, vec![_rhs]].concat())),
            },
            expr => match _rhs {
                Expr::Add(add) => Expr::Add(Add::new(vec![vec![expr], add.exprs].concat())),
                _ => Expr::Add(Add::new(vec![expr, _rhs])),
            },
        }
    }
}

impl<'a> std::ops::Mul<Expr<'a>> for Expr<'a> {
    type Output = Expr<'a>;
    fn mul(self, rhs: Expr<'a>) -> Self::Output {
        match self {
            Expr::Num(x) if x.num == 1 => rhs,
            Expr::Num(x) => match rhs {
                Expr::Num(y) => Expr::Num(Num::new(x.num * y.num)),
                Expr::Mul(mul) => Expr::Mul(Mul::new(vec![vec![Expr::Num(x)], mul.exprs].concat())),
                _ => Expr::Mul(Mul::new(vec![Expr::Num(x), rhs])),
            },

            Expr::Mul(mul1) => match rhs {
                Expr::Mul(mul2) => Expr::Mul(Mul::new(vec![mul1.exprs, mul2.exprs].concat())),
                exp => Expr::Mul(Mul::new(vec![mul1.exprs, vec![exp]].concat())),
            },
            expr => match rhs {
                Expr::Mul(mul) => Expr::Mul(Mul::new(vec![vec![expr], mul.exprs].concat())),
                _ => Expr::Mul(Mul::new(vec![expr, rhs])),
            },
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
