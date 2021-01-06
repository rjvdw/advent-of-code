use crate::op_code::OpCode;
use crate::register::Register;
use crate::sample::Sample;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Addr(usize, usize, usize),
    Addi(usize, usize, usize),

    Mulr(usize, usize, usize),
    Muli(usize, usize, usize),

    Banr(usize, usize, usize),
    Bani(usize, usize, usize),

    Borr(usize, usize, usize),
    Bori(usize, usize, usize),

    Setr(usize, usize, usize),
    Seti(usize, usize, usize),

    Gtir(usize, usize, usize),
    Gtri(usize, usize, usize),
    Gtrr(usize, usize, usize),

    Eqir(usize, usize, usize),
    Eqri(usize, usize, usize),
    Eqrr(usize, usize, usize),
}

impl Instruction {
    pub fn run(&self, register: &mut Register) {
        match self {
            Instruction::Addr(a, b, c) => register.set(c, register.get(a) + register.get(b)),
            Instruction::Addi(a, b, c) => register.set(c, register.get(a) + b),
            Instruction::Mulr(a, b, c) => register.set(c, register.get(a) * register.get(b)),
            Instruction::Muli(a, b, c) => register.set(c, register.get(a) * b),
            Instruction::Banr(a, b, c) => register.set(c, register.get(a) & register.get(b)),
            Instruction::Bani(a, b, c) => register.set(c, register.get(a) & b),
            Instruction::Borr(a, b, c) => register.set(c, register.get(a) | register.get(b)),
            Instruction::Bori(a, b, c) => register.set(c, register.get(a) | b),
            Instruction::Setr(a, _, c) => register.set(c, register.get(a)),
            Instruction::Seti(a, _, c) => register.set(c, *a),
            Instruction::Gtir(a, b, c) => register.set_bool(c, *a > register.get(b)),
            Instruction::Gtri(a, b, c) => register.set_bool(c, register.get(a) > *b),
            Instruction::Gtrr(a, b, c) => register.set_bool(c, register.get(a) > register.get(b)),
            Instruction::Eqir(a, b, c) => register.set_bool(c, *a == register.get(b)),
            Instruction::Eqri(a, b, c) => register.set_bool(c, register.get(a) == *b),
            Instruction::Eqrr(a, b, c) => register.set_bool(c, register.get(a) == register.get(b)),
        }
    }

    pub fn test_sample(op_code: OpCode, sample: &Sample, register: &mut Register) -> bool {
        if let Some(instruction) = Self::from(
            op_code,
            sample.instruction[1],
            sample.instruction[2],
            sample.instruction[3],
        ) {
            register.clear();
            for i in 0..4 {
                register.set(&i, sample.before[i]);
            }
            instruction.run(register);
            (0..4).all(|i| register.get(&i) == sample.after[i])
        } else {
            false
        }
    }

    pub fn from(op_code: OpCode, a: usize, b: usize, c: usize) -> Option<Instruction> {
        match op_code {
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
