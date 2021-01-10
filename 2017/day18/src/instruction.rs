use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction, Value};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Sound(Value),
    Set(char, Value),
    Add(char, Value),
    Multiply(char, Value),
    Modulo(char, Value),
    Recover(char),
    JumpGreaterThanZero(Value, Value),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _output_receiver: &mut O,
    ) -> i64 {
        match self {
            Instruction::Sound(_) => unimplemented!(),
            Instruction::Set(a, b) => {
                register.write(*a, b.get(register));
                1
            }
            Instruction::Add(a, b) => {
                register.increment(*a, b.get(register));
                1
            }
            Instruction::Multiply(a, b) => {
                let v_a = register.read(*a);
                let v_b = b.get(register);
                register.write(*a, v_a * v_b);
                1
            }
            Instruction::Modulo(a, b) => {
                let v_a = register.read(*a);
                let v_b = b.get(register);
                register.write(*a, v_a % v_b);
                1
            }
            Instruction::Recover(_) => unimplemented!(),
            Instruction::JumpGreaterThanZero(a, b) => {
                if a.get(register) > 0 {
                    b.get(register)
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
            "snd" => Ok(Instruction::Sound(parsed.get_argument(0)?)),
            "set" => Ok(Instruction::Set(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "add" => Ok(Instruction::Add(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "mul" => Ok(Instruction::Multiply(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "mod" => Ok(Instruction::Modulo(
                parsed.get_argument(0)?,
                parsed.get_argument(1)?,
            )),
            "rcv" => Ok(Instruction::Recover(parsed.get_argument(0)?)),
            "jgz" => Ok(Instruction::JumpGreaterThanZero(
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
            Instruction::Sound(a) => write!(f, "snd {}", a),
            Instruction::Set(a, b) => write!(f, "set {} {}", a, b),
            Instruction::Add(a, b) => write!(f, "add {} {}", a, b),
            Instruction::Multiply(a, b) => write!(f, "mul {} {}", a, b),
            Instruction::Modulo(a, b) => write!(f, "mod {} {}", a, b),
            Instruction::Recover(a) => write!(f, "rcv {}", a),
            Instruction::JumpGreaterThanZero(a, b) => write!(f, "jgz {} {}", a, b),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Instruction as MachineInstruction>::from_str(s)
    }
}
