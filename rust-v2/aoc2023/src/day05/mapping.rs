use std::iter::Peekable;
use std::ops::Range;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;

pub trait Mappings {
    fn parse<T>(input: &mut Peekable<T>) -> Result<Vec<Mapping>, ParseError>
    where
        T: Iterator<Item = String>;
    fn apply(&self, nr: u64) -> u64;
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

    fn apply(&self, nr: u64) -> u64 {
        let mut result = nr;
        for mapping in self {
            result = mapping.apply(result);
        }
        result
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

    pub fn apply(&self, nr: u64) -> u64 {
        for range in &self.ranges {
            if let Some(v) = range.apply(nr) {
                return v;
            }
        }

        nr
    }
}

#[derive(Debug, Clone)]
struct MappingRange {
    destination_range_start: u64,
    source_range_start: u64,
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
}

impl FromStr for MappingRange {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos1 = s.find(' ').ok_or(())?;
        let pos2 = s.rfind(' ').ok_or(())?;

        let destination_range_start = s[0..pos1].parse()?;
        let source_range_start = s[pos1 + 1..pos2].parse()?;
        let range_length = s[pos2 + 1..].parse::<u64>()?;

        Ok(MappingRange {
            destination_range_start,
            source_range_start,
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

        assert_eq!(mapping.apply(79), 81);
        assert_eq!(mapping.apply(14), 14);
        assert_eq!(mapping.apply(55), 57);
        assert_eq!(mapping.apply(13), 13);
        assert_eq!(mapping.apply(98), 50);
        assert_eq!(mapping.apply(99), 51);
        assert_eq!(mapping.apply(100), 100);
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

        assert_eq!(mappings.apply(79), 81);
        assert_eq!(mappings.apply(14), 53);
        assert_eq!(mappings.apply(55), 57);
        assert_eq!(mappings.apply(13), 52);
        assert_eq!(mappings.apply(98), 35);
        assert_eq!(mappings.apply(99), 36);
        assert_eq!(mappings.apply(100), 100);
    }

    #[test]
    fn test_mapping_range_apply() {
        let range = "52 50 48".parse::<MappingRange>().unwrap();

        assert_eq!(range.apply(79), Some(81));
        assert_eq!(range.apply(14), None);
        assert_eq!(range.apply(55), Some(57));
        assert_eq!(range.apply(13), None);
    }
}
