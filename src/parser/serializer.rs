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
        Expr::Pow(pow) => format!("{}^{{{}}}", serialize(&pow.body), serialize(&pow.pow)),
        Expr::Par(paren) => format!("\\left({}\\right)", serialize(&paren.inner)),
        Expr::Sym(x) => format!("{} ", x),
        Expr::Num(x) if x.num == -1 => String::from("-"),
        Expr::Num(x) => x.to_string(),
    }
}
