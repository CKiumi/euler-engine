use super::{Add, Expr, Mul, Num, Par, Pow, Sym};
use std::ops::BitXor;

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
        impl BitXor for $x {
            type Output = Pow<'static>;
            fn bitxor(self, rhs: $y) -> Self::Output {
                Pow::new(Expr::$x(self), Expr::$y(rhs))
            }
        }
    };

    (@impl,$x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> BitXor<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Pow<'a>;
            fn bitxor(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Pow::new(Expr::$x(self), Expr::$y(rhs))
            }
        }
    };
}

impl_ops_add!(Sym<'a> Pow<'a> Add<'a> Mul<'a> Par<'a> Num);
