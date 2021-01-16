use super::TypedOperation;
use std::fmt;
pub enum SpecialOperation {
    ShowType(Box<dyn TypedOperation>),
}

impl SpecialOperation {
    pub fn execute(&self) {
        use SpecialOperation::*;
        match self {
            ShowType(ref op) => {
                println!("{}: {}", op, op.typ());
            }
        }
    }
}

impl fmt::Display for SpecialOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use SpecialOperation::*;
        match self {
            ShowType(ref op) => write!(f, ":type {}", op),
        }
    }
}
