//! The error that gets returned if your program fails to parse.

use std::num::ParseIntError;

/// Possible ways in which parsing the program can fail.
#[derive(Debug)]
pub enum ProgramParseError {
    /// An invalid memory value was encountered.
    InvalidMemoryValue,

    /// The program was empty.
    EmptyProgram,
}

impl From<ParseIntError> for ProgramParseError {
    fn from(_: ParseIntError) -> Self {
        ProgramParseError::InvalidMemoryValue
    }
}
