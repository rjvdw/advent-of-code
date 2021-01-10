use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

use crate::screen::Screen;

#[derive(Debug)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Instruction {
    pub fn execute(&self, screen: &mut Screen) {
        match self {
            Instruction::Rect(width, height) => {
                for row in 0..*height {
                    for column in 0..*width {
                        screen.turn_on(row, column);
                    }
                }
            }
            Instruction::RotateRow(row, by) => {
                screen.shift_row(*row, *by);
            }
            Instruction::RotateColumn(column, by) => {
                screen.shift_column(*column, *by);
            }
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Rect(width, height) => write!(f, "rect {}x{}", width, height),
            Instruction::RotateRow(y, by) => write!(f, "rotate row y={} by {}", y, by),
            Instruction::RotateColumn(x, by) => write!(f, "rotate column x={} by {}", x, by),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix("rect ") {
            if let Some(idx) = r.find('x') {
                let width = r[..idx].parse()?;
                let height = r[idx + 1..].parse()?;

                return Ok(Instruction::Rect(width, height));
            }
        } else if let Some(r) = s.strip_prefix("rotate row ") {
            if let (Some(idx1), Some(idx2)) = (r.find(' '), r.rfind(' ')) {
                let y = r[2..idx1].parse()?;
                let by = r[idx2 + 1..].parse()?;

                return Ok(Instruction::RotateRow(y, by));
            }
        } else if let Some(r) = s.strip_prefix("rotate column ") {
            if let (Some(idx1), Some(idx2)) = (r.find(' '), r.rfind(' ')) {
                let x = r[2..idx1].parse()?;
                let by = r[idx2 + 1..].parse()?;

                return Ok(Instruction::RotateColumn(x, by));
            }
        }

        err_parse_error!("Invalid input: {}", s)
    }
}
