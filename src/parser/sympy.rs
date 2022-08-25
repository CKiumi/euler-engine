use super::latex_to_expr;
use crate::Expr;

pub fn to_sympy(expr: &Expr) -> String {
    match expr {
        Expr::Add(add) => {
            let mut result = to_sympy(&add.exprs[0]);
            add.exprs[1..]
                .iter()
                .for_each(|expr| result = format!("{}+{}", result, to_sympy(expr)));
            result
        }
        Expr::Mul(mul) => {
            let mut result = to_sympy(&mul.exprs[0]);
            mul.exprs[1..]
                .iter()
                .for_each(|expr| result = format!("{}*{}", result, to_sympy(expr)));
            result
        }
        Expr::Pow(pow) => format!("{}**{}", to_sympy(&pow.body), to_sympy(&pow.pow)),
        Expr::Par(paren) => format!("({})", to_sympy(&paren.inner)),
        Expr::Sym(x) => format!("Symbol(\"{}\")", x),
        Expr::Num(x) if x.num == -1 => String::from("-"),
        Expr::Num(x) => x.to_string(),
    }
}

pub fn latex_to_sympy(latex: &str) -> String {
    to_sympy(&latex_to_expr(latex))
}

#[test]
fn test_sympy() {
    let tests = [
        ["a", "Symbol(\"a\")"],
        ["a_{2}", "Symbol(\"a_{2}\")"],
        ["x^{y}", "Symbol(\"x\")**Symbol(\"y\")"],
        ["\\zeta", "Symbol(\"\\zeta\")"],
        ["a+b", "Symbol(\"a\")+Symbol(\"b\")"],
        ["ab", "Symbol(\"a\")*Symbol(\"b\")"],
        [
            "ab(a+b)",
            "Symbol(\"a\")*Symbol(\"b\")*(Symbol(\"a\")+Symbol(\"b\"))",
        ],
        [
            "ab(a+b)",
            "Symbol(\"a\")*Symbol(\"b\")*(Symbol(\"a\")+Symbol(\"b\"))",
        ],
    ];
    tests.iter().for_each(|test| {
        assert_eq!(latex_to_sympy(test[0]), test[1]);
    });
}
