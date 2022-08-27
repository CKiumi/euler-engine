use crate::{expr::FuncName, Expr};

pub fn to_sympy(expr: &Expr) -> String {
    match expr {
        Expr::Add(add) => {
            let mut result = to_sympy(&add.exprs[0]);
            for expr in &add.exprs[1..] {
                result = format!("{}+{}", result, to_sympy(expr));
            }
            result
        }
        Expr::Mul(mul) => {
            let mut result = to_sympy(&mul.exprs[0]);
            for expr in &mul.exprs[1..] {
                result = format!("{}*{}", result, to_sympy(expr));
            }
            result
        }
        Expr::Pow(pow) => format!("({})**({})", to_sympy(&pow.body), to_sympy(&pow.pow)),
        Expr::Par(paren) => format!("({})", to_sympy(&paren.inner)),
        Expr::Sym(x) => format!("Symbol(\"{}\")", x),
        Expr::Num(x) => x.to_string(),
        Expr::Func(func) => match func.name {
            FuncName::Sin | FuncName::Cos | FuncName::Tan => {
                format!("{}({})", func.name, to_sympy(&func.args))
            }
            FuncName::Re | FuncName::Im => {
                format!("Function(\"\\{}\")({})", func.name, to_sympy(&func.args))
            }
            FuncName::Sqrt => format!("sqrt({})", to_sympy(&func.args)),
        },
        Expr::Frac(frac) => format!("({})/({})", to_sympy(&frac.numer), to_sympy(&frac.denom)),
    }
}

#[test]
fn test_sympy() {
    use crate::latex_to_sympy;
    let tests = [
        ["a", "Symbol(\"a\")"],
        ["a-b", r#"Symbol("a")+-1*Symbol("b")"#],
        ["-ab", r#"-1*Symbol("a")*Symbol("b")"#],
        ["a_{2}", "Symbol(\"a_{2}\")"],
        ["x^{y}", "(Symbol(\"x\"))**(Symbol(\"y\"))"],
        ["\\zeta", "Symbol(\"\\zeta\")"],
        ["a+b", "Symbol(\"a\")+Symbol(\"b\")"],
        ["ab", "Symbol(\"a\")*Symbol(\"b\")"],
        [
            "ab(a+b)",
            r#"Symbol("a")*Symbol("b")*(Symbol("a")+Symbol("b"))"#,
        ],
        [
            "ab(a+b)",
            r#"Symbol("a")*Symbol("b")*(Symbol("a")+Symbol("b"))"#,
        ],
        [
            "(ab)\\Re^{x+y}(a+b)(a+b)",
            r#"(Symbol("a")*Symbol("b"))*(Function("\Re")(Symbol("a")+Symbol("b")))**(Symbol("x")+Symbol("y"))*(Symbol("a")+Symbol("b"))"#,
        ],
        [
            "\\sqrt{a+b}^{x+y}",
            r#"(sqrt(Symbol("a")+Symbol("b")))**(Symbol("x")+Symbol("y"))"#,
        ],
    ];
    tests.iter().for_each(|test| {
        assert_eq!(latex_to_sympy(test[0].to_string()), test[1]);
    });
}
