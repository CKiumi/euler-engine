use super::{Add, Expr, Mul, Num, Pow, Sym};
use std::ops;
/// Overload * operator
macro_rules! impl_ops_mul_with_mul {
    (@left,$x:ident$(< $ltx:tt >)?) => {
        impl<'a> ops::Mul<$x$(< $ltx >)?> for Mul<'a> {
            type Output = Mul<'a>;
            fn mul(self, mut _rhs: $x$(< $ltx >)?) -> Mul<'a> {
                Mul::new([&self.exprs, &[Expr::$x(_rhs)][..]].concat())
            }
        }
    };
    (@right,$x:ident$(< $ltx:tt >)?) => {
        impl<'a> ops::Mul<Mul<'a>> for $x$(< $ltx >)? {
            type Output = Mul<'a>;
            fn mul(self, mut _rhs: Mul<'a>) -> Mul<'a> {
                Mul::new([&[Expr::$x(self)][..],&_rhs.exprs].concat())
            }
        }
    };
    ($($y:ident$(< $lty:tt >)?),*) => {
     $(
        impl_ops_mul_with_mul!(@left,$y$(< $lty >)?);
        impl_ops_mul_with_mul!(@right,$y$(< $lty >)?);
     )*
};
}
impl_ops_mul_with_mul!(Sym<'a>, Pow<'a>, Add<'a>, Num);

macro_rules! impl_ops_mul {
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> ops::Mul<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Mul<'a>;
            fn mul(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Mul::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?,$($z:ident$(< $ltz:tt >)?),*) => {
        impl_ops_mul!($x$(< $ltx >)?;$y$(< $lty >)?);
        impl_ops_mul!($x$(< $ltx >)?;$($z$(< $ltz >)?),*);
    };
}

impl_ops_mul!(Sym<'a>; Sym<'a>,Pow<'a>,Add<'a>,Num);
impl_ops_mul!(Add<'a>; Sym<'a>,Pow<'a>,Add<'a>,Num);
impl_ops_mul!(Pow<'a>; Sym<'a>,Pow<'a>,Add<'a>,Num);
impl_ops_mul!(Num; Sym<'a>,Pow<'a>,Add<'a>);

impl ops::Mul<Num> for Num {
    type Output = Num;
    fn mul<'a>(self, rhs: Num) -> Self::Output {
        Num::new(self.num * rhs.num)
    }
}

impl<'a> ops::Mul<Mul<'a>> for Mul<'a> {
    type Output = Mul<'a>;
    fn mul(self, rhs: Mul<'a>) -> Self::Output {
        Mul::new(vec![self.exprs, rhs.exprs].concat())
    }
}

#[test]
fn test_mul_ops() {
    use super::{Num, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let n = Num::new(1);
    assert_eq!((x * y * n * y).to_string(), "x*y*y");
    assert_eq!(((x * y) * (n * y)).to_string(), "x*y*y");
}
