use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(isize),
    JumpIfEven(char, isize),
    JumpIfOne(char, isize),
}

impl Instruction {
    pub fn execute(&self, idx: isize, registers: &mut HashMap<char, u64>) -> isize {
        match self {
            Instruction::Half(register) => {
                *registers.entry(*register).or_insert(0) /= 2;
                idx + 1
            }
            Instruction::Triple(register) => {
                *registers.entry(*register).or_insert(0) *= 3;
                idx + 1
            }
            Instruction::Increment(register) => {
                *registers.entry(*register).or_insert(0) += 1;
                idx + 1
            }
            Instruction::Jump(offset) => idx + offset,
            Instruction::JumpIfEven(register, offset) => {
                idx + if *registers.get(register).unwrap_or(&0) % 2 == 0 {
                    *offset
                } else {
                    1
                }
            }
            Instruction::JumpIfOne(register, offset) => {
                idx + if *registers.get(register).unwrap_or(&0) == 1 {
                    *offset
                } else {
                    1
                }
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Half(reg) => write!(f, "hlf {}", reg),
            Instruction::Triple(reg) => write!(f, "tpl {}", reg),
            Instruction::Increment(reg) => write!(f, "inc {}", reg),
            Instruction::Jump(offset) => write!(f, "jmp {:+}", offset),
            Instruction::JumpIfEven(reg, offset) => write!(f, "jie {}, {:+}", reg, offset),
            Instruction::JumpIfOne(reg, offset) => write!(f, "jio {}, {:+}", reg, offset),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = instruction.strip_prefix("hlf ") {
            Ok(Instruction::Half(parse_register(instruction, rest)?))
        } else if let Some(rest) = instruction.strip_prefix("tpl ") {
            Ok(Instruction::Triple(parse_register(instruction, rest)?))
        } else if let Some(rest) = instruction.strip_prefix("inc ") {
            Ok(Instruction::Increment(parse_register(instruction, rest)?))
        } else if let Some(rest) = instruction.strip_prefix("jmp ") {
            Ok(Instruction::Jump(parse_offset(rest)?))
        } else if let Some(rest) = instruction.strip_prefix("jie ") {
            Ok(Instruction::JumpIfEven(
                parse_register(instruction, rest)?,
                parse_offset(rest)?,
            ))
        } else if let Some(rest) = instruction.strip_prefix("jio ") {
            Ok(Instruction::JumpIfOne(
                parse_register(instruction, rest)?,
                parse_offset(rest)?,
            ))
        } else {
            Err(ParseError(format!("Invalid instruction: {}", instruction)))
        }
    }
}

fn parse_register(instruction: &str, rest: &str) -> Result<char, ParseError> {
    match rest.chars().next() {
        Some(ch) => Ok(ch),
        None => Err(ParseError(format!("Invalid instruction: {}", instruction))),
    }
}

fn parse_offset(rest: &str) -> Result<isize, ParseIntError> {
    let offset = rest.find(' ').unwrap_or(0);
    rest[offset..].trim().parse::<isize>()
}
