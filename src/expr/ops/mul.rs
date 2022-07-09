use super::{Add, Expr, Mul, Num, Par, Pow, Sym};
use std::ops;
/// Overload * operator
macro_rules! impl_ops_mul_with_mul {
    ($($y:ident$(< $lty:tt >)?),*) => {
        $(
           impl_ops_mul_with_mul!(@left,$y$(< $lty >)?);
           impl_ops_mul_with_mul!(@right,$y$(< $lty >)?);
        )*
    };

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
}
impl_ops_mul_with_mul!(Sym<'a>, Pow<'a>, Add<'a>, Par<'a>, Num);

impl<'a> ops::Mul<Mul<'a>> for Mul<'a> {
    type Output = Mul<'a>;
    fn mul(self, rhs: Mul<'a>) -> Self::Output {
        Mul::new(vec![self.exprs, rhs.exprs].concat())
    }
}

macro_rules! impl_ops_add {
    ($($x:ident$(< $ltx:tt >)?)*) => {
        impl_ops_add!(@step1, $($x$(< $ltx >)?)*; $($x$(< $ltx >)?)*);
    };

    (@step1,$head:ident$(< $lth:tt >)?$($tail:ident$(< $ltt:tt >)?)* ;$($y:ident$(< $lty:tt >)?)*) => {
        impl_ops_add!(@step1,$($tail$(< $ltt >)?)* ;$($y$(< $lty >)?)*);
        impl_ops_add!(@step2,$head$(< $lth >)?;$($y$(< $lty >)?)*);
    };

    (@step1, ;$($y:ident$(< $lty:tt >)?)*) => {};

    (@step2,$x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?$($z:ident$(< $ltz:tt >)?)*) => {
        impl_ops_add!(@step2,$x$(< $ltx >)?;$($z$(< $ltz >)?)*);
        impl_ops_add!(@impl,$x$(< $ltx >)?;$y$(< $lty >)?);
    };

    (@step2,$x:ident$(< $ltx:tt >)?;)=>{};

    //Num + Num
    (@impl,$x:ident;$y:ident) => {
        impl ops::Mul for $x {
            type Output = Mul<'static>;
            fn mul(self, rhs: $y) -> Self::Output {
                Mul::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };

    (@impl,$x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> ops::Mul<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Mul<'a>;
            fn mul(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Mul::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
}

impl_ops_add!(Sym<'a> Pow<'a> Add<'a> Par<'a> Num);

#[test]
fn test_mul_ops() {
    use super::{Num, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let n = Num::new(1);
    assert_eq!((x * y * n * y).to_string(), "x*y*y");
    assert_eq!(((x * y) * (n * y)).to_string(), "x*y*y");
}
