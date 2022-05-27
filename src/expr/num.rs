use std::{
    fmt::{Display, Formatter, Result},
    ops,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Num {
    num: i32,
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

impl ops::Add<Num> for Num {
    type Output = Num;
    fn add(self, _rhs: Num) -> Num {
        Num::new(self.num + _rhs.num)
    }
}

#[test]
fn test_num() {
    let x = Num::new(1);
    let y = Num::new(2);
    assert_eq!((x + y).to_string().as_str(), "3");
}
