mod command;
mod parser;
mod special;

use command::CommandOperation;
use special::SpecialOperation;

pub enum Operation {
    Special(SpecialOperation),
    Command(CommandOperation),
}

impl From<SpecialOperation> for Operation {
    fn from(op: SpecialOperation) -> Self {
        Operation::Special(op)
    }
}
impl From<CommandOperation> for Operation {
    fn from(op: CommandOperation) -> Self {
        Operation::Command(op)
    }
}

impl Operation {
    pub fn execute(&self) {
        use Operation::*;
        match self {
            Special(ref op) => op.execute(),
            Command(ref op) => op.execute(),
        }
    }
}

pub fn parse(line: &str) -> Result<Operation, String> {
    parser::operation(line)
}

// TODO
type Type = String;
pub trait TypedOperation: std::fmt::Display {
    fn typ(&self) -> &Type; // todo! not to use `String` as type
}
