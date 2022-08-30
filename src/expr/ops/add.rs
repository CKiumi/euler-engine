use super::{Add, Expr, Frac, Func, Mat, Mul, Num, Par, Pow, Sym};
use std::ops;

/// Overload + operator
macro_rules! impl_ops_add_with_add {
    ($($x:ident),*) => {
        $(
            impl ops::Add<$x> for Add {
                type Output = Add;
                fn add(self, mut _rhs: $x) -> Add {
                    Add::new([&self.exprs, &[Expr::$x(_rhs)][..]].concat())
                }
            }
            impl ops::Add<Add> for $x {
                type Output = Add;
                fn add(self, mut _rhs: Add) -> Add {
                    Add::new([&[Expr::$x(self)][..],&_rhs.exprs].concat())
                }
            }
        )*
    };
}

impl_ops_add_with_add!(Sym, Pow, Mul, Par, Num, Func, Frac, Mat);

impl ops::Add<Add> for Add {
    type Output = Add;
    fn add(self, mut _rhs: Add) -> Add {
        Add::new([self.exprs, _rhs.exprs].concat())
    }
}

macro_rules! impl_ops_add {
    ($($x:ident)*) => {
        impl_ops_add!(@step1, $($x)*; $($x)*);
    };

    (@step1,$head:ident$($tail:ident)* ;$($y:ident)*) => {
        impl_ops_add!(@step1,$($tail)* ;$($y)*);
        impl_ops_add!(@step2,$head;$($y)*);
    };

    (@step1, ;$($y:ident)*) => {};

    (@step2,$x:ident;$y:ident$($z:ident)*) => {
        impl_ops_add!(@step2,$x;$($z)*);
        impl_ops_add!(@impl,$x;$y);
    };

    (@step2,$x:ident;)=>{};

    (@impl,$x:ident;$y:ident) => {
        impl ops::Add<$y> for $x {
            type Output = Add;
            fn add(self, rhs: $y) -> Self::Output {
                Add::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
}

impl_ops_add!(Sym Pow Mul Par Num Func Frac Mat);
