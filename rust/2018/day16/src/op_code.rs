use std::collections::HashSet;
use std::fmt;

use rdcl_aoc_helpers::machine::instruction::MachineInstruction;
use rdcl_aoc_helpers::machine::output_receiver::NoopOutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;

use shared::device::instruction::{reg, Instruction};

use crate::sample::Sample;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OpCode {
    Addr,
    Addi,

    Mulr,
    Muli,

    Banr,
    Bani,

    Borr,
    Bori,

    Setr,
    Seti,

    Gtir,
    Gtri,
    Gtrr,

    Eqir,
    Eqri,
    Eqrr,

    Unknown,
}

impl OpCode {
    pub fn is_unknown(&self) -> bool {
        matches!(self, OpCode::Unknown)
    }

    pub fn test<R: MachineRegister>(&self, sample: &Sample, register: &mut R) -> bool {
        if let Some(instruction) = self.as_instruction(
            sample.instruction[1],
            sample.instruction[2],
            sample.instruction[3],
        ) {
            for i in 0..4 {
                register.write(reg(i as i64), sample.before[i]);
            }
            instruction.execute(register, &mut NoopOutputReceiver);
            (0..4).all(|i| register.read(reg(i as i64)) == sample.after[i])
        } else {
            false
        }
    }

    pub fn as_instruction(&self, a: i64, b: i64, c: i64) -> Option<Instruction> {
        match self {
            OpCode::Addr => Some(Instruction::Addr(a, b, c)),
            OpCode::Addi => Some(Instruction::Addi(a, b, c)),
            OpCode::Mulr => Some(Instruction::Mulr(a, b, c)),
            OpCode::Muli => Some(Instruction::Muli(a, b, c)),
            OpCode::Banr => Some(Instruction::Banr(a, b, c)),
            OpCode::Bani => Some(Instruction::Bani(a, b, c)),
            OpCode::Borr => Some(Instruction::Borr(a, b, c)),
            OpCode::Bori => Some(Instruction::Bori(a, b, c)),
            OpCode::Setr => Some(Instruction::Setr(a, b, c)),
            OpCode::Seti => Some(Instruction::Seti(a, b, c)),
            OpCode::Gtir => Some(Instruction::Gtir(a, b, c)),
            OpCode::Gtri => Some(Instruction::Gtri(a, b, c)),
            OpCode::Gtrr => Some(Instruction::Gtrr(a, b, c)),
            OpCode::Eqir => Some(Instruction::Eqir(a, b, c)),
            OpCode::Eqri => Some(Instruction::Eqri(a, b, c)),
            OpCode::Eqrr => Some(Instruction::Eqrr(a, b, c)),
            OpCode::Unknown => None,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Addr => write!(f, "addr"),
            OpCode::Addi => write!(f, "addi"),
            OpCode::Mulr => write!(f, "mulr"),
            OpCode::Muli => write!(f, "muli"),
            OpCode::Banr => write!(f, "banr"),
            OpCode::Bani => write!(f, "bani"),
            OpCode::Borr => write!(f, "borr"),
            OpCode::Bori => write!(f, "bori"),
            OpCode::Setr => write!(f, "setr"),
            OpCode::Seti => write!(f, "seti"),
            OpCode::Gtir => write!(f, "gtir"),
            OpCode::Gtri => write!(f, "gtri"),
            OpCode::Gtrr => write!(f, "gtrr"),
            OpCode::Eqir => write!(f, "eqir"),
            OpCode::Eqri => write!(f, "eqri"),
            OpCode::Eqrr => write!(f, "eqrr"),
            OpCode::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn all_op_codes() -> HashSet<OpCode> {
    let mut op_codes = HashSet::new();
    op_codes.insert(OpCode::Addr);
    op_codes.insert(OpCode::Addi);
    op_codes.insert(OpCode::Mulr);
    op_codes.insert(OpCode::Muli);
    op_codes.insert(OpCode::Banr);
    op_codes.insert(OpCode::Bani);
    op_codes.insert(OpCode::Borr);
    op_codes.insert(OpCode::Bori);
    op_codes.insert(OpCode::Setr);
    op_codes.insert(OpCode::Seti);
    op_codes.insert(OpCode::Gtir);
    op_codes.insert(OpCode::Gtri);
    op_codes.insert(OpCode::Gtrr);
    op_codes.insert(OpCode::Eqir);
    op_codes.insert(OpCode::Eqri);
    op_codes.insert(OpCode::Eqrr);
    op_codes
}
