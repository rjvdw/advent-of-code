use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::machine::instruction::{MachineInstruction, ParsedMachineInstruction};
use rdcl_aoc_helpers::machine::output_receiver::OutputReceiver;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Addr(i64, i64, i64),
    Addi(i64, i64, i64),

    Mulr(i64, i64, i64),
    Muli(i64, i64, i64),

    Banr(i64, i64, i64),
    Bani(i64, i64, i64),

    Borr(i64, i64, i64),
    Bori(i64, i64, i64),

    Setr(i64, i64, i64),
    Seti(i64, i64, i64),

    Gtir(i64, i64, i64),
    Gtri(i64, i64, i64),
    Gtrr(i64, i64, i64),

    Eqir(i64, i64, i64),
    Eqri(i64, i64, i64),
    Eqrr(i64, i64, i64),
}

impl MachineInstruction for Instruction {
    fn execute<R: MachineRegister, O: OutputReceiver<R>>(
        &self,
        register: &mut R,
        _: &mut O,
    ) -> i64 {
        match self {
            Instruction::Addr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write(c, a + b);
            }
            Instruction::Addi(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write(c, a + b);
            }
            Instruction::Mulr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write(c, a * b);
            }
            Instruction::Muli(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write(c, a * b);
            }
            Instruction::Banr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write(c, a & b);
            }
            Instruction::Bani(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write(c, a & b);
            }
            Instruction::Borr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write(c, a | b);
            }
            Instruction::Bori(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write(c, a | b);
            }
            Instruction::Setr(a, _, c) => {
                let a = register.read(reg(*a));
                let c = reg(*c);
                register.write(c, a);
            }
            Instruction::Seti(a, _, c) => {
                let a = *a;
                let c = reg(*c);
                register.write(c, a);
            }
            Instruction::Gtir(a, b, c) => {
                let a = *a;
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write_bool(c, a > b);
            }
            Instruction::Gtri(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write_bool(c, a > b);
            }
            Instruction::Gtrr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write_bool(c, a > b);
            }
            Instruction::Eqir(a, b, c) => {
                let a = *a;
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write_bool(c, a == b);
            }
            Instruction::Eqri(a, b, c) => {
                let a = register.read(reg(*a));
                let b = *b;
                let c = reg(*c);
                register.write_bool(c, a == b);
            }
            Instruction::Eqrr(a, b, c) => {
                let a = register.read(reg(*a));
                let b = register.read(reg(*b));
                let c = reg(*c);
                register.write_bool(c, a == b);
            }
        }

        1
    }

    fn from_parsed_machine_instruction(
        parsed: &ParsedMachineInstruction,
    ) -> Result<Self, ParseError> {
        let a = parsed.get_argument(0)?;
        let b = parsed.get_argument(1)?;
        let c = parsed.get_argument(2)?;

        match parsed.get_command() {
            "addr" => Ok(Instruction::Addr(a, b, c)),
            "addi" => Ok(Instruction::Addi(a, b, c)),
            "mulr" => Ok(Instruction::Mulr(a, b, c)),
            "muli" => Ok(Instruction::Muli(a, b, c)),
            "banr" => Ok(Instruction::Banr(a, b, c)),
            "bani" => Ok(Instruction::Bani(a, b, c)),
            "borr" => Ok(Instruction::Borr(a, b, c)),
            "bori" => Ok(Instruction::Bori(a, b, c)),
            "setr" => Ok(Instruction::Setr(a, b, c)),
            "seti" => Ok(Instruction::Seti(a, b, c)),
            "gtir" => Ok(Instruction::Gtir(a, b, c)),
            "gtri" => Ok(Instruction::Gtri(a, b, c)),
            "gtrr" => Ok(Instruction::Gtrr(a, b, c)),
            "eqir" => Ok(Instruction::Eqir(a, b, c)),
            "eqri" => Ok(Instruction::Eqri(a, b, c)),
            "eqrr" => Ok(Instruction::Eqrr(a, b, c)),
            _ => Err(parse_error!("Invalid instruction: {}", parsed)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Addr(a, b, c) => write!(f, "addr {} {} {}", a, b, c),
            Instruction::Addi(a, b, c) => write!(f, "addi {} {} {}", a, b, c),
            Instruction::Mulr(a, b, c) => write!(f, "mulr {} {} {}", a, b, c),
            Instruction::Muli(a, b, c) => write!(f, "muli {} {} {}", a, b, c),
            Instruction::Banr(a, b, c) => write!(f, "banr {} {} {}", a, b, c),
            Instruction::Bani(a, b, c) => write!(f, "bani {} {} {}", a, b, c),
            Instruction::Borr(a, b, c) => write!(f, "borr {} {} {}", a, b, c),
            Instruction::Bori(a, b, c) => write!(f, "bori {} {} {}", a, b, c),
            Instruction::Setr(a, b, c) => write!(f, "setr {} {} {}", a, b, c),
            Instruction::Seti(a, b, c) => write!(f, "seti {} {} {}", a, b, c),
            Instruction::Gtir(a, b, c) => write!(f, "gtir {} {} {}", a, b, c),
            Instruction::Gtri(a, b, c) => write!(f, "gtri {} {} {}", a, b, c),
            Instruction::Gtrr(a, b, c) => write!(f, "gtrr {} {} {}", a, b, c),
            Instruction::Eqir(a, b, c) => write!(f, "eqir {} {} {}", a, b, c),
            Instruction::Eqri(a, b, c) => write!(f, "eqri {} {} {}", a, b, c),
            Instruction::Eqrr(a, b, c) => write!(f, "eqrr {} {} {}", a, b, c),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as MachineInstruction>::from_str(s)
    }
}

pub fn reg(key: i64) -> char {
    if (0..26).contains(&key) {
        (key as u8 + b'a') as char
    } else {
        panic!("Cannot use key {} as a register.", key)
    }
}
