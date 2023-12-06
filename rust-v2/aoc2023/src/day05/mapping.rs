use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;

pub trait Mappable {
    fn apply_to_nr(&self, nr: u64) -> u64;

    fn apply_to_ranges(&self, ranges: &[(u64, u64)]) -> Vec<(u64, u64)>;
}

pub trait Mappings {
    fn parse<T>(input: &mut Peekable<T>) -> Result<Vec<Mapping>, ParseError>
    where
        T: Iterator<Item = String>;
}

impl Mappable for Vec<Mapping> {
    fn apply_to_nr(&self, nr: u64) -> u64 {
        self.iter()
            .fold(nr, |acc, mapping| mapping.apply_to_nr(acc))
    }

    fn apply_to_ranges(&self, ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
        self.iter().fold(ranges.to_vec(), |acc, mapping| {
            mapping.apply_to_ranges(&acc)
        })
    }
}

impl Mappings for Vec<Mapping> {
    fn parse<T>(mut input: &mut Peekable<T>) -> Result<Vec<Mapping>, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut mappings: Vec<Mapping> = vec![];
        while input.peek().is_some() {
            mappings.push(Mapping::parse(&mut input)?);
        }
        Ok(mappings)
    }
}

#[derive(Debug, Clone)]
pub struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    pub fn parse<T>(input: &mut T) -> Result<Mapping, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut mapping = Mapping { ranges: vec![] };

        input.next(); // the first line contains the name of the mapping and may be skipped
        for line in input {
            if line.is_empty() {
                break;
            }
            mapping.ranges.push(line.parse()?);
        }

        Ok(mapping)
    }
}

impl Mappable for Mapping {
    fn apply_to_nr(&self, nr: u64) -> u64 {
        self.ranges
            .iter()
            .map(|range| range.apply(nr))
            .filter(|o| o.is_some())
            .flatten()
            .next()
            .unwrap_or(nr)
    }

