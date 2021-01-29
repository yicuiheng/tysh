mod expr;
mod typ;
use super::{Expr, Ident, Type};

pub type ParseError = String; // TODO: not to use `String` as error type

pub fn expr(s: &str) -> Result<Expr, ParseError> {
    toplevel(s, expr::expr)
}

pub fn typ(s: &str) -> Result<Type, ParseError> {
    toplevel(s, typ::typ)
}

fn toplevel<T, F>(s: &str, f: F) -> Result<T, ParseError>
where
    F: Fn(&[char]) -> Result<(T, &[char]), ParseError>,
{
    let s: Vec<_> = s.chars().collect();
    let s = whitespaces(&s);
    let (t, rest) = f(s)?;
    if rest.is_empty() {
        Ok(t)
    } else {
        Err(format!(
            "incomplete : rest is `{}`",
            rest.iter().collect::<String>()
        ))
    }
}

fn whitespaces<'a>(s: &'a [char]) -> &'a [char] {
    for (i, c) in s.iter().enumerate() {
        if !c.is_whitespace() {
            return &s[i..];
        }
    }
    &s[s.len()..]
}

fn char_(s: &[char], c: char) -> Result<&[char], ParseError> {
    match s.iter().nth(0) {
        Some(c_) if c == *c_ => Ok(&s[1..]),
        _ => Err(format!("reserved char `{}` does not match", c))
    }
}

fn reserved<'a>(s: &'a [char], word: &str) -> Result<&'a [char], ParseError> {
    if word.len() <= s.len() && word.chars().enumerate().all(|(i, c)| s[i] == c) {
        Ok(whitespaces(&s[word.len()..]))
    } else {
        Err(format!("reserved word `{}` does not match", word))
    }
}

fn ident(s: &[char]) -> Result<(Ident, &[char]), ParseError> {
    let mut ident = String::new();
    for (i, c) in s.iter().enumerate() {
        if i == 0 {
            if c.is_alphabetic() {
                ident.push(*c);
            } else {
                return Err(format!(
                    "identifier needs to start with alphabetic character"
                ));
            }
        } else {
            if c.is_alphanumeric() || *c == '_' {
                ident.push(*c);
            } else {
                return Ok((ident, whitespaces(&s[i..])));
            }
        }
    }
    Err(format!(
        "{} is invalid as identifier",
        s.iter().collect::<String>()
    ))
}
