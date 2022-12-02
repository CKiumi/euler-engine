use super::{Add, Expr, Frac, Func, Gate, Ket, Mat, Mul, Num, Par, Pow, Sym, Tensor};
use std::ops;
/// Overload * operator
macro_rules! impl_ops_mul_with_mul {
    ($($x:ident),*) => {
        $(
            impl ops::Mul<$x> for Mul {
                type Output = Mul;
                fn mul(self, mut _rhs: $x) -> Mul {
                    Mul::new([&self.exprs, &[Expr::$x(_rhs)][..]].concat())
                }
            }
            impl ops::Mul<Mul> for $x {
                type Output = Mul;
                fn mul(self, mut _rhs: Mul) -> Mul {
                    Mul::new([&[Expr::$x(self)][..],&_rhs.exprs].concat())
                }
            }
        )*
    };
}
impl_ops_mul_with_mul!(Sym, Pow, Add, Par, Num, Func, Frac, Mat, Tensor, Ket, Gate);

impl ops::Mul<Mul> for Mul {
    type Output = Mul;
    fn mul(self, rhs: Mul) -> Self::Output {
        Mul::new(vec![self.exprs, rhs.exprs].concat())
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
        impl  ops::Mul<$y> for $x {
            type Output = Mul;
            fn mul(self, rhs: $y) -> Self::Output {
                Mul::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
}

impl_ops_add!(Sym Pow Add Par Num Func Frac Mat Tensor Ket Gate);

#[test]
fn test_mul_ops() {
    use super::Num;
    use crate::expr::test_util::*;
    let n = Num::new(1);
    asrt(x() * y() * n * y(), "x*y*y");
    asrt((x() * y()) * (n * y()), "x*y*y");
}
