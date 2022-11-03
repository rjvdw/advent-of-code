use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

impl Range {
    pub fn new(from: i64, to: i64) -> Range {
        Range { from, to }
    }

    /// Creates a partition of `self` such that for every element in the partition the element is
    /// either completely disjoint from `other`, or completely overlaps.
    pub fn partition(&self, other: &Range) -> Vec<Range> {
        if self.to < other.from || self.from > other.to {
            // completely disjoint
            vec![*self]
        } else {
            let mut partitions = vec![
                // the part of `self` that overlaps with `other`
                Range::new(
                    [self.from, other.from].iter().max().copied().unwrap(),
                    [self.to, other.to].iter().min().copied().unwrap(),
                ),
            ];

            // the part of `self` that extends to the left of `other`
            if self.from < other.from {
                partitions.push(Range::new(self.from, other.from - 1));
            }

            // the part of `self` that extends to the right of `other`
            if self.to > other.to {
                partitions.push(Range::new(other.to + 1, self.to));
            }

            partitions
        }
    }

    pub fn is_disjoint(&self, other: &Range) -> bool {
        self.to < other.from || self.from > other.to
    }

    pub fn fits_within(&self, other: &Range) -> bool {
        self.from >= other.from && self.to <= other.to
    }

    pub fn size(&self) -> usize {
        (self.to - self.from) as usize + 1
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.from, self.to)
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = if let Some(v) = s.find("..") {
            v
        } else {
            return Err(parse_error!("Invalid range: {}", s));
        };

        let from = s[..i].parse()?;
        let to = s[i + 2..].parse()?;
        if from > to {
            return Err(parse_error!("Invalid range: {}", s));
        }

        Ok(Range { from, to })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(
            Range::new(0, 10).partition(&Range::new(5, 15)),
            vec![Range::new(5, 10), Range::new(0, 4)],
        );

        assert_eq!(
            Range::new(0, 10).partition(&Range::new(15, 25)),
            vec![Range::new(0, 10)],
        );

        assert_eq!(
            Range::new(0, 10).partition(&Range::new(2, 7)),
            vec![Range::new(2, 7), Range::new(0, 1), Range::new(8, 10)],
        );

        assert_eq!(
            Range::new(2, 7).partition(&Range::new(0, 10)),
            vec![Range::new(2, 7)],
        );
    }

    #[test]
    fn test_fits_within() {
        let range_a = Range::new(-5, 5);
        let range_b = Range::new(-10, 10);
        let range_c = Range::new(0, 10);

        assert!(range_a.fits_within(&range_a));
        assert!(range_a.fits_within(&range_b));
        assert!(!range_a.fits_within(&range_c));
        assert!(!range_b.fits_within(&range_a));
        assert!(range_b.fits_within(&range_b));
        assert!(!range_b.fits_within(&range_c));
        assert!(!range_c.fits_within(&range_a));
        assert!(range_c.fits_within(&range_b));
        assert!(range_c.fits_within(&range_c));
    }

    #[test]
    fn test_size() {
        assert_eq!(Range::new(-7, 12).size(), 20);
        assert_eq!(Range::new(7, 12).size(), 6);
    }

    #[test]
    fn test_from_str() {
        let range = "5..10".parse::<Range>().unwrap();
        assert_eq!(range.from, 5);
        assert_eq!(range.to, 10);

        let range = "invalid".parse::<Range>();
        assert!(range.is_err());
    }
}
