use std::fmt::{Display, Formatter, Result};
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Num {
    pub num: i32,
}

impl Num {
    pub fn new(num: i32) -> Self {
        Num { num }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.num)
    }
}

#[test]
fn test_num() {
    use super::Expr;
    let x = Expr::Num(Num::new(1));
    let y = Expr::Num(Num::new(2));
    assert_eq!((x + y).to_string(), "3");
    assert_eq!((Num::new(3) ^ Num::new(3)).to_string(), "27");
}
