use super::{Expr, ToExpr};
use crate::Num;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Add {
    pub exprs: Vec<Expr>,
}

impl ToExpr for Add {
    fn to_expr(self) -> Expr {
        Expr::Add(self)
    }
}

impl Add {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Add {
            exprs: exprs
                .into_iter()
                .filter(|x| {
                    if let Expr::Num(n) = x {
                        n.num != 0
                    } else {
                        true
                    }
                })
                .collect::<Vec<Expr>>(),
        }
    }

    /// Add(x1,x1,num1,x3)->Add(Multi(2,x1),num1,x3) otherwise crash
    /// multi expr will be collected beforehand
    /// depend on col_multi
    pub fn collect(&self) -> Self {
        let mut result = vec![self.exprs[0].collect()];
        (1..self.exprs.len()).for_each(|i| {
            for j in 0..result.len() {
                let (co1, body1) = self.exprs[i].collect().detach_coeff();
                let (co2, body2) = result[j].collect().detach_coeff();
                if body1 == body2 {
                    match co1.num + co2.num {
                        x if x == 0 => {
                            result.remove(j);
                        }
                        x => {
                            result[j] = if let Some(mul) = body1 {
                                Expr::Mul(Num::new(x) * mul)
                            } else {
                                Expr::Num(Num::new(x))
                            }
                        }
                    };
                    break;
                } else if j == result.len() - 1 {
                    result.push(self.exprs[i].collect().clone());
                }
            }
        });
        Add::new(result)
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = self.exprs[0].to_string();
        for i in 1..self.exprs.len() {
            result = format!("{}+{}", result, self.exprs[i]);
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod test_add {
    use super::super::test_util::*;
    use super::super::Num;
    #[test]
    fn test_fmt() {
        let n2 = Num::new(2);
        let n0 = Num::new(0);
        asrt(x() + y(), "x+y");
        asrt(x() + n0 + y(), "x+y");
        asrt(x() + y() + y(), "x+y+y");
        asrt(n2 * x() + y(), "2*x+y");
        let test = x() + y() + x() * y();
        asrt(test, "x+y+x*y");
        asrt((x() + y()) ^ x(), "(x+y)^{x}");
        asrt(((x() ^ y()) + y() + x()) ^ x(), "(x^{y}+y+x)^{x}");
        asrt((n2 + n2).collect(), "4");
        asrt((x() + y() + y()).collect(), "x+2*y");
        asrt((x() + y() + z()).collect(), "x+y+z");
        asrt((x() + (y() ^ n2) + (y() ^ n2) * n2).collect(), "x+3*y^{2}");
        asrt((n2 * x() + x() + y()).collect(), "3*x+y");
    }
}
