use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Instruction {
    // not used atm
    // pub fn is_acc(&self) -> bool {
    //     matches!(self, Instruction::Acc(_))
    // }

    pub fn is_jmp(&self) -> bool {
        matches!(self, Instruction::Jmp(_))
    }

    pub fn is_nop(&self) -> bool {
        matches!(self, Instruction::Nop(_))
    }
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _output_receiver: &mut O,
    ) -> i64 {
        match self {
            Instruction::Acc(value) => {
                register.increment('a', *value);
                1
            }
            Instruction::Jmp(value) => *value,
            Instruction::Nop(_) => 1,
        }
    }

    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError> {
        match parsed.get_command() {
            "acc" => Ok(Instruction::Acc(parsed.get_argument(0)?)),
            "jmp" => Ok(Instruction::Jmp(parsed.get_argument(0)?)),
            "nop" => Ok(Instruction::Nop(parsed.get_argument(0)?)),
            _ => Err(ParseError(format!("Unknown command: {}", parsed))),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}
