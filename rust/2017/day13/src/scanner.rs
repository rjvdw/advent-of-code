use std::collections::HashMap;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone)]
pub struct Scanner {
    depth: u64,
    range: u64,
}

impl Scanner {
    pub fn new(depth: u64, range: u64) -> Scanner {
        Scanner { depth, range }
    }

    pub fn get_visualisation_data(scanners: &[Scanner]) -> (HashMap<u64, u64>, u64, u64) {
        let mut map = HashMap::new();
        let mut max_depth = 0;
        let mut max_range = 0;

        for scanner in scanners {
            if scanner.depth > max_depth {
                max_depth = scanner.depth;
            }
            if scanner.range > max_range {
                max_range = scanner.range;
            }
            map.insert(scanner.depth, scanner.range);
        }

        (map, max_depth, max_range)
    }

    fn period(&self) -> u64 {
        (self.range - 1) * 2
    }

    pub fn position(&self, delay: u64) -> u64 {
        (self.depth + delay) % self.period()
    }

    pub fn severity(&self) -> u64 {
        self.depth * self.range
    }

    pub fn will_detect_you(&self, t: u64) -> bool {
        t % self.period() == 0
    }
}

impl FromStr for Scanner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut scanner = Scanner { depth: 0, range: 0 };
        for (idx, nr) in s.split(": ").enumerate() {
            match idx {
                0 => {
                    scanner.depth = nr.parse()?;
                }
                1 => {
                    scanner.range = nr.parse()?;
                }
                _ => {
                    return Err(parse_error!("Invalid scanner: {}", s));
                }
            }
        }
        Ok(scanner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_period() {
        let scanner = Scanner::new(4, 5);
        assert_eq!(scanner.period(), 8);
    }

    #[test]
    fn test_position() {
        let scanner = Scanner::new(4, 5);
        assert_eq!(scanner.position(3), 7);
        assert_eq!(scanner.position(8), 4);
    }

    #[test]
    fn test_severity() {
        let scanner = Scanner::new(4, 5);
        assert_eq!(scanner.severity(), 20);
    }

    #[test]
    fn test_will_detect_you() {
        let scanner = Scanner::new(4, 5);
        assert!(scanner.will_detect_you(0));
        assert!(!scanner.will_detect_you(3));
        assert!(scanner.will_detect_you(8));
    }
}
