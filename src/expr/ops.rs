use super::{Add, Expr, Mul, Num, Pow, Sym};
use std::ops::{self, BitXor};

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

impl ops::Add<Num> for Num {
    type Output = Num;
    fn add(self, _rhs: Num) -> Num {
        Num::new(self.num + _rhs.num)
    }
}

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
