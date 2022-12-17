use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

use crate::label::Label;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Valve {
    label: Label,
    flow_rate: u32,
}

impl Valve {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn flow_rate(&self) -> u32 {
        self.flow_rate
    }
}

impl FromStr for Valve {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub = match s.strip_prefix("Valve ") {
            Some(s) => s,
            None => {
                return err_parse_error!("Invalid valve: {}", s);
            }
        };

        let label = sub[..2].parse::<Label>()?;

        let sub = match sub[2..].strip_prefix(" has flow rate=") {
            Some(s) => s,
            None => {
                return err_parse_error!("Invalid valve: {}", s);
            }
        };

        let flow_rate = sub.parse()?;

        Ok(Valve { label, flow_rate })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "Valve AA has flow rate=0".parse::<Valve>().unwrap(),
            Valve {
                label: Label('A', 'A'),
                flow_rate: 0,
            }
        );
    }
}
