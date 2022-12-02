use crate::expr::{Func, FuncName, GateName, Tensor};
use crate::{Expr, Mul, Num};

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
        Expr::Mul(Mul { exprs }) => {
            if exprs.len() == 1 {
                return serialize(&exprs[0]);
            }
            let mut result = match exprs[0] {
                Expr::Num(num) if num == Num::new(-1) => "-".to_string(),
                _ => serialize(&exprs[0]),
            };
            for expr in &exprs[1..] {
                result.push_str(&serialize(expr));
            }
            result
        }
        Expr::Tensor(Tensor { exprs }) => {
            if exprs.len() == 1 {
                return serialize(&exprs[0]);
            }
            let mut result = match exprs[0] {
                Expr::Num(num) if num == Num::new(-1) => "-".to_string(),
                _ => serialize(&exprs[0]),
            };
            for expr in &exprs[1..] {
                result.push_str(&serialize(expr));
            }
            result
        }
        Expr::Pow(pow) => match &*pow.body {
            Expr::Func(Func { name, args }) => {
                if name == &FuncName::Sqrt {
                    format!("\\{}{{{}}}^{{{}}}", name, serialize(args), pow.pow)
                } else {
                    format!("\\{}^{{{}}}{}", name, pow.pow, lr(serialize(args)))
                }
            }
            Expr::Frac(_) => format!("{}^{{{}}}", lr(serialize(&pow.body)), serialize(&pow.pow)),
            _ => format!("{}^{{{}}}", serialize(&pow.body), serialize(&pow.pow)),
        },
        Expr::Gate(gate) => match gate.name {
            GateName::H(qbit) => format!("H_{{{}}}", qbit),
            GateName::I(qbit) => format!("H_{{{}}}", qbit),
            GateName::X(qbit) => format!("H_{{{}}}", qbit),
            GateName::Y(qbit) => format!("H_{{{}}}", qbit),
            GateName::Z(qbit) => format!("H_{{{}}}", qbit),
            GateName::S(qbit) => format!("H_{{{}}}", qbit),
            GateName::T(qbit) => format!("H_{{{}}}", qbit),
        },
        Expr::Par(paren) => format!("\\left({}\\right)", serialize(&paren.inner)),
        Expr::Sym(x) if x.to_string().starts_with('\\') => format!("{} ", x),
        Expr::Sym(x) => format!("{}", x),
        Expr::Num(x) => x.to_string(),
        Expr::Func(func) => match func.name {
            FuncName::Sqrt => format!("\\sqrt{{{}}}", serialize(&func.args)),
            _ => format!("\\{}{}", func.name, lr(serialize(&func.args))),
        },
        Expr::Frac(frac) => format!(
            "\\frac{{{}}}{{{}}}",
            serialize(&frac.numer),
            serialize(&frac.denom)
        ),
        Expr::Mat(mat) => {
            let mut result = "\\begin{pmatrix}".to_string();
            for row in &mat.elems {
                for expr in row {
                    result.push_str(&serialize(expr));
                    result.push_str(" & ");
                }
                result.pop();
                result.pop();
                result.push_str("\\\\");
            }
            result.pop();
            result.pop();
            result.push_str("\\end{pmatrix}");
            result
        }
        Expr::Ket(ket) => format!("\\left|{}\\right>", ket.inner),
    }
}

fn lr(body: String) -> String {
    format!("\\left({body}\\right)")
}

#[test]
fn test_serializer() {
    use crate::expr::test_util::asrt;
    use crate::parser::latex_to_expr;
    let tests = [
        ["aaaa", "aaaa"],
        ["a+a+a", "a+a+a"],
        ["-a+b", "-a+b"],
        ["x-(a+b)", "x-\\left(a+b\\right)"],
        ["a-b", "a-b"],
        ["a-1", "a-1"],
        ["a+bc", "a+bc"],
        ["ab+b\\alpha sdas+x", "ab+b\\alpha sdas+x"],
        ["", ""],
        ["b^{a}", "b^{a}"],
        ["b_{a}", "b_{a}"],
        ["b_{a}^{c}", "b_{a}^{c}"],
        ["23a", "23a"],
        ["23a+23a", "23a+23a"],
        ["x^{2}+x^{2}", "x^{2}+x^{2}"],
        ["2x^{2}+2x^{2}", "2x^{2}+2x^{2}"],
        ["2x^{x+y}+2x^{xy}", "2x^{x+y}+2x^{xy}"],
        ["2x_{2}^{2}", "2x_{2}^{2}"],
        ["a_{b}^{c}+d_{e}^{f}", "a_{b}^{c}+d_{e}^{f}"],
        ["a_{b}^{c}+xd_{e}^{f}", "a_{b}^{c}+xd_{e}^{f}"],
        ["(a+b)", "\\left(a+b\\right)"],
        ["\\Re(a+b)", "\\Re\\left(a+b\\right)"],
        ["\\Re^{2}(a+b)", "\\Re^{2}\\left(a+b\\right)"],
        ["x\\Re^{x+y}(a+b)y", "x\\Re^{x+y}\\left(a+b\\right)y"],
        ["\\sqrt{2}", "\\sqrt{2}"],
        ["\\sqrt{2}^{2}", "\\sqrt{2}^{2}"],
        ["\\frac{2}{2}^{2}", "\\left(\\frac{2}{2}\\right)^{2}"],
        ["H_{0}H_{1}\\left|00\\right>", "H_{0}H_{1}\\left|00\\right>"],
    ];
    tests.iter().for_each(|test| {
        asrt(serialize(&latex_to_expr(test[0])), test[1]);
    });
}
