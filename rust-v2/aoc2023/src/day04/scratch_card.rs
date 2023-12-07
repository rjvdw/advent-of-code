use std::collections::HashSet;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::parser::{
    parse_whitespace_separated_to_hashset, parse_whitespace_separated_to_vec,
};

#[derive(Debug, Clone)]
pub struct ScratchCard {
    id: usize,
    winning: HashSet<u32>,
    yours: Vec<u32>,
}

impl ScratchCard {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn count_matching_numbers(&self) -> usize {
        self.yours
            .iter()
            .filter(|nr| self.winning.contains(nr))
            .count()
    }

    pub fn naive_score(&self) -> u32 {
        let mut score = 0;
        for nr in &self.yours {
            if self.winning.contains(nr) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
}

impl FromStr for ScratchCard {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.strip_prefix("Card ").ok_or(())?.trim();
        let pos1 = line.find(':').ok_or(())?;
        let pos2 = line.find('|').ok_or(())?;

        let id = line[..pos1].parse::<usize>()?;
        let winning = parse_whitespace_separated_to_hashset(&line[pos1 + 1..pos2])?;
        let yours = parse_whitespace_separated_to_vec(&line[pos2 + 1..])?;

        Ok(ScratchCard { id, winning, yours })
    }
}
