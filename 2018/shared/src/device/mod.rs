use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::device::instruction::Instruction;

pub mod hook;
pub mod instruction;

pub fn parse_instructions<R: Read>(readable: R) -> Result<(i64, Vec<Instruction>), ParseError> {
    let mut lines = BufReader::new(readable).lines();
    if let Some(Ok(line)) = lines.next() {
        if let Some(instruction_pointer) = line.strip_prefix("#ip ") {
            let mut instructions = Vec::new();
            for line in lines {
                let line = line?;
                instructions.push(line.parse()?);
            }
            Ok((instruction_pointer.parse()?, instructions))
        } else {
            Err(parse_error!("Invalid instruction pointer: {}", line))
        }
    } else {
        Err(parse_error!("Empty input file"))
    }
}
