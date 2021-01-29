use std::collections::HashMap;

pub mod eval;
pub mod parse;
pub mod print;
pub mod typecheck;

pub type Ident = String;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprVar(pub Ident);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVar(pub Ident);

#[derive(Debug, Clone)]
pub enum Expr {
    Var(ExprVar),
    Int(i32),
    String(String),
    Path(Vec<String>),
    Bytes(Vec<u8>),
    App(Box<Expr>, ExprVar, Box<Expr>),
    Command(Command),
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: ExprVar,
    pub args: HashMap<ExprVar, Box<Expr>>,
}

impl From<Command> for Expr {
    fn from(cmd: Command) -> Expr {
        Expr::Command(cmd)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Func {
        context: ExprVar,
        params: HashMap<ExprVar, Type>,
        ret: Box<Type>
    },
    Int,
    String,
    Path,
    Bytes,

    Context
}

impl Type {
    pub fn subst(self, name: &TypeVar, typ: &Type) -> Type {
        match self {
            Type::Var(name_) if name == &name_ => typ.clone(),
            Type::Func{context, params, ret} => Type::Func {
                context,
                params: params.into_iter().map(|(key, typ_)| (key, typ_.subst(name, typ))).collect(),
                ret: Box::new(ret.subst(name, typ))
            },
            _ => self
        }
    }
}
