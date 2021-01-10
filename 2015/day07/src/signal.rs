use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

use crate::instruction::Instruction;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Signal {
    Wire(u16),
    Value(u16),
}

impl Signal {
    pub fn evaluate(
        &self,
        map: &HashMap<Signal, Instruction>,
        evaluated: &mut HashMap<Signal, u16>,
    ) -> Option<u16> {
        if let Some(value) = evaluated.get(self) {
            Some(*value)
        } else {
            match self {
                Signal::Value(value) => {
                    evaluated.insert(*self, *value);
                    Some(*value)
                }
                wire => map.get(wire)?.evaluate(map, evaluated),
            }
        }
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Signal::Wire(label) => {
                let label = *label;
                let ch1 = ((label >> 8) as u8) as char;
                let ch2 = (label as u8) as char;

                if ch1 == '\0' {
                    write!(f, "{}", ch2)
                } else {
                    write!(f, "{}{}", ch1, ch2)
                }
            }
            Signal::Value(value) => write!(f, "{}", *value),
        }
    }
}

impl FromStr for Signal {
    type Err = ParseError;

    fn from_str(label: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = label.parse::<u16>() {
            Ok(Signal::Value(value))
        } else {
            let mut chars = label.chars();

            if label.len() == 1 {
                Ok(Signal::Wire(chars.next().unwrap() as u16))
            } else if label.len() == 2 {
                let ch1 = chars.next().unwrap() as u16;
                let ch2 = chars.next().unwrap() as u16;

                Ok(Signal::Wire((ch1 << 8) + ch2))
            } else {
                err_parse_error!("Invalid label: {}", label)
            }
        }
    }
}
