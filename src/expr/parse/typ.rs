use std::collections::HashMap;
use crate::expr::{parse::ParseError, ExprVar, Type};
use super::{reserved, ident};

pub fn typ(rest: &[char]) -> Result<(Type, &[char]), ParseError> {
    let primitive_type = vec![
        ("int", Type::Int),
        ("string", Type::String),
        ("path", Type::Path),
        ("bytes", Type::Bytes)
    ];
    for (word, typ) in primitive_type {
        if let Ok(rest) = reserved(rest, word) {
            return Ok((typ, rest));
        }
    }
    let rest = reserved(rest, "[")?;
    let (context, rest) = ident(rest)?;
    let context = ExprVar(context);
    let mut rest = reserved(rest, "]")?;
    let mut params = HashMap::new();
    loop {
        let (field, rest_) = if let (Some('-'), Some('-')) = (rest.iter().nth(0), rest.iter().nth(1))
        {
            ident(&rest[2..])?
        } else {
            break;
        };
        let rest_ = if let Some(':') = rest_.iter().nth(0) {
            &rest_[1..]
        } else {
            return Err(format!("field and type in parameter must be divided by `:`"));
        };
        let (typ, rest_) = typ(rest_)?;
        let rest_ = reserved(rest_, "->")?;
        params.insert(ExprVar(field), typ);
        rest = rest_;
    }
    let (ret, rest) = typ(rest)?;
    Ok((Type::Func {
        context,
        params,
        ret: Box::new(ret)
    }, rest))
}
