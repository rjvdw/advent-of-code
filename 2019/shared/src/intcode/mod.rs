//! Implementation of the Intcode machine.

use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use crate::intcode::program_parse_error::ProgramParseError;
use crate::intcode::program_status::ProgramStatus;

pub mod parse;
pub mod program_parse_error;
pub mod program_status;

/// An Intcode program.
#[derive(Debug, Clone)]
pub struct Program {
    memory: Vec<i64>,
    instruction_pointer: i64,
    inbox: VecDeque<i64>,
    outbox: VecDeque<i64>,
    status: ProgramStatus,
}

impl Program {
    /// Create a new program, with some initial memory.
    pub fn new(memory: Vec<i64>) -> Program {
        Program {
            memory,
            instruction_pointer: 0,
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
            status: ProgramStatus::Paused,
        }
    }

    /// Send a message to the program.
    pub fn send_message(&mut self, message: i64) {
        self.inbox.push_back(message);
    }

    /// Receive a message from the program.
    pub fn receive_message(&mut self) -> Option<i64> {
        self.outbox.pop_front()
    }

    /// Run the program until it halts.
    pub fn run(&mut self) -> ProgramStatus {
        self.status = ProgramStatus::Running;
        loop {
            let (op, mut modes) = self.read_instruction();
            match op {
                1 => {
                    // [c] = [a] + [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    self.write_arg(&mut modes, a + b);
                }
                2 => {
                    // [c] = [a] * [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    self.write_arg(&mut modes, a * b);
                }
                3 => {
                    // [a] = <in>
                    if let Some(message) = self.inbox.pop_front() {
                        self.write_arg(&mut modes, message);
                    } else {
                        // noop, stay on the current instruction
                        self.instruction_pointer -= 1;
                        self.status = ProgramStatus::Paused;
                        break;
                    }
                }
                4 => {
                    // <out> = [a]
                    let a = self.read_arg(&mut modes);
                    self.outbox.push_back(a);
                }
                99 => {
                    // HALT
                    self.status = ProgramStatus::Halted;
                    break;
                }
                _ => unreachable!(),
            };
        }
        self.status
    }

    /// Reads an instruction, and moves the instruction pointer one position.
    fn read_instruction(&mut self) -> (i64, i64) {
        let op = self[self.instruction_pointer] % 100;
        let modes = self[self.instruction_pointer] / 100;
        self.instruction_pointer += 1;
        (op, modes)
    }

    /// Reads an argument, and moves the instruction pointer one position.
    fn read_arg(&mut self, modes: &mut i64) -> i64 {
        let (arg, mode) = self.get_arg_and_mode(modes);
        match mode {
            0 => self[arg],
            1 => arg,
            _ => unreachable!(),
        }
    }

    /// Reads the current position to find out where to write, and moves the instruction pointer one
    /// position.
    fn write_arg(&mut self, modes: &mut i64, value: i64) {
        let (arg, mode) = self.get_arg_and_mode(modes);
        match mode {
            0 => {
                self[arg] = value;
            }
            _ => unreachable!(),
        }
    }

    /// Reads the current position, and works out the mode for this position. Moves the instruction
    /// pointer one position.
    fn get_arg_and_mode(&mut self, modes: &mut i64) -> (i64, i64) {
        let arg = self[self.instruction_pointer];
        self.instruction_pointer += 1;

        let mode = *modes % 10;
        *modes /= 10;
        (arg, mode)
    }

    /// Return a memory dump.
    pub fn dump(&self) -> Vec<i64> {
        self.memory.clone()
    }
}

impl Index<i64> for Program {
    type Output = i64;

    /// Read the memory at a specific index.
    fn index(&self, index: i64) -> &Self::Output {
        let index = usize::try_from(index).unwrap();
        &self.memory[index]
    }
}

impl IndexMut<i64> for Program {
    /// Allows you to modify the memory at a specific index.
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let index = usize::try_from(index).unwrap();
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
    fn test_run_6() {
        let mut program = Program::new(vec![1002, 4, 3, 4, 33]);
        program.run();
        assert_eq!(program.dump(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_parse() {
        let program: Program = "1,0,0,3,99".parse().unwrap();
        assert_eq!(program.dump(), vec![1, 0, 0, 3, 99]);
    }
}
