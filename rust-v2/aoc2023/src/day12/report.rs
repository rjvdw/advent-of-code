use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Clone, Default)]
pub struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    pub fn unfold(&self, copies: usize) -> Record {
        let mut unfolded = Record::default();

        for i in 0..copies {
            if i != 0 {
                unfolded.springs.push(Spring::Unknown);
            }
            unfolded.springs.append(&mut self.springs.clone());
            unfolded.groups.append(&mut self.groups.clone());
        }

        unfolded
    }

    pub fn count_correct_configurations(&self) -> usize {
        let mut record = self.clone();
        if record.springs[0] != Spring::Operational {
            record.springs.insert(0, Spring::Operational);
        }
        if record.springs[record.springs.len() - 1] != Spring::Operational {
            record.springs.push(Spring::Operational);
        }

        record.find_possible_fillings(0, 0, &mut HashMap::new())
    }

    /// Recursively find all possible ways in which the specified groups can fit in this record.
    fn find_possible_fillings(
        &self,
        index: usize,
        group_index: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(&count) = cache.get(&(index, group_index)) {
            return count;
        } else if group_index >= self.groups.len() {
            // all groups have been matched, check if there are any remaining damaged springs
            return if self
                .springs
                .iter()
                .skip(index)
                .any(|&spring| spring == Spring::Damaged)
            {
                0
            } else {
                1
            };
        }

        let mut count = 0;
        let group = self.groups[group_index] + 2;
        let max = self.springs.len()
            - self
                .groups
                .iter()
                .skip(group_index)
                .map(|x| x + 1)
                .sum::<usize>();

        for i in index..max {
            if self.springs[i] == Spring::Damaged {
                // damaged spring encountered which does not correspond with a group, so no point in continuing
                break;
            } else if self.group_fits(i, group) {
                count += self.find_possible_fillings(i + group - 1, group_index + 1, cache);
            }
        }

        cache.insert((index, group_index), count);
        count
    }

    /// Checks whether at a given point a group with a given length will fit.
    ///
    /// - `index` - The index at which to check.
    /// - `len` - The length of the group (i.e. 5 would correspond to `.###.`).
    fn group_fits(&self, index: usize, len: usize) -> bool {
        self.springs[index].can_be_operational()
            && self.springs[index + 1..index + len - 1]
                .iter()
                .all(|s| s.can_be_damaged())
            && self.springs[index + len - 1].can_be_operational()
    }
}

impl FromStr for Record {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut record = Record::default();
        let pos = s.find(' ').ok_or(())?;

        for ch in s[0..pos].chars() {
            record.springs.push(match ch {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => {
                    return err_parse_error!("Invalid symbol: {}", ch);
                }
            });
        }

        for s in s[pos + 1..].split(',') {
            record.groups.push(s.parse()?);
        }

        Ok(record)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &spring in &self.springs {
            write!(f, "{spring}")?;
        }

        let groups = self
            .groups
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        write!(f, " {groups}")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn can_be_operational(&self) -> bool {
        matches![self, Spring::Operational | Spring::Unknown]
    }

    fn can_be_damaged(&self) -> bool {
        matches![self, Spring::Damaged | Spring::Unknown]
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Spring::Operational => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_correct_configurations_1() {
        let record = record("???.### 1,1,3");
        assert_eq!(record.count_correct_configurations(), 1);
    }

    #[test]
    fn test_count_correct_configurations_2() {
        let record = record(".??..??...?##. 1,1,3");
        assert_eq!(record.count_correct_configurations(), 4);
    }

    #[test]
    fn test_count_correct_configurations_3() {
        let record = record("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(record.count_correct_configurations(), 1);
    }

    #[test]
    fn test_count_correct_configurations_4() {
        let record = record("????.#...#... 4,1,1");
        assert_eq!(record.count_correct_configurations(), 1);
    }

    #[test]
    fn test_count_correct_configurations_5() {
        let record = record("????.######..#####. 1,6,5");
        assert_eq!(record.count_correct_configurations(), 4);
    }

    #[test]
    fn test_count_correct_configurations_6() {
        let record = record("?###???????? 3,2,1");
        assert_eq!(record.count_correct_configurations(), 10);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_1() {
        let record = record("???.### 1,1,3").unfold(5);
        assert_eq!(record.count_correct_configurations(), 1);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_2() {
        let record = record(".??..??...?##. 1,1,3").unfold(5);
        assert_eq!(record.count_correct_configurations(), 16384);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_3() {
        let record = record("?#?#?#?#?#?#?#? 1,3,1,6").unfold(5);
        assert_eq!(record.count_correct_configurations(), 1);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_4() {
        let record = record("????.#...#... 4,1,1").unfold(5);
        assert_eq!(record.count_correct_configurations(), 16);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_5() {
        let record = record("????.######..#####. 1,6,5").unfold(5);
        assert_eq!(record.count_correct_configurations(), 2500);
    }

    #[test]
    fn test_count_correct_configurations_after_unfolding_6() {
        let record = record("?###???????? 3,2,1").unfold(5);
        assert_eq!(record.count_correct_configurations(), 506250);
    }

    fn record(input: &str) -> Record {
        input.parse::<Record>().unwrap()
    }
}
