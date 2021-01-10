use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::signal::Signal;

const LEFT_RIGHT_SEPARATOR: &str = "->";
const OP_NOT: &str = "NOT";
const OP_AND: &str = "AND";
const OP_OR: &str = "OR";
const OP_LSHIFT: &str = "LSHIFT";
const OP_RSHIFT: &str = "RSHIFT";

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Assign(Signal, Signal),
    Not(Signal, Signal),
    And(Signal, Signal, Signal),
    Or(Signal, Signal, Signal),
    Lshift(Signal, usize, Signal),
    Rshift(Signal, usize, Signal),
}

impl Instruction {
    pub fn get_output(&self) -> Signal {
        match self {
            Instruction::Assign(_, output) => *output,
            Instruction::Not(_, output) => *output,
            Instruction::And(_, _, output) => *output,
            Instruction::Or(_, _, output) => *output,
            Instruction::Lshift(_, _, output) => *output,
            Instruction::Rshift(_, _, output) => *output,
        }
    }

    pub fn evaluate(
        &self,
        map: &HashMap<Signal, Instruction>,
        evaluated: &mut HashMap<Signal, u16>,
    ) -> Option<u16> {
        match self {
            Instruction::Assign(input, _) => input.evaluate(map, evaluated),
            Instruction::Not(input, output) => {
                let value = u16::MAX - input.evaluate(map, evaluated)?;
                evaluated.insert(*output, value);
                Some(value)
            }
            Instruction::And(input1, input2, output) => {
                let value = input1.evaluate(map, evaluated)? & input2.evaluate(map, evaluated)?;
                evaluated.insert(*output, value);
                Some(value)
            }
            Instruction::Or(input1, input2, output) => {
                let value = input1.evaluate(map, evaluated)? | input2.evaluate(map, evaluated)?;
                evaluated.insert(*output, value);
                Some(value)
            }
            Instruction::Lshift(input, offset, output) => {
                let value = input.evaluate(map, evaluated)? << offset;
                evaluated.insert(*output, value);
                Some(value)
            }
            Instruction::Rshift(input, offset, output) => {
                let value = input.evaluate(map, evaluated)? >> offset;
                evaluated.insert(*output, value);
                Some(value)
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign(input, output) => {
                write!(f, "{} {} {}", *input, LEFT_RIGHT_SEPARATOR, *output)
            }
            Instruction::Not(input, output) => {
                write!(
                    f,
                    "{} {} {} {}",
                    OP_NOT, *input, LEFT_RIGHT_SEPARATOR, *output
                )
            }
            Instruction::And(input1, input2, output) => {
                write!(
                    f,
                    "{} {} {} {} {}",
                    *input1, OP_AND, *input2, LEFT_RIGHT_SEPARATOR, *output
                )
            }
            Instruction::Or(input1, input2, output) => {
                write!(
                    f,
                    "{} {} {} {} {}",
                    *input1, OP_OR, *input2, LEFT_RIGHT_SEPARATOR, *output
                )
            }
            Instruction::Lshift(input, offset, output) => {
                write!(
                    f,
                    "{} {} {} {} {}",
                    *input, OP_LSHIFT, *offset, LEFT_RIGHT_SEPARATOR, *output
                )
            }
            Instruction::Rshift(input, offset, output) => {
                write!(
                    f,
                    "{} {} {} {} {}",
                    *input, OP_RSHIFT, *offset, LEFT_RIGHT_SEPARATOR, *output
                )
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let error = || parse_error!("Invalid instruction: {}", instruction);

        if let Some((left, right)) = split_instruction(instruction) {
            let output = right.parse::<Signal>()?;

            if let Some(label) = left.strip_prefix(OP_NOT) {
                let input = label.trim().parse::<Signal>()?;
                Ok(Instruction::Not(input, output))
            } else if let Some(idx) = left.find(OP_AND) {
                let input1 = left[..idx].trim().parse::<Signal>()?;
                let input2 = left[idx + OP_AND.len()..].trim().parse::<Signal>()?;
                Ok(Instruction::And(input1, input2, output))
            } else if let Some(idx) = left.find(OP_OR) {
                let input1 = left[..idx].trim().parse::<Signal>()?;
                let input2 = left[idx + OP_OR.len()..].trim().parse::<Signal>()?;
                Ok(Instruction::Or(input1, input2, output))
            } else if let Some(idx) = left.find(OP_LSHIFT) {
                let input1 = left[..idx].trim().parse::<Signal>()?;
                let offset = left[idx + OP_LSHIFT.len()..].trim().parse::<usize>()?;
                Ok(Instruction::Lshift(input1, offset, output))
            } else if let Some(idx) = left.find(OP_RSHIFT) {
                let input1 = left[..idx].trim().parse::<Signal>()?;
                let offset = left[idx + OP_RSHIFT.len()..].trim().parse::<usize>()?;
                Ok(Instruction::Rshift(input1, offset, output))
            } else {
                let input = left.trim().parse::<Signal>()?;
                Ok(Instruction::Assign(input, output))
            }
        } else {
            Err(error())
        }
    }
}

fn split_instruction(instruction: &str) -> Option<(&str, &str)> {
    let split_point = instruction.find(LEFT_RIGHT_SEPARATOR)?;

    let left = instruction[..split_point].trim();
    let right = instruction[split_point + LEFT_RIGHT_SEPARATOR.len()..].trim();

    Some((left, right))
}
