use super::*;
use std::collections::HashMap;

pub type TypingError = String; // TODO: not to use `String` as error type

type Env = HashMap<ExprVar, Type>;

struct Subst(pub HashMap<TypeVar, Type>);
impl Subst {
    pub fn new() -> Self { Subst(HashMap::new()) }
    pub fn apply(&self, typ: Type) -> Type {
        self.0.iter().fold(typ, |acc, (key, typ)| acc.subst(key, typ))
    }
    pub fn compose(Subst(subst1): Self, Subst(subst2): Self) -> Result<Self, TypingError> {
        let subst: Result<_, TypingError> = subst2.into_iter().try_fold(subst1, |mut acc, (key, typ)| {
            if acc.contains_key(&key) {
                let (_, typ1) = acc.remove_entry(&key).unwrap();
                let typ2 = typ;
                let (typ, subst) = unify(typ1, typ2)?;
                let Subst(mut subst) = Subst::compose(Subst(acc), subst).unwrap();
                subst.insert(key, typ);
                Ok(subst)
            } else {
                acc.insert(key, typ);
                Ok(acc)
            }
        });
        let subst = subst?;
        Ok(Subst(subst))
    }
}

fn occur(typ: &Type, name: &TypeVar) -> bool {
    match typ {
        Type::Var(name_) if name == name_ => true,
        Type::Func {
            context: _,
            params,
            ret
        } => {
            let params: &HashMap<ExprVar, Type> = params;
            params.iter().any(|(_, typ)| occur(typ, name)) || occur(ret, name)
        }
        _ => false
    }
}

fn unify(typ1: Type, typ2: Type) -> Result<(Type, Subst), TypingError> {
    match (typ1, typ2) {
        (typ1, typ2) if typ1 == typ2 => Ok((typ1, Subst::new())),
        (Type::Func{context: context1, params: mut params1, ret: ret1}, Type::Func{context: _, params: mut params2, ret: ret2}) => {
            let keys1: Vec<_> = params1.keys().cloned().collect();
            let keys2: Vec<_> = params2.keys().cloned().collect();
            if keys1 != keys2 {
                return Err(format!("unification error: {:?} vs {:?}", keys1, keys2));
            }
            let params: Result<_, TypingError> = keys1.into_iter().try_fold((HashMap::new(), Subst::new()), |(mut acc_params, acc_subst), field| {
                let (_, typ1) = params1.remove_entry(&field).unwrap();
                let (_, typ2) = params2.remove_entry(&field).unwrap();
                let (typ, subst) = unify(typ1, typ2)?;
                let subst = Subst::compose(acc_subst, subst)?;
                acc_params.insert(field, typ);
                Ok((acc_params, subst))
            });
            let (params, subst) = params?;
            let (ret, subst_) = unify(*ret1, *ret2)?;
            let subst = Subst::compose(subst, subst_)?;
            Ok((Type::Func {
                context: context1,
                params,
                ret: Box::new(ret)
            }, subst))
        }
        (Type::Var(name), typ) | (typ, Type::Var(name)) =>
            if occur(&typ, &name) {
                Err(format!("unification error: type variable {} occurs in {}", name, typ))
            } else {
                let mut new_subst = Subst::new();
                new_subst.0.insert(name, typ.clone());
                Ok((typ, new_subst))
            }
        (typ1, typ2) => Err(format!("unification error: {} vs {}", typ1, typ2))
    }
}

impl Expr {
    pub fn typecheck(&self) -> Result<Type, TypingError> {
        let (typ, subst) = expr(self, &Env::new())?;
        Ok(subst.apply(typ))
    }
}

fn expr(e: &Expr, env: &Env) -> Result<(Type, Subst), TypingError> {
    match e {
        Expr::Var(ref name) => env
            .get(name)
            .cloned()
            .ok_or(format!("unbound variable: {}", name))
            .map(|typ| (typ, Subst::new())),
        Expr::Int(_) => Ok((Type::Int, Subst::new())),
        Expr::String(_) => Ok((Type::String, Subst::new())),
        Expr::Path(_) => Ok((Type::Path, Subst::new())),
        Expr::Bytes(_) => Ok((Type::Bytes, Subst::new())),
        Expr::App(ref func, ref field, ref arg) => {
            if let (Type::Func {context, mut params, ret}, subst) = expr(func, env)? {
                let (arg_type, subst_) = expr(arg, env)?;
                let subst = Subst::compose(subst, subst_)?;
                let (_, param_type) = params.remove_entry(field).ok_or(format!("not {:?} in {}", params, field))?;
                let (_, subst_) = unify(arg_type, param_type)?;
                let subst = Subst::compose(subst, subst_)?;
                if params.is_empty() {
                    Ok((subst.apply(*ret), subst))
                } else {
                    Ok((Type::Func {
                        context,
                        params: params.into_iter().map(|(field, typ)| (field, subst.apply(typ))).collect(),
                        ret: Box::new(subst.apply(*ret))
                    }, subst))
                }
            } else {
                Err(format!("{} is not appliable expr", func))
            }
        }
        Expr::Command(cmd) => command(cmd),
    }
}

fn command(cmd: &Command) -> Result<(Type, Subst), TypingError> {
    let typ = TYPE_DEFINITIONS
        .get(&cmd.name)
        .ok_or(format!("undefined command: {}", cmd.name))?;
    Ok((typ.clone(), Subst::new()))
}

use once_cell::sync::Lazy;
static TYPE_DEFINITIONS: Lazy<HashMap<ExprVar, Type>> = Lazy::new(|| {
    let mut buf = String::new();
    use std::io::Read;
    std::fs::File::open("./type-definitions") // TODO:
        .expect("type definition file not found")
        .read_to_string(&mut buf)
        .expect("cannot read type definitions");
    let mut result = HashMap::new();
    for line in buf.lines() {
        let mut splitter = line.splitn(2, ':');
        let name = splitter.next().unwrap().to_string();
        let typ = splitter.next().unwrap().to_string();
        result.insert(ExprVar(name), parse::typ(&typ).expect("invalid type"));
    }
    result
});
