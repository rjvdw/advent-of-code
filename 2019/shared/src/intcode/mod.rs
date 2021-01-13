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
    relative_base: i64,
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
            relative_base: 0,
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
        if self.has_halted() {
            panic!("Cannot run a program that has been halted.");
        }

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
                5 => {
                    // if [a] != 0, go to [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    if a != 0 {
                        self.instruction_pointer = b;
                    }
                }
                6 => {
                    // if [a] == 0, go to [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    if a == 0 {
                        self.instruction_pointer = b;
                    }
                }
                7 => {
                    // [c] = [a] < [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    self.write_bool_arg(&mut modes, a < b);
                }
                8 => {
                    // [c] = [a] == [b]
                    let a = self.read_arg(&mut modes);
                    let b = self.read_arg(&mut modes);
                    self.write_bool_arg(&mut modes, a == b);
                }
                9 => {
                    // <relative base> += [a]
                    self.relative_base += self.read_arg(&mut modes);
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
        let relative_base = self.relative_base;
        match mode {
            0 => self[arg],
            1 => arg,
            2 => self[relative_base + arg],
            _ => unreachable!(),
        }
    }

    /// Reads the current position to find out where to write, and moves the instruction pointer one
    /// position.
    fn write_arg(&mut self, modes: &mut i64, value: i64) {
        let (arg, mode) = self.get_arg_and_mode(modes);
        let relative_base = self.relative_base;
        match mode {
            0 => {
                self[arg] = value;
            }
            2 => {
                self[relative_base + arg] = value;
            }
            _ => unreachable!(),
        }
    }

    /// Reads the current position to find out where to write, and moves the instruction pointer one
    /// position.
    fn write_bool_arg(&mut self, modes: &mut i64, value: bool) {
        self.write_arg(modes, if value { 1 } else { 0 })
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
    pub fn memory_dump(&self) -> Vec<i64> {
        self.memory.clone()
    }

    /// Return an output dump.
    pub fn output_dump(&self) -> Vec<i64> {
        self.outbox.iter().copied().collect()
    }

    /// Check if the program has halted.
    pub fn has_halted(&self) -> bool {
        matches!(self.status, ProgramStatus::Halted)
    }
}

impl Index<i64> for Program {
    type Output = i64;

    /// Read the memory at a specific index.
    fn index(&self, index: i64) -> &Self::Output {
        let index = usize::try_from(index).unwrap();
        if index < self.memory.len() {
            &self.memory[index]
        } else {
            &0
        }
    }
}

impl IndexMut<i64> for Program {
    /// Allows you to modify the memory at a specific index.
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        let index = usize::try_from(index).unwrap();
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        &mut self.memory[index]
    }
}

impl FromStr for Program {
    type Err = ProgramParseError;

    /// Parse an intcode program from a comma separated list of ints.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut int_codes = vec![];
        for nr in line.split(',') {
            int_codes.push(nr.parse()?);
        }
        Ok(Program::new(int_codes))
    }
}

/// Allows you to easily define programs from within your code. Useful for unit tests.
#[macro_export]
macro_rules! program {
    ($($x:expr),+ $(,)?) => (
        $crate::intcode::Program::new(vec![$($x),*])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_1() {
        let mut program = program![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(
            program.memory_dump(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }

    #[test]
    fn test_run_2() {
        let mut program = program![1, 0, 0, 0, 99];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.memory_dump(), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_3() {
        let mut program = program![2, 3, 0, 3, 99];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.memory_dump(), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_4() {
        let mut program = program![2, 4, 4, 5, 99, 0];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.memory_dump(), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_5() {
        let mut program = program![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.memory_dump(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_6() {
        let mut program = program![1002, 4, 3, 4, 33];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.memory_dump(), vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_run_7() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0
        // (if it is not).
        let mut program = program![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        program.send_message(8);
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.output_dump(), vec![1]);
    }

    #[test]
    fn test_run_8() {
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0
        // (if it is not).
        let mut program = program![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        program.send_message(5);
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.output_dump(), vec![1]);
    }

    #[test]
    fn test_run_9() {
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0
        // (if it is not).
        let mut program = program![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        program.send_message(10);
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.output_dump(), vec![0]);
    }

    #[test]
    fn test_run_10() {
        let mut program =
            program![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(
            program.output_dump(),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }

    #[test]
    fn test_run_11() {
        let mut program = program![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.output_dump(), vec![34915192 * 34915192]);
    }

    #[test]
    fn test_run_12() {
        let mut program = program![104, 1125899906842624, 99];
        assert_eq!(program.run(), ProgramStatus::Halted);
        assert_eq!(program.output_dump(), vec![1125899906842624]);
    }

    #[test]
    fn test_parse() {
        let program: Program = "1,0,0,3,99".parse().unwrap();
        assert_eq!(program.memory_dump(), vec![1, 0, 0, 3, 99]);
    }
}
