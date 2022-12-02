use super::ToExpr;
use crate::Expr;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum GateName {
    I(u32),
    X(u32),
    Y(u32),
    Z(u32),
    H(u32),
    S(u32),
    T(u32),
}

impl Display for GateName {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            GateName::I(qbit) => write!(f, "I({})", qbit),
            GateName::X(qbit) => write!(f, "X({})", qbit),
            GateName::Y(qbit) => write!(f, "Y({})", qbit),
            GateName::Z(qbit) => write!(f, "Z({})", qbit),
            GateName::S(qbit) => write!(f, "S({})", qbit),
            GateName::H(qbit) => write!(f, "H({})", qbit),
            GateName::T(qbit) => write!(f, "T({})", qbit),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Gate {
    pub name: GateName,
}

impl Gate {
    pub fn new(name: GateName) -> Self {
        Gate { name }
    }
    pub fn change_qbit(&self, qbit: u32) -> Self {
        match self.name {
            GateName::I(_) => Gate::new(GateName::I(qbit)),
            GateName::X(_) => Gate::new(GateName::X(qbit)),
            GateName::Y(_) => Gate::new(GateName::Y(qbit)),
            GateName::Z(_) => Gate::new(GateName::Z(qbit)),
            GateName::S(_) => Gate::new(GateName::S(qbit)),
            GateName::H(_) => Gate::new(GateName::H(qbit)),
            GateName::T(_) => Gate::new(GateName::T(qbit)),
        }
    }
}

impl ToExpr for Gate {
    fn to_expr(self) -> Expr {
        Expr::Gate(self)
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}
