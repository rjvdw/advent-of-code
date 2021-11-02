use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[rustfmt::skip]
const BIT_FLAGS: [u8; 10] = [
    // rows
    0b01000000,
    0b00100000,
    0b00010000,
    0b00001000,
    0b00000100,
    0b00000010,
    0b00000001,

    // cols
    0b00000100,
    0b00000010,
    0b00000001,
];

#[derive(Debug, Eq, PartialEq)]
pub struct BoardingPass(pub u8, pub u8);

impl BoardingPass {
    pub fn get_seat_id(&self) -> u16 {
        (self.0 as u16) * 8 + (self.1 as u16)
    }
}

impl FromStr for BoardingPass {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'B' {
                row |= BIT_FLAGS[i];
            } else if c == 'R' {
                col |= BIT_FLAGS[i];
            }
        }

        Ok(BoardingPass(row, col))
    }
}
