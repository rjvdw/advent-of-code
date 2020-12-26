use std::collections::HashMap;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

#[derive(Debug)]
pub struct Group {
    nr_of_people_in_group: usize,
    answered_with_yes: HashMap<char, usize>,
}

impl Group {
    pub fn nr_of_questions_anyone_answered_with_yes(&self) -> usize {
        self.answered_with_yes.len()
    }

    pub fn nr_of_questions_everyone_answered_with_yes(&self) -> usize {
        self.answered_with_yes
            .values()
            .filter(|&&v| v == self.nr_of_people_in_group)
            .count()
    }
}

impl MultilineFromStr for Group {
    type Err = ParseError;

    fn new() -> Self {
        Group {
            nr_of_people_in_group: 0,
            answered_with_yes: HashMap::with_capacity(26),
        }
    }

    fn indicates_new_record(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if !line.is_empty() {
            self.nr_of_people_in_group += 1;
            for c in line.chars() {
                *self.answered_with_yes.entry(c).or_insert(0) += 1;
            }
        }

        Ok(())
    }
}