    fn apply_to_ranges(&self, source_range: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut unmatched = source_range.to_vec();
        let mut matched = vec![];
        for mapping_range in &self.ranges {
            let mut next_unmatched = vec![];
            for range in unmatched {
                let result = mapping_range.apply_to_range(range);

                if let Some(v) = result.unmatched_left {
                    next_unmatched.push(v);
                }
                if let Some(v) = result.unmatched_right {
                    next_unmatched.push(v);
                }
                if let Some(v) = result.matched {
                    matched.push(v);
                }
            }
            unmatched = next_unmatched;
        }
        for range in unmatched {
            matched.push(range);
        }
        matched
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct ApplyToRangeResult {
    unmatched_left: Option<(u64, u64)>,
    unmatched_right: Option<(u64, u64)>,
    matched: Option<(u64, u64)>,
}

#[derive(Debug, Clone)]
struct MappingRange {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
    source_range: Range<u64>,
}

impl MappingRange {
    fn apply(&self, nr: u64) -> Option<u64> {
        if self.source_range.contains(&nr) {
            let offset = nr - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }

    fn apply_to_range(&self, (start, end): (u64, u64)) -> ApplyToRangeResult {
        let mut result = ApplyToRangeResult {
            unmatched_left: None,
            unmatched_right: None,
            matched: None,
        };

        let source_range_start = self.source_range_start;
        let source_range_end = source_range_start + self.range_length;

        if end < source_range_start {
            // no overlap at all
            result.unmatched_left = Some((start, end));
        } else if start >= source_range_end {
            // no overlap at all
            result.unmatched_right = Some((start, end));
        } else if start >= source_range_start && end <= source_range_end {
            // full overlap
            let offset = start - source_range_start;
            let size = end - start;
            let start = self.destination_range_start + offset;
            let end = start + size;
            result.matched = Some((start, end));
        } else if start < source_range_start && end > source_range_end {
            // full overlap
            result.unmatched_left = Some((start, source_range_start));
            result.unmatched_right = Some((source_range_end, end));
            result.matched = Some((
                self.destination_range_start,
                self.destination_range_start + self.range_length,
            ));
        } else if start < source_range_start {
            // partial overlap at the start
            result.unmatched_left = Some((start, source_range_start));
            let size = end - source_range_start;
            let start = self.destination_range_start;
            let end = start + size;
            result.matched = Some((start, end));
        } else {
            // partial overlap at the end
            result.unmatched_right = Some((source_range_end, end));
            let start = self.destination_range_start + start - source_range_start;
            let end = self.destination_range_start + self.range_length;
            result.matched = Some((start, end));
        }

        result
    }
}

impl FromStr for MappingRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos1 = s.find(' ').ok_or(())?;
        let pos2 = s.rfind(' ').ok_or(())?;

        let destination_range_start = s[0..pos1].parse()?;
        let source_range_start = s[pos1 + 1..pos2].parse()?;
        let range_length = s[pos2 + 1..].parse()?;

        Ok(MappingRange {
            destination_range_start,
            source_range_start,
            range_length,
            source_range: source_range_start..source_range_start + range_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_apply() {
        let mut input = "test map:
            50 98 2
            52 50 48
            "
        .lines()
        .map(|line| line.trim().to_string());

        let mapping = Mapping::parse(&mut input).unwrap();

        assert_eq!(mapping.apply_to_nr(79), 81);
        assert_eq!(mapping.apply_to_nr(14), 14);
        assert_eq!(mapping.apply_to_nr(55), 57);
        assert_eq!(mapping.apply_to_nr(13), 13);
        assert_eq!(mapping.apply_to_nr(98), 50);
        assert_eq!(mapping.apply_to_nr(99), 51);
        assert_eq!(mapping.apply_to_nr(100), 100);
    }

    #[test]
    fn test_mappings_apply() {
        let mut input = "test map:
            50 98 2
            52 50 48

            test map:
            0 15 37
            37 52 2
            39 0 15
            "
        .lines()
        .map(|line| line.trim().to_string())
        .peekable();

        let mappings = <Vec<Mapping>>::parse(&mut input).unwrap();

        assert_eq!(mappings.apply_to_nr(79), 81);
        assert_eq!(mappings.apply_to_nr(14), 53);
        assert_eq!(mappings.apply_to_nr(55), 57);
        assert_eq!(mappings.apply_to_nr(13), 52);
        assert_eq!(mappings.apply_to_nr(98), 35);
        assert_eq!(mappings.apply_to_nr(99), 36);
        assert_eq!(mappings.apply_to_nr(100), 100);
    }

    #[test]
    fn test_mapping_range_apply() {
        let range = "52 50 48".parse::<MappingRange>().unwrap();

        assert_eq!(range.apply(79), Some(81));
        assert_eq!(range.apply(14), None);
        assert_eq!(range.apply(55), Some(57));
        assert_eq!(range.apply(13), None);
    }

    #[test]
    fn test_mapping_range_apply_to_range() {
        let range = "50 20 10".parse::<MappingRange>().unwrap();

        assert_eq!(
            range.apply_to_range((0, 10)),
            ApplyToRangeResult {
                unmatched_left: Some((0, 10)),
                unmatched_right: None,
                matched: None,
            }
        );
        assert_eq!(
            range.apply_to_range((15, 25)),
            ApplyToRangeResult {
                unmatched_left: Some((15, 20)),
                unmatched_right: None,
                matched: Some((50, 55)),
            }
        );
        assert_eq!(
            range.apply_to_range((20, 30)),
            ApplyToRangeResult {
                unmatched_left: None,
                unmatched_right: None,
                matched: Some((50, 60)),
            }
        );
        assert_eq!(
            range.apply_to_range((15, 35)),
            ApplyToRangeResult {
                unmatched_left: Some((15, 20)),
                unmatched_right: Some((30, 35)),
                matched: Some((50, 60)),
            }
        );
        assert_eq!(
            range.apply_to_range((25, 35)),
            ApplyToRangeResult {
                unmatched_left: None,
                unmatched_right: Some((30, 35)),
                matched: Some((55, 60)),
            }
        );
    }

    #[test]
    fn test_multiple_mapping_ranges_applied_to_range() {
        let mut input = "test map:
            45 77 23
            81 45 19
            68 64 13
            "
        .lines()
        .map(|line| line.trim().to_string())
        .peekable();

        let mappings = <Vec<Mapping>>::parse(&mut input).unwrap();
        let ranges = vec![(74, 88), (54, 63), (46, 50)];

        // 74..88
        //   - matches with 45 77 23 and 68 64 13:
        //     - split into 74..77 and 77..88
        //     - map 74..77 to 78..81
        //     - map 77..88 to 45..56
        //     - end up with 74..77 and 45..56
        // 54..64
        //   - matches with 81 45 19:
        //     - map 54..63 to 90..99
        // 46..50
        //   - matches with 81 45 19:
        //     - map 46..50 to 82..86

        let mut actual = mappings.apply_to_ranges(&ranges);
        actual.sort_by_key(|v| v.0);

        assert_eq!(actual, vec![(45, 56), (78, 81), (82, 86), (90, 99)]);
    }
}
