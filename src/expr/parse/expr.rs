use std::collections::HashMap;

use super::{ident, ParseError, whitespaces, char_};
use crate::expr::{Command, Expr, ExprVar};

pub fn expr(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    app(rest)
}

fn app(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    let (mut expr, mut rest) = factor_expr(rest)?;
    loop {
        let (ident, rest_) = if let (Some('-'), Some('-')) = (rest.iter().nth(0), rest.iter().nth(1))
        {
            ident(&rest[2..])?
        } else {
            return Ok((expr, rest));
        };
        let rest_ = if let Some(':') = rest_.iter().nth(0) {
            &rest_[1..]
        } else {
            return Err(format!("field and expr in argument must be divided by `:`"));
        };
        let (arg, rest_) = factor_expr(rest_)?;
        expr = Expr::App(Box::new(expr), ExprVar(ident), Box::new(arg));
        rest = rest_;
    }
}

fn factor_expr(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    var(rest)
        .or_else(|_| int(rest))
        .or_else(|_| string(rest))
        .or_else(|_| path(rest))
        .or_else(|_| bytes(rest))
        .or_else(|_| command(rest).map(|(cmd, rest)| (Expr::Command(cmd), rest)))
}

fn var(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    if let Some('$') = rest.iter().nth(0) {
        let (ident, rest) = ident(&rest[1..])?;
        Ok((Expr::Var(ExprVar(format!("${}", ident))), rest))
    } else {
        Err(format!("variable name must start with '$'"))
    }
}

fn int(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    fn positive_num(rest: &[char]) -> Result<(i32, &[char]), ParseError> {
        let (mut n, mut rest) = match rest.iter().nth(0) {
            Some(c) if '1' <= *c && *c <= '9' => (c.to_string().parse().unwrap(), &rest[1..]),
            _ => return Err(format!("`{:?}` can not be parsed as integer", rest)),
        };
        while let Some(c) = rest.iter().nth(0) {
            if !c.is_numeric() {
                break;
            }
            let digit: i32 = c.to_string().parse().unwrap();
            n = n * 10 + digit;
            rest = &rest[1..];
        }
        Ok((n, whitespaces(rest)))
    }
    if let Some('-') = rest.iter().nth(0) {
        positive_num(&rest[1..]).map(|(n, rest)| (Expr::Int(-n), rest))
    } else {
        positive_num(rest).map(|(n, rest)| (Expr::Int(n), rest))
    }
}

fn string(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    let mut rest = char_(rest, '"')?;
    let mut is_escaped = false;
    let mut s = String::new();
    loop {
        match rest.iter().nth(0) {
            Some(c) if is_escaped => {
                s.push(*c);
                is_escaped = false;
                rest = &rest[1..];
            }
            Some('\\') => {
                is_escaped = true;
                rest = &rest[1..];
            }
            Some('"') => {
                rest = &rest[1..];
                break;
            }
            Some(c) => {
                s.push(*c);
                rest = &rest[1..];
            }
            _ => {
                return Err(format!("TODO"));
            }
        }
    }
    Ok((Expr::String(s), whitespaces(rest)))
}

fn path(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    let (prime, mut rest) = prime_path(rest)?;
    let mut path = vec![prime];
    loop {
        if let Ok(rest_) = char_(rest, '/') {
            let (entry, rest_) = entry_name(rest_)?;
            path.push(entry);
            rest = rest_;
        } else {
            return Ok((Expr::Path(path), whitespaces(rest)));
        }
    }
}

fn entry_name(mut rest: &[char]) -> Result<(String, &[char]), ParseError> {
    let mut entry_name = String::new();
    while let Some(c) = rest.iter().nth(0) {
        if c.is_whitespace() || *c == '/' {
            break;
        }
        entry_name.push(*c);
        rest = &rest[1..];
    }
    Ok((entry_name, rest))
}

fn prime_path(rest: &[char]) -> Result<(String, &[char]), ParseError> {
    if let (Some('.'), Some('.')) = (rest.iter().nth(0), rest.iter().nth(1)) {
        return Ok(("..".to_string(), &rest[2..]));
    } else if let Some('.') = rest.iter().nth(0) {
        return Ok((".".to_string(), &rest[1..]));
    } else if let Some('/') = rest.iter().nth(0) {
        return Ok(("".to_string(), rest));
    } else {
        return Err(format!("invalid path: {:?}", rest));
    }
}

fn bytes(rest: &[char]) -> Result<(Expr, &[char]), ParseError> {
    Err(format!("not implemented"))
}

fn command(rest: &[char]) -> Result<(Command, &[char]), ParseError> {
    let (name, rest) = ident(rest)?;
    Ok((
        Command {
            name: ExprVar(name),
            args: HashMap::new(),
        },
        rest,
    ))
}
