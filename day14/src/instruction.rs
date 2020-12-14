use std::str::FromStr;

use helpers::ParseError;

#[derive(Debug)]
pub enum Instruction {
    SetMask(String),
    WriteValue(usize, u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(" = ") {
            Some(idx) => {
                let value = &s[(idx + 3)..];

                if s.starts_with("mask") {
                    Ok(Instruction::SetMask(value.to_string()))
                } else if s.starts_with("mem") {
                    match s.find(']') {
                        Some(idx) => {
                            let address = s[4..idx].parse::<usize>()?;
                            let value = value.parse::<u64>()?;

                            Ok(Instruction::WriteValue(address, value))
                        }
                        None => Err(ParseError(format!("Invalid line: {}", s))),
                    }
                } else {
                    Err(ParseError(format!("Invalid line: {}", s)))
                }
            }
            None => Err(ParseError(format!("Invalid line: {}", s))),
        }
    }
}
