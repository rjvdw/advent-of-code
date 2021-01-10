use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    /// Add two numbers
    Plus,

    /// Multiply two numbers
    Times,

    /// Discard the previous number
    Nop,
}

impl Operator {
    pub fn evaluate(self, n1: i64, n2: i64) -> i64 {
        match self {
            Operator::Plus => n1 + n2,
            Operator::Times => n1 * n2,
            Operator::Nop => n2,
        }
    }
}

impl FromStr for Operator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Plus),
            "*" => Ok(Operator::Times),
            _ => Err(parse_error!("Invalid operator: {}", s)),
        }
    }
}
