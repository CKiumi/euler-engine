mod add;
mod frac;
mod func;
mod mul;
mod num;
mod ops;
mod par;
mod pow;
pub mod sym;
pub use add::Add;
pub use frac::Frac;
pub use func::{Func, FuncName};
pub use mul::Mul;
pub use num::Num;
pub use par::Par;
pub use pow::Pow;
use std::fmt::{Display, Formatter, Result};
pub use sym::Sym;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Expr {
    Num(Num),
    Sym(Sym),
    Add(Add),
    Mul(Mul),
    Pow(Pow),
    Par(Par),
    Func(Func),
    Frac(Frac),
}

pub trait ToExpr {
    fn to_expr(self) -> Expr;
}

impl Expr {
    fn collect(&self) -> Self {
        match self {
            Expr::Add(add) => Expr::Add(add.collect()),
            Expr::Mul(mul) => Expr::Mul(mul.collect()),
            x => x.clone(),
        }
    }

    pub fn expand(&self) -> Self {
        match self {
            Expr::Pow(pow) => match (&pow.body, &pow.pow) {
                (box Expr::Par(par), box Expr::Num(num)) if num.num > 0 => {
                    let res = Mul::new(vec![Expr::Par(par.clone()); num.num as usize]);
                    Expr::Par(res.expand())
                }
                _ => unimplemented!(),
            },
            Expr::Mul(mul) => Expr::Par(mul.expand()),
            _ => unimplemented!(),
        }
    }

    /// Pow(x,y)->(x,y) otherwise expr->(expr,1)
    fn detach_pow(&self) -> (Self, Self) {
        match self {
            Expr::Pow(pow) => (*pow.body.clone(), *pow.pow.clone()),
            _ => (self.clone(), Expr::Num(Num::new(1))),
        }
    }

    /// Multi(num,x1,...,xn)->(num,Multi(x1,...,xn)) otherwise expr->(1,expr)
    pub fn detach_coeff(&self) -> (Num, Option<Mul>) {
        match self {
            Expr::Mul(mul) => match mul.exprs[0] {
                Expr::Num(x) => (x, Some(Mul::new(mul.exprs[1..].to_vec()))),
                _ => (Num::new(1), Some(Mul::new(mul.exprs.clone()))),
            },
            Expr::Num(num) => (*num, None),
            x => (Num::new(1), Some(Mul::new(vec![x.clone()]))),
        }
    }

    /// Paren(Add(x1,...x_n))->[x1,...x_n] ,Paren(expr)->[expr]
    /// Add(x1,...x_n)->[x1,...x_n], otherwise expr -> [expr]
    fn to_terms(&self) -> Vec<Expr> {
        match self {
            Expr::Par(p) => match *p.inner.clone() {
                Expr::Add(a) => a.exprs,
                p => vec![p],
            },
            Expr::Add(a) => a.exprs.clone(),
            _ => vec![self.clone()],
        }
    }
}

macro_rules! match_all_pairs {
    ($($x:ident)*) => {
        match_all_pairs!(@add,$($x)*; $($x)*;);
        match_all_pairs!(@mul,$($x)*; $($x)*;);
        match_all_pairs!(@pow,$($x)*; $($x)*;);
    };
    (@add, $head:ident $($tail:ident)*; $($x:ident)*;$($arm:tt)*) => {
        match_all_pairs!(@add, $($tail)*; $($x)*;
        $($arm)* $((Expr::$head(x), Expr::$x(y))=>{Expr::Add(x+y)},)*);
    };
    (@mul, $head:ident $($tail:ident)*; $($x:ident)*;$($arm:tt)*) => {
        match_all_pairs!(@mul, $($tail)*; $($x)*;
        $($arm)* $((Expr::$head(x), Expr::$x(y))=>{Expr::Mul(x*y)},)*);
    };
    (@pow, $head:ident $($tail:ident)*; $($x:ident)*;$($arm:tt)*) => {
        match_all_pairs!(@pow, $($tail)*; $($x)*;
        $($arm)* $((Expr::$head(x), Expr::$x(y))=>{Expr::Pow(x^y)},)*);
    };
    (@add, ;$($x:ident)*; $($arm:tt)*) => {
        impl std::ops::Add<Expr> for Expr {
            type Output = Expr;
            fn add(self, _rhs: Expr) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
    (@mul, ;$($x:ident)*; $($arm:tt)*) => {
        impl std::ops::Mul<Expr> for Expr {
            type Output = Expr;
            fn mul(self, _rhs: Expr) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
    (@pow, ;$($x:ident)*; $($arm:tt)*) => {
        impl std::ops::BitXor<Expr> for Expr {
            type Output = Expr;
            fn bitxor(self, _rhs: Expr) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
}

match_all_pairs!(Num Sym Add Mul Pow Par Func Frac);

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Sym(sym) => write!(f, "{}", sym),
            Expr::Add(add) => write!(f, "{}", add),
            Expr::Mul(mul) => write!(f, "{}", mul),
            Expr::Num(num) => write!(f, "{}", num),
            Expr::Pow(pow) => write!(f, "{}", pow),
            Expr::Func(func) => write!(f, "{}", func),
            Expr::Par(paren) => write!(f, "{}", paren),
            Expr::Frac(frac) => write!(f, "{}", frac),
        }
    }
}

#[cfg(test)]
pub mod test_util {
    use super::*;
    pub fn x() -> Sym {
        Sym::new("x")
    }
    pub fn y() -> Sym {
        Sym::new("y")
    }
    pub fn z() -> Sym {
        Sym::new("z")
    }
    pub fn asrt<T: ToString, S: AsRef<str>>(x: T, y: S) {
        assert_eq!(x.to_string(), y.as_ref());
    }
}

#[test]
fn test_expr() {
    use test_util::*;
    let pow = Expr::Pow(x() ^ y());
    asrt(pow.detach_pow().0, "x");
    asrt(pow.detach_pow().1, "y");
    asrt(Expr::Sym(x()).detach_pow().0, "x");
    asrt(Expr::Sym(x()).detach_pow().1, "1");
    asrt(
        Expr::Pow(Pow::new(
            Expr::Par(Par::new(Expr::Add(x() + y()))),
            Expr::Num(Num::new(2)),
        ))
        .expand(),
        "(x^{2}+2*x*y+y^{2})",
    );
}
