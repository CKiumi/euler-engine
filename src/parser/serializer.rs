use crate::expr::{Func, FuncName};
use crate::Expr;

pub fn serialize(expr: &Expr) -> String {
    match expr {
        Expr::Add(add) => {
            let mut result = serialize(&add.exprs[0]);
            (1..add.exprs.len()).for_each(|i| {
                result = match &serialize(&add.exprs[i])[..1] {
                    "-" => format!("{}{}", result, serialize(&add.exprs[i])),
                    _ => format!("{}+{}", result, serialize(&add.exprs[i])),
                }
            });
            result
        }
        Expr::Mul(mul) => {
            let mut result = String::new();
            mul.exprs
                .iter()
                .for_each(|expr| result = format!("{}{}", result, serialize(expr)));
            result
        }
        Expr::Pow(pow) => {
            if let Expr::Func(Func { name, args }) = &*pow.body {
                format!("\\{}^{{{}}}{}", name, pow.pow, lr(serialize(args)))
            } else {
                format!("{}^{{{}}}", serialize(&pow.body), serialize(&pow.pow))
            }
        }
        Expr::Par(paren) => format!("\\left({}\\right)", serialize(&paren.inner)),
        Expr::Sym(x) => format!("{} ", x),
        Expr::Num(x) if x.num == -1 => String::from("-"),
        Expr::Num(x) => x.to_string(),
        Expr::Func(func) => match func.name {
            FuncName::Sqrt => format!("\\sqrt{{{}}}", serialize(&func.args)),
            _ => format!("\\{}{}", func.name, lr(serialize(&func.args))),
        },
    }
}

fn lr(body: String) -> String {
    format!("\\left({body}\\right)")
}
