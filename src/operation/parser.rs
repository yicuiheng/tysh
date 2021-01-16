use std::collections::HashMap;

use super::{command::CommandOperation, special::SpecialOperation, Operation};

type ParseError = String; // TODO

pub fn operation(line: &str) -> Result<Operation, ParseError> {
    let line: Vec<char> = line.chars().collect();
    let tokens: Vec<String> = tokenize(&line)?;
    parse_from_tokens(&tokens)
}

fn parse_from_tokens(tokens: &[String]) -> Result<Operation, ParseError> {
    parse_special_operator(&tokens)
        .map(Operation::Special)
        .or_else(|_| parse_command_operator(&tokens).map(Operation::Command))
}

fn tokenize(line: &Vec<char>) -> Result<Vec<String>, ParseError> {
    let mut result = vec![];
    let mut is_prev_whiltespace = true;
    let mut is_string_literal = false;
    let mut is_string_literal_escape = false;
    let mut current = String::new();
    for c in line {
        match (is_string_literal, is_string_literal_escape, c) {
            (true, true, c) => {
                is_string_literal_escape = false;
                current.push(*c);
                continue;
            }
            (true, false, '\\') => {
                is_string_literal_escape = true;
                current.push(*c);
                continue;
            }
            (true, false, '"') => {
                is_string_literal = false;
                current.push(*c);
                continue;
            }
            (true, false, _) => {
                current.push(*c);
                continue;
            }
            (false, true, _) => unreachable!(),
            (false, false, _) => (),
        }
        if is_prev_whiltespace && c.is_whitespace() {
            continue;
        } else if !is_prev_whiltespace && !c.is_whitespace() {
            current.push(*c);
            continue;
        } else if is_prev_whiltespace && !c.is_whitespace() {
            current = c.to_string();
        } else if !is_prev_whiltespace && c.is_whitespace() {
            result.push(std::mem::take(&mut current));
        }
        is_prev_whiltespace = c.is_whitespace();
    }
    if is_string_literal {
        Err(format!("string literal does not close"))
    } else {
        Ok(result)
    }
}

fn parse_special_operator(tokens: &[String]) -> Result<SpecialOperation, ParseError> {
    let op = tokens
        .iter()
        .nth(0)
        .ok_or_else(|| "special command".to_string())?;
    match op.as_str() {
        ":type" => {
            let target_op = parse_command_operator(&tokens[1..])?;
            Ok(SpecialOperation::ShowType(Box::new(target_op)))
        }
        op => Err(format!("invalid special operator: {}", op)),
    }
}

fn parse_arg(arg: &String) -> Result<(String, String), ParseError> {
    // state
    //   0: expect `-`
    //   1: expect `-`
    //   2: expect `:` or others
    //   3: accept
    let mut state = 0;
    let mut current = String::new();
    let mut key = String::new();
    for c in arg.chars() {
        match state {
            0 if c == '-' => state = 1,
            1 if c == '-' => state = 2,
            2 if c == ':' => {
                key = std::mem::take(&mut current);
                state = 3;
            }
            2 => current.push(c),
            3 => current.push(c),
            _ => return Err(format!("invalid argument: {}", arg)),
        }
    }
    if state != 3 {
        return Err(format!("invalid argument: {}", arg));
    }
    Ok((key, current))
}

fn parse_command_operator(tokens: &[String]) -> Result<CommandOperation, ParseError> {
    let (command_name, args) = tokens
        .split_first()
        .ok_or_else(|| format!("command name required"))?;
    let args: Result<HashMap<String, String>, ParseError> = args.iter().map(parse_arg).collect();
    Ok(CommandOperation {
        command_name: command_name.clone(),
        args: args?,
    })
}
