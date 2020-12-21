use std::num::ParseIntError;
use std::str::FromStr;

use helpers::parse_error::ParseError;

const VALUE_SEPARATOR: &str = " = ";
const SET_MASK_KEYWORD: &str = "mask";
const WRITE_VALUE_KEYWORD: &str = "mem";

#[derive(Debug)]
pub enum Instruction {
    SetMask(u64, u64),
    WriteValue(usize, u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(VALUE_SEPARATOR) {
            Some(idx) => {
                let start = idx + VALUE_SEPARATOR.len();
                let value = &s[start..];

                if s.starts_with(SET_MASK_KEYWORD) {
                    let (or_mask, and_mask) = parse_mask(value)?;
                    Ok(Instruction::SetMask(or_mask, and_mask))
                } else if s.starts_with(WRITE_VALUE_KEYWORD) {
                    let start = WRITE_VALUE_KEYWORD.len() + 1;
                    let end = idx - 1;
                    let address = s[start..end].parse::<usize>()?;
                    let value = value.parse::<u64>()?;
                    Ok(Instruction::WriteValue(address, value))
                } else {
                    Err(ParseError(format!("Invalid line: {}", s)))
                }
            }
            None => Err(ParseError(format!("Invalid line: {}", s))),
        }
    }
}

fn parse_mask(mask: &str) -> Result<(u64, u64), ParseIntError> {
    Ok((
        u64::from_str_radix(mask.replace('X', "0").as_str(), 2)?,
        u64::from_str_radix(mask.replace('X', "1").as_str(), 2)?,
    ))
}
