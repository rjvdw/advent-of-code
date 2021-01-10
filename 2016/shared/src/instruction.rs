use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Value, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(Value, Value),
    Toggle(Value),
    Out(Value),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        output_receiver: &mut O,
    ) -> i64 {
        match self {
            Instruction::Copy(v, reg) => {
                register.write(*reg, v.get(register));
                1
            }
            Instruction::Increment(reg) => {
                register.increment(*reg, 1);
                1
            }
            Instruction::Decrement(reg) => {
                register.increment(*reg, -1);
                1
            }
            Instruction::JumpNotZero(v, offset) => {
                if v.get(register) == 0 {
                    1
                } else {
                    offset.get(register)
                }
            }
            Instruction::Toggle(offset) => offset.get(register),
            Instruction::Out(signal) => {
                if output_receiver.receive(signal.get(register), register) {
                    i64::MIN // abort
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
            "cpy" => Ok(Instruction::Copy(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "inc" => Ok(Instruction::Increment(parsed.get_argument(0)?)),
            "dec" => Ok(Instruction::Decrement(parsed.get_argument(0)?)),
            "jnz" => Ok(Instruction::JumpNotZero(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "tgl" => Ok(Instruction::Toggle(parsed.get_argument(0)?)),
            "out" => Ok(Instruction::Out(parsed.get_argument(0)?)),
            _ => Err(parse_error!("Unknown command: {}", parsed)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Copy(a, b) => write!(f, "cpy {} {}", a, b),
            Instruction::Increment(a) => write!(f, "inc {}", a),
            Instruction::Decrement(a) => write!(f, "dec {}", a),
            Instruction::JumpNotZero(a, b) => write!(f, "jnz {} {}", a, b),
            Instruction::Toggle(a) => write!(f, "tgl {}", a),
            Instruction::Out(a) => write!(f, "out {}", a),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}
