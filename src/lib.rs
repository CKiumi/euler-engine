#![feature(box_patterns)]
#![feature(test)]
mod expr;
pub mod parser;
pub use expr::{Add, Expr, Mul, Num, Pow, Sym};
use parser::{latex_to_expr, serialize, to_sympy};
use wasm_bindgen::prelude::*;

extern crate test;

#[cfg(test)]
mod euler_bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn expander(b: &mut Bencher) {
        b.iter(|| {
            expand(
                "\\left(a+b\\right)\\left(c+d\\right)\\left(e+f\\right)\\left(g+h\\right)\\left(i+j\\right)\\left(x+y\\right)"
                    .to_string(),
            )
        });
    }
}

#[wasm_bindgen]
pub fn latex_to_sympy(latex: String) -> String {
    to_sympy(&latex_to_expr(&latex))
}

#[wasm_bindgen]
pub fn expand(input: String) -> String {
    serialize(&latex_to_expr(&input).expand())
}

#[wasm_bindgen]
pub fn collect(input: String) -> String {
    match latex_to_expr(&input) {
        Expr::Add(add) => serialize(&Expr::Add(add.collect())),
        Expr::Mul(mul) => serialize(&Expr::Mul(mul.collect())),
        Expr::Num(num) => serialize(&Expr::Num(num)),
        _ => input,
    }
}

#[test]
fn test_lib() {
    assert_eq!(
        expand("\\left(x+y\\right)^{2}".to_string()),
        "\\left(x^{2}+2xy+y^{2}\\right)"
    );
    assert_eq!(
        expand("(\\sin^{2}(x)+\\Re^{2}(y))^{2}".to_string()),
        "\\left(\\sin^{4}\\left(x\\right)+2\\sin^{2}\\left(x\\right)\\Re^{2}\\left(y\\right)+\\Re^{4}\\left(y\\right)\\right)"
    );
}
