use std::fmt;

use super::{ExprVar, TypeVar, Command, Expr, Type};

impl fmt::Display for ExprVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for TypeVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(ref ident) => write!(f, "{}", ident),
            Expr::Int(ref n) => write!(f, "{}", n),
            Expr::String(ref str) => write!(f, r#""{}""#, str),
            Expr::Path(ref dirs) => write!(f, "{}", dirs.join("/")),
            Expr::Bytes(_) => write!(f, "<bytes>"),
            Expr::App(ref e, ref field, ref typ) => {
                write!(f, "{} --{}:{}", e, field, typ)
            }
            Expr::Command(ref cmd) => write!(f, "{}", cmd),
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<_> = self
            .args
            .iter()
            .map(|(ref field, ref typ)| format!("--{}:{}", field, typ))
            .collect();
        write!(f, "{} {}", self.name, args.join(" "))
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Var(ref name) => write!(f, "{}", name),
            Type::Func {
                context,
                params,
                ret
            } => {
                        let params: Vec<_> = params
            .iter()
            .map(|(field, typ)| format!("--{}:{}", field, typ))
            .collect();
        write!(
            f,
            "[{}] {} -> {}",
            context,
            params.join(" -> "),
            ret
        )
            }
            Type::Int => write!(f, "int"),
            Type::String => write!(f, "string"),
            Type::Path => write!(f, "path"),
            Type::Bytes => write!(f, "bytes"),
            Type::Context => write!(f, "context")
        }
    }
}
