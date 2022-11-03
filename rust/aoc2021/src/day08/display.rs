use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

const SEPARATOR: &str = " | ";
const A: u8 = 0b0000_0001;
const B: u8 = 0b0000_0010;
const C: u8 = 0b0000_0100;
const D: u8 = 0b0000_1000;
const E: u8 = 0b0001_0000;
const F: u8 = 0b0010_0000;
const G: u8 = 0b0100_0000;
const ALL_SEGMENTS: [u8; 7] = [A, B, C, D, E, F, G];

#[derive(Debug, Copy, Clone)]
pub struct Display {
    pub digits: [u8; 10],
    pub output: [u8; 4],
}

impl Display {
    pub fn get_output(&self, mapping: [u8; 7]) -> Result<u32, ()> {
        let [a, b, c, d, e, f, g] = mapping;

        let mut decoded = 0;
        for segments in self.output {
            let digit = match segments {
                x if x == (a | b | c | e | f | g) => 0,
                x if x == (c | f) => 1,
                x if x == (a | c | d | e | g) => 2,
                x if x == (a | c | d | f | g) => 3,
                x if x == (b | c | d | f) => 4,
                x if x == (a | b | d | f | g) => 5,
                x if x == (a | b | d | e | f | g) => 6,
                x if x == (a | c | f) => 7,
                x if x == (a | b | c | d | e | f | g) => 8,
                x if x == (a | b | c | d | f | g) => 9,
                _ => return Err(()), // invalid digit
            };
            decoded *= 10;
            decoded += digit;
        }
        Ok(decoded)
    }
}

impl FromStr for Display {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = if let Some(p) = s.find(SEPARATOR) {
            p
        } else {
            return Err(parse_error!("Invalid input: {}", s));
        };

        let mut digits = [0u8; 10];
        for (idx, digit) in s[..p].split_whitespace().enumerate() {
            digits[idx] = map_str_to_u8(digit);
        }

        let mut output = [0u8; 4];
        for (idx, digit) in s[p + SEPARATOR.len()..].split_whitespace().enumerate() {
            output[idx] = map_str_to_u8(digit);
        }

        Ok(Display { digits, output })
    }
}

pub fn count_segments(digit: u8) -> usize {
    ALL_SEGMENTS
        .iter()
        .filter(|&&segment| segment & digit != 0)
        .count()
}

fn map_str_to_u8(input: &str) -> u8 {
    let mut digit = 0u8;
    for ch in input.chars() {
        digit |= match ch {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => 0,
        };
    }
    digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_output_1() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [
                A | B | C | E | F | G,
                C | F,
                A | C | D | E | G,
                A | C | D | F | G,
            ],
        };

        assert_eq!(display.get_output([A, B, C, D, E, F, G]), Ok(123));
    }

    #[test]
    fn test_get_output_2() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [
                B | C | D | F,
                A | B | D | F | G,
                A | B | D | E | F | G,
                A | C | F,
            ],
        };

        assert_eq!(display.get_output([A, B, C, D, E, F, G]), Ok(4567));
    }

    #[test]
    fn test_get_output_3() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [
                A | B | C | D | E | F | G,
                A | B | C | D | E | F | G,
                A | B | C | D | F | G,
                A | B | C | D | F | G,
            ],
        };

        assert_eq!(display.get_output([A, B, C, D, E, F, G]), Ok(8899));
    }

    #[test]
    fn test_get_output_invalid() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [F | G, F | G, F | G, F | G],
        };

        assert_eq!(display.get_output([A, B, C, D, E, F, G]), Err(()));
    }

    #[test]
    fn test_get_output_with_valid_mapping() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [
                B | D | E | F | A,
                B | D | E | G | A,
                C | D | E | G,
                B | C | E | G | A,
            ],
        };

        assert_eq!(display.get_output([B, C, D, E, F, G, A]), Ok(2345));
    }

    #[test]
    fn test_get_output_with_invalid_mapping() {
        let display = Display {
            digits: [0; 10], // not relevant for this test
            output: [0; 4],  // not relevant for this test
        };

        assert_eq!(display.get_output([1u8; 7]), Err(()));
    }
}
