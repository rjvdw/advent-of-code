use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpIfEven(Value, i32),
    JumpIfOne(Value, i32),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _output_receiver: &mut O,
    ) -> i32 {
        match self {
            Instruction::Half(reg) => {
                register.write(*reg, register.read(*reg) / 2);
                1
            }
            Instruction::Triple(reg) => {
                register.write(*reg, register.read(*reg) * 3);
                1
            }
            Instruction::Increment(reg) => {
                register.increment(*reg, 1);
                1
            }
            Instruction::Jump(offset) => *offset,
            Instruction::JumpIfEven(v, offset) => {
                if v.get(register) % 2 == 0 {
                    *offset
                } else {
                    1
                }
            }
            Instruction::JumpIfOne(v, offset) => {
                if v.get(register) == 1 {
                    *offset
                } else {
                    1
                }
            }
        }
    }

    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError> {
        match parsed.get_command() {
            "hlf" => Ok(Instruction::Half(parsed.get_argument(0)?)),
            "tpl" => Ok(Instruction::Triple(parsed.get_argument(0)?)),
            "inc" => Ok(Instruction::Increment(parsed.get_argument(0)?)),
            "jmp" => Ok(Instruction::Jump(parsed.get_argument(0)?)),
            "jie" => Ok(Instruction::JumpIfEven(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "jio" => Ok(Instruction::JumpIfOne(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            _ => Err(ParseError(format!("Unknown command: {}", parsed))),
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}
