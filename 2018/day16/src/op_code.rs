use std::collections::HashSet;
use std::fmt;

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
