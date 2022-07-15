mod add;
mod mul;
mod num;
mod ops;
mod par;
mod pow;
pub mod sym;
pub use add::Add;
pub use mul::Mul;
pub use num::Num;
pub use par::Par;
pub use pow::Pow;
use std::fmt::{Display, Formatter, Result};
pub use sym::Sym;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Expr<'a> {
    Num(Num),
    Sym(Sym<'a>),
    Add(Add<'a>),
    Mul(Mul<'a>),
    Pow(Pow<'a>),
    Par(Par<'a>),
}

impl<'a> Expr<'a> {
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
    pub fn detach_coeff(&self) -> (Num, Option<Mul<'a>>) {
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
    fn to_terms(&self) -> Vec<Expr<'a>> {
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
        impl<'a> std::ops::Add<Expr<'a>> for Expr<'a> {
            type Output = Expr<'a>;
            fn add(self, _rhs: Expr<'a>) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
    (@mul, ;$($x:ident)*; $($arm:tt)*) => {
        impl<'a> std::ops::Mul<Expr<'a>> for Expr<'a> {
            type Output = Expr<'a>;
            fn mul(self, _rhs: Expr<'a>) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
    (@pow, ;$($x:ident)*; $($arm:tt)*) => {
        impl<'a> std::ops::BitXor<Expr<'a>> for Expr<'a> {
            type Output = Expr<'a>;
            fn bitxor(self, _rhs: Expr<'a>) -> Self::Output {
                match (self,_rhs){$($arm)*}
            }
        }
    };
}

match_all_pairs!(Num Sym Add Mul Pow Par);

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Expr::Sym(sym) => write!(f, "{}", sym),
            Expr::Add(add) => write!(f, "{}", add),
            Expr::Mul(mul) => write!(f, "{}", mul),
            Expr::Num(num) => write!(f, "{}", num),
            Expr::Pow(pow) => write!(f, "{}", pow),
            Expr::Par(paren) => write!(f, "{}", paren),
        }
    }
}

#[test]
fn test_expr() {
    use super::Sym;
    let x = Sym::new("x");
    let y = Sym::new("y");
    let pow = Expr::Pow(x ^ y);
    assert_eq!(pow.detach_pow().0.to_string(), "x");
    assert_eq!(pow.detach_pow().1.to_string(), "y");
    assert_eq!(Expr::Sym(x).detach_pow().0.to_string(), "x");
    assert_eq!(Expr::Sym(x).detach_pow().1.to_string(), "1");
    assert_eq!(
        Expr::Pow(Pow::new(
            Expr::Par(Par::new(Expr::Add(x + y))),
            Expr::Num(Num::new(2))
        ))
        .expand()
        .to_string(),
        "(x^{2}+2*x*y+y^{2})"
    );
}
