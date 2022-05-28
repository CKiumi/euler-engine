mod expr;
pub mod parser;
pub use expr::{Add, Expr, Mul, Num, Pow, Sym};
use parser::{latex_to_expr, serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn collect(input: String) -> String {
    match latex_to_expr(&input) {
        Expr::Add(add) => serialize(&Expr::Add(add.collect())),
        _ => input,
    }
}
