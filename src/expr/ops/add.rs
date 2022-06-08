use super::{Add, Expr, Mul, Num, Pow, Sym};
use std::ops;

/// Overload + operator
macro_rules! impl_ops_add_with_add {
    (@left,$x:ident$(< $ltx:tt >)?) => {
        impl<'a> ops::Add<$x$(< $ltx >)?> for Add<'a> {
            type Output = Add<'a>;
            fn add(self, mut _rhs: $x$(< $ltx >)?) -> Add<'a> {
                Add::new([&self.exprs, &[Expr::$x(_rhs)][..]].concat())
            }
        }
    };
    (@right,$x:ident$(< $ltx:tt >)?) => {
        impl<'a> ops::Add<Add<'a>> for $x$(< $ltx >)? {
            type Output = Add<'a>;
            fn add(self, mut _rhs: Add<'a>) -> Add<'a> {
                Add::new([&[Expr::$x(self)][..],&_rhs.exprs].concat())
            }
        }
    };
    ($($y:ident$(< $lty:tt >)?),*) => {
     $(
        impl_ops_add_with_add!(@left,$y$(< $lty >)?);
        impl_ops_add_with_add!(@right,$y$(< $lty >)?);
     )*
};
}
impl_ops_add_with_add!(Sym<'a>, Pow<'a>, Mul<'a>, Num);

macro_rules! impl_ops_add {
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> ops::Add<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Add<'a>;
            fn add(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Add::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?,$($z:ident$(< $ltz:tt >)?),*) => {
            impl_ops_add!($x$(< $ltx >)?;$y$(< $lty >)?);
            impl_ops_add!($x$(< $ltx >)?;$($z$(< $ltz >)?),*);
    };
}

impl_ops_add!(Sym<'a>; Sym<'a>,Pow<'a>,Mul<'a>,Num);
impl_ops_add!(Mul<'a>; Sym<'a>,Pow<'a>,Mul<'a>,Num);
impl_ops_add!(Pow<'a>; Sym<'a>,Pow<'a>,Mul<'a>,Num);
impl_ops_add!(Num; Sym<'a>,Pow<'a>,Mul<'a>);

/// Num + Num
impl ops::Add<Num> for Num {
    type Output = Num;
    fn add(self, _rhs: Num) -> Num {
        Num::new(self.num + _rhs.num)
    }
}
