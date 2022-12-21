use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug, Clone)]
pub enum Monkey {
    Value(i64),
    Math(Operation, String, String),
    Var,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(' ') {
            Ok(Monkey::Math(
                match s.chars().nth(5) {
                    Some('+') => Operation::Add,
                    Some('-') => Operation::Sub,
                    Some('*') => Operation::Mul,
                    Some('/') => Operation::Div,
                    _ => {
                        return err_parse_error!("Invalid input: {}", s);
                    }
                },
                s[..4].to_string(),
                s[7..].to_string(),
            ))
        } else {
            Ok(Monkey::Value(s.parse()?))
        }
    }
}
