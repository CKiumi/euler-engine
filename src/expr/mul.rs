use super::{num::Num, pow::Pow, Expr};
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone)]
pub struct Mul<'a> {
    pub exprs: Vec<Expr<'a>>,
}

impl<'a> Mul<'a> {
    pub fn new(exprs: Vec<Expr<'a>>) -> Self {
        Mul { exprs }
    }

    /// Mul(x1,x1,x3)->Mul(Pow(x1,2),x3) otherwise crash
    fn to_pow(&self) -> Self {
        let mut result = vec![self.exprs[0].clone()];
        (1..self.exprs.len()).for_each(|i| {
            for j in 0..result.len() {
                let (body1, pow1) = self.exprs[i].detach_pow();
                let (body2, pow2) = result[j].detach_pow();
                if body1 == body2 {
                    result[j] = Expr::Pow(Pow::new(body1, pow1 + pow2));
                    break;
                } else if j == result.len() - 1 {
                    result.push(self.exprs[i].clone())
                }
            }
        });
        Mul::new(result)
    }

    /// Multi(x1,x1,num1,x3,num2)->Multi(num1*num2,Pow(x1,2),x3) otherwise Expr->Expr
    /// depend on multi_to_pow
    pub fn collect(&self) -> Self {
        let mut coef = Expr::Num(Num::new(1));
        let mut body = vec![Expr::Num(Num::new(1))];
        self.exprs.iter().for_each(|expr| match expr {
            Expr::Num(_) => coef = coef.clone() * (*expr).clone(),
            expr => body.push(expr.clone()),
        });
        body[0] = coef;
        Mul::new(body).to_pow()
    }
}

impl<'a> Display for Mul<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = self.exprs[0].to_string();
        for i in 1..self.exprs.len() {
            result = format!("{}*{}", result, self.exprs[i]);
        }
        write!(f, "{}", result)
    }
}

#[test]
fn test_mul() {
    use super::{Num, Sym};
    let x = Sym::new("x");
    let y = Sym::new("y");
    let n3 = Num::new(3);
    assert_eq!((x * y * y).to_pow().to_string(), "x*y^2");
    assert_eq!((x * y * n3 * y).collect().to_string(), "3*x*y^2");
}
