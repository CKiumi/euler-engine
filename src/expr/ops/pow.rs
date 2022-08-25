use super::{Add, Expr, Func, Mul, Num, Par, Pow, Sym};
use std::ops::BitXor;

macro_rules! impl_ops_add {
    ($($x:ident)*) => {
        impl_ops_add!(@step1, $($x)*; $($x)*);
    };

    (@step1,$head:ident$(< $lth:tt >)?$($tail:ident)* ;$($y:ident)*) => {
        impl_ops_add!(@step1,$($tail)* ;$($y)*);
        impl_ops_add!(@step2,$head;$($y)*);
    };

    (@step1, ;$($y:ident)*) => {};

    (@step2,$x:ident;$y:ident$($z:ident$(< $ltz:tt >)?)*) => {
        impl_ops_add!(@step2,$x;$($z)*);
        impl_ops_add!(@impl,$x;$y);
    };

    (@step2,$x:ident;)=>{};

    (@impl,$x:ident;$y:ident) => {
        impl  BitXor<$y> for $x {
            type Output = Pow;
            fn bitxor(self, rhs: $y) -> Self::Output {
                Pow::new(Expr::$x(self), Expr::$y(rhs))
            }
        }
    };
}

impl_ops_add!(Sym Pow Add Mul Par Num Func);
