use std::io::{BufRead, BufReader, Read};

use crate::intcode::program_parse_error::ProgramParseError;
use crate::intcode::Program;

/// Read a program from a readable.
pub fn parse_input<R: Read>(r: R) -> Result<Program, ProgramParseError> {
    match BufReader::new(r).lines().next() {
        Some(Ok(line)) => line.parse(),
        _ => Err(ProgramParseError::EmptyProgram),
    }
}
