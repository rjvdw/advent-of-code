use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use helpers::ParseError;

use crate::rules_map::RulesMap;

pub enum Rule {
    Simple(char),
    Ref(usize),
    Compound(usize, Box<Rule>),
    Either(Vec<Rule>),
}

impl Rule {
    pub fn test(&self, rules_map: &HashMap<usize, Rule>, line: &str) -> bool {
        self.match_line(rules_map, line, 0)
            .iter()
            .any(|&matched_len| matched_len == line.len())
    }

    fn match_line(
        &self,
        rules_map: &HashMap<usize, Rule>,
        line: &str,
        offset: usize,
    ) -> HashSet<usize> {
        match self {
            Rule::Simple(ch) => {
                let mut results = HashSet::new();
                if line.chars().nth(offset) == Some(*ch) {
                    results.insert(offset + 1);
                }
                results
            }
            Rule::Ref(rule_key) => rules_map
                .get_rule(rule_key)
                .match_line(rules_map, line, offset),
            Rule::Compound(rule_key, rest) => {
                let mut results = HashSet::new();
                let rule = rules_map.get_rule(rule_key);
                for next_offset in rule.match_line(rules_map, line, offset) {
                    if next_offset != line.len() {
                        results.extend(rest.match_line(rules_map, line, next_offset));
                    }
                }
                results
            }
            Rule::Either(rules) => rules
                .iter()
                .flat_map(|rule| rule.match_line(rules_map, line, offset))
                .collect(),
        }
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            match s.chars().nth(1) {
                Some(ch) => Ok(Rule::Simple(ch)),
                None => Err(ParseError(format!("Invalid simple rule: {}", s))),
            }
        } else if s.contains('|') {
            let mut rules = Vec::new();
            for part in s.split(" | ") {
                rules.push(part.parse::<Rule>()?);
            }
            Ok(Rule::Either(rules))
        } else {
            match s.find(' ') {
                Some(idx) => {
                    let first = s[..idx].parse::<usize>()?;
                    let tail = s[idx + 1..].parse::<Rule>()?;

                    Ok(Rule::Compound(first, Box::new(tail)))
                }
                None => Ok(Rule::Ref(s.parse::<usize>()?)),
            }
        }
    }
}
