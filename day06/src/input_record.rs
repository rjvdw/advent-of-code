use std::collections::HashMap;
use std::fmt;

use helpers::FromMultilineStr;

#[derive(Debug)]
pub struct InputRecord {
    nr_of_people_in_group: usize,
    answered_with_yes: HashMap<char, usize>,
}

impl InputRecord {
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

#[derive(Debug)]
pub struct InputRecordError {
    msg: String,
}

impl fmt::Display for InputRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromMultilineStr for InputRecord {
    type Err = InputRecordError;

    fn new() -> Self {
        InputRecord {
            nr_of_people_in_group: 0,
            answered_with_yes: HashMap::with_capacity(26),
        }
    }

    fn indicates_new_record(line: &String) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &String) -> Result<(), Self::Err> {
        if !line.is_empty() {
            self.nr_of_people_in_group += 1;
            for c in line.chars() {
                *self.answered_with_yes.entry(c).or_insert(0) += 1;
            }
        }

        Ok(())
    }
}
