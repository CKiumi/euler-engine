use super::{Add, Expr, Num, Par, Pow, ToExpr};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Mul {
    pub exprs: Vec<Expr>,
}

impl ToExpr for Mul {
    fn to_expr(self) -> Expr {
        Expr::Mul(self)
    }
}

impl Mul {
    /// [x1,x2,1,x3]->Mul(x1,x2,x3)
    pub fn new(exprs: Vec<Expr>) -> Self {
        Mul {
            exprs: exprs
                .into_iter()
                .filter(|x| {
                    if let Expr::Num(n) = x {
                        n.num != 1
                    } else {
                        true
                    }
                })
                .collect::<Vec<Expr>>(),
        }
    }

    /// Mul(x1,x1,x3)->Mul(Pow(x1,2),x3) otherwise crash
    fn to_pow(&self) -> Self {
        let mut result = vec![self.exprs[0].clone()];
        (1..self.exprs.len()).for_each(|i| {
            for j in 0..result.len() {
                let (body1, pow1) = self.exprs[i].detach_pow();
                let (body2, pow2) = result[j].detach_pow();
                if body1 == body2 {
                    result[j] = Expr::Pow(Pow::new(body1, (pow1 + pow2).collect()));
                    break;
                } else if j == result.len() - 1 {
                    result.push(self.exprs[i].clone())
                }
            }
        });
        Mul::new(result)
    }

    pub fn expand(&self) -> Par {
        let mut res = self.exprs[0].to_terms();
        for i in 1..self.exprs.len() {
            let exprs1 = res.clone();
            let exprs2 = self.exprs[i].to_terms();
            res = vec![];
            for e1 in exprs1 {
                for e2 in &exprs2 {
                    res.push(e1.clone() * e2.clone());
                }
            }
        }
        Par::new(Expr::Add(Add::new(res).collect()))
    }

    /// Multi(x1,x1,num1,x3,num2)->Multi(num1*num2,Pow(x1,2),x3) otherwise Expr->Expr
    /// depend on multi_to_pow
    pub fn collect(&self) -> Self {
        let mut coef = Num::new(1);
        let mut body = vec![Expr::Num(Num::new(1))];
        self.exprs.iter().for_each(|expr| match expr {
            Expr::Num(n) => coef = Num::new(coef.num * n.num),
            expr => body.push(expr.clone()),
        });
        body[0] = Expr::Num(coef);
        let mut res = Mul::new(body).to_pow();
        res.exprs.sort();
        res
    }
}

impl Display for Mul {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = String::new();
        for expr in &self.exprs {
            result = if let Expr::Add(_) = expr {
                format!("{}*({})", result, expr)
            } else {
                format!("{}*{}", result, expr)
            };
        }
        write!(f, "{}", &result[1..])
    }
}

#[test]
fn test_mul() {
    use super::test_util::*;
    use super::{Num, Par};
    let n3 = Num::new(3);
    asrt((x() * y() * y()).to_pow(), "x*y^{2}");
    asrt((x() * y() * n3 * y()).collect(), "3*x*y^{2}");
    let par = Par::from(x() + y());
    asrt((x() * par.clone()).expand(), "(x^{2}+x*y)");
    asrt((par.clone() * x()).expand(), "(x^{2}+x*y)");
    asrt((n3 * par.clone()).expand(), "(3*x+3*y)");
    asrt((par.clone() * par.clone()).expand(), "(x^{2}+2*x*y+y^{2})");
    asrt(
        (par.clone() * par.clone() * par).expand(),
        "(x^{3}+3*y*x^{2}+3*x*y^{2}+y^{3})",
    );
}
