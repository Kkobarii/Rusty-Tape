use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rel {
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
}

impl Display for Rel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rel::Lt => write!(f, "<"),
            Rel::Gt => write!(f, ">"),
            Rel::Le => write!(f, "<="),
            Rel::Ge => write!(f, ">="),
            Rel::Eq => write!(f, "=="),
            Rel::Ne => write!(f, "!="),
        }
    }
}