use std::fmt::{Display, Formatter, Result};

use super::sym::Sym;
pub struct Add<'a> {
    exprs: Vec<Sym<'a>>,
}

impl<'a> Add<'a> {
    pub fn new(exprs: Vec<Sym<'a>>) -> Self {
        Add { exprs }
    }
}

impl<'a> Display for Add<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut result = self.exprs[0].to_string();
        for i in 1..self.exprs.len() {
            result = format!("{}+{}", result, self.exprs[i]);
        }
        result = format!("({})", result);
        write!(f, "{}", result)
    }
}

#[test]
fn test_add() {
    let x = Sym::new("x");
    let y = Sym::new("y");
    let add = Add::new(vec![x, y, y]);
    println!("{add}");
}
