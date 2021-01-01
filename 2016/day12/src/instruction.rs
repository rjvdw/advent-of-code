use std::collections::HashMap;
use std::str::{FromStr, SplitWhitespace};

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone)]
pub enum Value {
    Raw(i32),
    Register(char),
}

impl Value {
    fn get_value(&self, registers: &HashMap<char, i32>) -> i32 {
        match self {
            Value::Raw(v) => *v,
            Value::Register(reg) => *registers.get(reg).unwrap_or(&0),
        }
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            let ch = s.chars().next().unwrap();
            if ch.is_alphabetic() {
                return Ok(Value::Register(ch));
            }
        }

        Ok(Value::Raw(s.parse()?))
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Value, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(Value, Value),
}

impl Instruction {
    pub fn run(&self, registers: &mut HashMap<char, i32>) -> i32 {
        match self {
            Instruction::Copy(v, reg) => {
                registers.insert(*reg, v.get_value(registers));
                1
            }
            Instruction::Increment(reg) => {
                *registers.entry(*reg).or_insert(0) += 1;
                1
            }
            Instruction::Decrement(reg) => {
                *registers.entry(*reg).or_insert(0) -= 1;
                1
            }
            Instruction::JumpNotZero(v, offset) => {
                if v.get_value(registers) == 0 {
                    1
                } else {
                    offset.get_value(registers)
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix("cpy ") {
            let mut parts = r.split_whitespace();
            let value = part_as(s, &mut parts)?;
            let register = if let Some(v) = parts.next().filter(|v| v.len() == 1) {
                v.chars().next().unwrap()
            } else {
                error(s, "Invalid register")?
            };
            if parts.next().is_some() {
                error(s, "Unexpected value")?
            }
            Ok(Instruction::Copy(value, register))
        } else if let Some(r) = s.strip_prefix("inc ") {
            if r.len() == 1 {
                Ok(Instruction::Increment(r.chars().next().unwrap()))
            } else {
                error(s, "Invalid register")?
            }
        } else if let Some(r) = s.strip_prefix("dec ") {
            if r.len() == 1 {
                Ok(Instruction::Decrement(r.chars().next().unwrap()))
            } else {
                error(s, "Invalid register")?
            }
        } else if let Some(r) = s.strip_prefix("jnz ") {
            let mut parts = r.split_whitespace();
            let value1 = part_as(s, &mut parts)?;
            let value2 = part_as(s, &mut parts)?;
            if parts.next().is_some() {
                error(s, "Unexpected value")?
            }
            Ok(Instruction::JumpNotZero(value1, value2))
        } else {
            error(s, "Unrecognized operation")
        }
    }
}

fn part_as<T: FromStr>(line: &str, parts: &mut SplitWhitespace) -> Result<T, ParseError>
where
    ParseError: From<<T as FromStr>::Err>,
{
    if let Some(v) = parts.next() {
        Ok(v.parse::<T>()?)
    } else {
        error(line, "Could not parse value")
    }
}

fn error<T>(line: &str, msg: &str) -> Result<T, ParseError> {
    Err(ParseError(format!(
        "Invalid instruction {} - {}",
        line, msg
    )))
}
