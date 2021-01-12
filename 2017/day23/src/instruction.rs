use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _output_receiver: &mut O,
    ) -> i64 {
        match self {
            Instruction::Set(reg, b) => {
                let b = b.get(register);
                register.write(*reg, b);
                1
            }
            Instruction::Sub(reg, b) => {
                let b = b.get(register);
                register.increment(*reg, -b);
                1
            }
            Instruction::Mul(reg, b) => {
                let a = register.read(*reg);
                let b = b.get(register);
                register.write(*reg, a * b);
                1
            }
            Instruction::Jnz(a, b) => {
                let a = a.get(register);
                if a == 0 {
                    1
                } else {
                    b.get(register)
                }
            }
        }
    }

    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError> {
        match parsed.get_command() {
            "set" => Ok(Instruction::Set(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "sub" => Ok(Instruction::Sub(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "mul" => Ok(Instruction::Mul(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "jnz" => Ok(Instruction::Jnz(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            _ => Err(parse_error!("Invalid instruction: {}", parsed)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Set(a, b) => write!(f, "set {} {}", a, b),
            Instruction::Sub(a, b) => write!(f, "sub {} {}", a, b),
            Instruction::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Instruction::Jnz(a, b) => write!(f, "jnz {} {}", a, b),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}
