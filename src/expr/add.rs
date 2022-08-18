use super::Expr;
use crate::Num;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct Add<'a> {
    pub exprs: Vec<Expr<'a>>,
}

impl<'a> Add<'a> {
    pub fn new(exprs: Vec<Expr<'a>>) -> Self {
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
        let mut result = vec![self.exprs[0].collect().clone()];
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
                                Expr::Mul(Num::new(x) * mul.clone())
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

impl<'a> Display for Add<'a> {
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
    use super::super::{Num, Sym};
    #[test]
    fn test_fmt() {
        let x = Sym::new("x");
        let y = Sym::new("y");
        let z = Sym::new("z");
        let n2 = Num::new(2);
        let n0 = Num::new(0);

        assert_eq!((x + y).to_string(), "x+y");
        assert_eq!((x + n0 + y).to_string(), "x+y");

        assert_eq!((x + y + y).to_string(), "x+y+y");
        assert_eq!((n2 * x + y).to_string(), "2*x+y");

        let test = x + y + x * y;
        assert_eq!(test.to_string(), "x+y+x*y");

        assert_eq!(((x + y) ^ x).to_string(), "(x+y)^{x}");
        assert_eq!((((x ^ y) + y + x) ^ x).to_string(), "(x^{y}+y+x)^{x}");
        assert_eq!((n2 + n2).collect().to_string(), "4");
        assert_eq!((x + y + y).collect().to_string(), "x+2*y");
        assert_eq!((x + y + z).collect().to_string(), "x+y+z");
        assert_eq!(
            (x + (y ^ n2) + (y ^ n2) * n2).collect().to_string(),
            "x+3*y^{2}"
        );
        assert_eq!((n2 * x + x + y).collect().to_string(), "3*x+y");
    }
}
