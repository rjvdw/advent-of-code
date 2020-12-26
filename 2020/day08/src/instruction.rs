use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

use crate::ProgramState;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    ACC,
    JMP,
    NOP,
}

/// A single instruction, consisting of an operation and a value.
#[derive(Debug, Copy, Clone)]
pub struct Instruction(pub Operation, pub i32);

impl Instruction {
    pub fn run(&self, state: ProgramState) -> ProgramState {
        let &Instruction(op, value) = self;
        match op {
            Operation::ACC => (state.0 + value, state.1 + 1),
            Operation::JMP => {
                if value >= 0 {
                    (state.0, state.1 + (value as usize))
                } else {
                    (state.0, state.1 - ((-value) as usize))
                }
            }
            Operation::NOP => (state.0, state.1 + 1),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(' ') {
            Some(pos) => {
                let op: Operation = match &s[..pos] {
                    "acc" => Ok(Operation::ACC),
                    "jmp" => Ok(Operation::JMP),
                    "nop" => Ok(Operation::NOP),
                    _ => Err(ParseError(format!("Invalid input line: '{}'", s))),
                }?;
                let value: i32 = s[pos + 1..].parse::<i32>()?;

                Ok(Instruction(op, value))
            }
            None => Err(ParseError(format!("Invalid input line: '{}'", s))),
        }
    }
}
