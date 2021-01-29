use crate::expr::Expr;
use std::fmt;

#[derive(Debug)]
pub enum MetaOperation {
    ShowType(Expr),
}

impl fmt::Display for MetaOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetaOperation::ShowType(ref e) => write!(f, ":type {}", e),
        }
    }
}

impl MetaOperation {
    pub fn execute(&self) {
        match self {
            MetaOperation::ShowType(ref e) => println!("{}: {}", e, e.typecheck().unwrap()),
        }
    }
}

pub fn parse(s: &str) -> Result<MetaOperation, String> {
    const SHOW_TYPE_PREFIX: &str = ":type ";
    if s.starts_with(SHOW_TYPE_PREFIX) {
        let rest = &s[SHOW_TYPE_PREFIX.len()..];
        let e = crate::expr::parse::expr(rest)?;
        Ok(MetaOperation::ShowType(e))
    } else {
        Err(format!("invalid meta operation"))
    }
}
