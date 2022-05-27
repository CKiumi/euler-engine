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
use sym::Sym;

#[derive(PartialEq, Eq, Clone)]
pub enum Expr<'a> {
    Num(Num),
    Sym(Sym<'a>),
    Add(Add<'a>),
    Mul(Mul<'a>),
    Pow(Pow<'a>),
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
    use super::{Add, Expr, Mul, Sym};
    #[test]
    fn test_fmt() {
        println!("{}", sym!("x"));
        println!("{}", add![sym!("x"), sym!("x"), sym!("x")]);
        println!("{}", mul![sym!("x"), sym!("x"), sym!("x")]);
    }
}
