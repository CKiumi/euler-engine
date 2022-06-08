use super::{Add, Expr, Mul, Num, Pow, Sym};
use std::ops::BitXor;

/// Overload ^ operator
macro_rules! impl_ops_pow {
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> BitXor<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Pow<'a>;
            fn bitxor(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Pow::new(Expr::$x(self), Expr::$y(rhs))
            }
        }
    };
    ($x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?,$($z:ident$(< $ltz:tt >)?),*) => {
            impl_ops_pow!($x$(< $ltx >)?;$y$(< $lty >)?);
            impl_ops_pow!($x$(< $ltx >)?;$($z$(< $ltz >)?),*);
    };
}

impl_ops_pow!(Sym<'a>; Sym<'a>,Pow<'a>,Add<'a>,Mul<'a>,Num);
impl_ops_pow!(Add<'a>; Sym<'a>,Pow<'a>,Add<'a>,Mul<'a>,Num);
impl_ops_pow!(Mul<'a>; Sym<'a>,Pow<'a>,Add<'a>,Mul<'a>,Num);
impl_ops_pow!(Pow<'a>; Sym<'a>,Pow<'a>,Add<'a>,Mul<'a>,Num);
impl_ops_pow!(Num; Sym<'a>,Pow<'a>,Add<'a>,Mul<'a>);

impl BitXor<Num> for Num {
    type Output = Num;
    fn bitxor<'a>(self, rhs: Num) -> Self::Output {
        Num::new(self.num.pow(rhs.num as u32))
    }
}
