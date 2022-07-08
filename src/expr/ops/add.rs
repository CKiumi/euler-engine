use super::{Add, Expr, Mul, Num, Pow, Sym};
use std::ops;

/// Overload + operator
macro_rules! impl_ops_add_with_add {
    ($($y:ident$(< $lty:tt >)?),*) => {
        $(
           impl_ops_add_with_add!(@left,$y$(< $lty >)?);
           impl_ops_add_with_add!(@right,$y$(< $lty >)?);
        )*
    };

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
}

impl_ops_add_with_add!(Sym<'a>, Pow<'a>, Mul<'a>, Num);

impl<'a> ops::Add<Add<'a>> for Add<'a> {
    type Output = Add<'a>;
    fn add(self, mut _rhs: Add<'a>) -> Add<'a> {
        Add::new([self.exprs, _rhs.exprs].concat())
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
        impl ops::Add for $x {
            type Output = Add<'static>;
            fn add(self, rhs: $y) -> Self::Output {
                Add::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };

    (@impl,$x:ident$(< $ltx:tt >)?;$y:ident$(< $lty:tt >)?) => {
        impl <'a> ops::Add<$y$(<$lty>)?> for $x$(<$ltx>)? {
            type Output = Add<'a>;
            fn add(self, rhs: $y$(< $lty >)?) -> Self::Output {
                Add::new(vec![Expr::$x(self), Expr::$y(rhs)])
            }
        }
    };
}

impl_ops_add!(Sym<'a> Pow<'a> Mul<'a> Num);
