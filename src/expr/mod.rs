mod add;
mod mul;
mod num;
mod pow;
pub mod sym;
use add::Add;
use mul::Mul;
use num::Num;
use pow::Pow;
use std::fmt::{Display, Formatter, Result};
use std::ops;
use sym::Sym;

#[derive(PartialEq, Eq, Clone)]
pub enum Expr<'a> {
    Num(Num),
    Sym(Sym<'a>),
    Add(Add<'a>),
    Mul(Mul<'a>),
    Pow(Pow<'a>),
}

impl<'a> ops::Add<Expr<'a>> for Expr<'a> {
    type Output = Expr<'a>;
    fn add(self, _rhs: Expr<'a>) -> Expr {
        match self {
            Expr::Num(x) => match _rhs {
                Expr::Num(y) => Expr::Num(Num::new(x.num + y.num)),
                Expr::Add(add) => Expr::Add(Add::new(vec![vec![Expr::Num(x)], add.exprs].concat())),
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

#[macro_export]
macro_rules! sym {
    ( $x:expr ) => {{
        Expr::Sym(Sym::new($x))
    }};
}

#[macro_export]
macro_rules! add {
    (  $( $x:expr ),* ) => {{
        let mut exprs = Vec::new();
        $(
            exprs.push($x);
        )*
        Expr::Add(Add::new(exprs))
    }};
}

#[macro_export]
macro_rules! mul {
    (  $( $x:expr ),* ) => {{
        let mut exprs = Vec::new();
        $(
            exprs.push($x);
        )*
        Expr::Mul(Mul::new(exprs))
    }};
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

#[cfg(test)]
mod test_expr {
    #[test]
    fn test_fmt() {}
}
