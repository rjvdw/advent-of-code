use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

pub const MODULUS: u64 = 2147483647;
pub const A_FACTOR: u64 = 16807;
pub const B_FACTOR: u64 = 48271;

#[derive(Debug, Copy, Clone)]
pub struct Generator {
    factor: u64,
    value: u64,
}

impl Generator {
    pub fn new_a(value: u64) -> Generator {
        Generator {
            factor: A_FACTOR,
            value,
        }
    }

    pub fn new_b(value: u64) -> Generator {
        Generator {
            factor: B_FACTOR,
            value,
        }
    }

    fn name(&self) -> char {
        match self.factor {
            A_FACTOR => 'A',
            B_FACTOR => 'B',
            _ => unreachable!(),
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * self.factor) % MODULUS;
        Some(self.value)
    }
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Generator {} starts with {}", self.name(), self.value)
    }
}

impl FromStr for Generator {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Err(parse_error!("Invalid input line: {}", s));

        if let Some(r) = s.strip_prefix("Generator ") {
            if let Some(name) = r.chars().next() {
                if let Some(idx) = r.rfind(' ') {
                    let value = r[idx + 1..].parse()?;
                    result = match name {
                        'A' => Ok(Generator::new_a(value)),
                        'B' => Ok(Generator::new_b(value)),
                        _ => Err(parse_error!("Invalid generator name: {}", name)),
                    };
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_five_values_a() {
        let mut generator = Generator::new_a(65);

        assert_eq!(generator.next(), Some(1092455));
        assert_eq!(generator.next(), Some(1181022009));
        assert_eq!(generator.next(), Some(245556042));
        assert_eq!(generator.next(), Some(1744312007));
        assert_eq!(generator.next(), Some(1352636452));
    }

    #[test]
    fn test_first_five_values_b() {
        let mut generator = Generator::new_b(8921);

        assert_eq!(generator.next(), Some(430625591));
        assert_eq!(generator.next(), Some(1233683848));
        assert_eq!(generator.next(), Some(1431495498));
        assert_eq!(generator.next(), Some(137874439));
        assert_eq!(generator.next(), Some(285222916));
    }
}
