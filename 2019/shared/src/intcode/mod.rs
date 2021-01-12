//! Implementation of the Intcode machine.

use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::intcode::program_parse_error::ProgramParseError;

pub mod program_parse_error;

/// An Intcode program.
#[derive(Debug, Clone)]
pub struct Program {
    memory: Vec<usize>,
    instruction_pointer: usize,
}

impl Program {
    /// Create a new program, with some initial memory.
    pub fn new(memory: Vec<usize>) -> Program {
        Program {
            memory,
            instruction_pointer: 0,
        }
    }

    /// Run the program until it halts.
    pub fn run(&mut self) {
        loop {
            match self.memory[self.instruction_pointer] {
                1 => {
                    let a = self.arg(1);
                    let b = self.arg(2);
                    let c = self.arg(3);
                    self.memory[c] = self.memory[a] + self.memory[b];
                    self.instruction_pointer += 4;
                }
                2 => {
                    let a = self.arg(1);
                    let b = self.arg(2);
                    let c = self.arg(3);
                    self.memory[c] = self.memory[a] * self.memory[b];
                    self.instruction_pointer += 4;
                }
                99 => break,
                _ => unreachable!(),
            };
        }
    }

    fn arg(&self, offset: usize) -> usize {
        self.memory[self.instruction_pointer + offset]
    }

    /// Return a memory dump.
    pub fn dump(&self) -> Vec<usize> {
        self.memory.clone()
    }
}

impl Index<usize> for Program {
    type Output = usize;

    /// Read the memory at a specific index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

impl FromStr for Program {
    type Err = ProgramParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut int_codes = vec![];
        for nr in line.split(',') {
            int_codes.push(nr.parse()?);
        }
        Ok(Program::new(int_codes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let mut program = Program::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        program.run();
        assert_eq!(
            program.dump(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run_2() {
        let mut program = Program::new(vec![1, 0, 0, 0, 99]);
        program.run();
        assert_eq!(program.dump(), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_3() {
        let mut program = Program::new(vec![2, 3, 0, 3, 99]);
        program.run();
        assert_eq!(program.dump(), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_4() {
        let mut program = Program::new(vec![2, 4, 4, 5, 99, 0]);
        program.run();
        assert_eq!(program.dump(), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_5() {
        let mut program = Program::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        program.run();
        assert_eq!(program.dump(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_parse() {
        let program: Program = "1,0,0,3,99".parse().unwrap();
        assert_eq!(program.dump(), vec![1, 0, 0, 3, 99]);
    }
}
