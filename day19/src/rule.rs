use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use helpers::ParseError;

use crate::rules_map::RulesMap;

pub enum Rule {
    Simple(char),
    Compound(Vec<usize>),
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
        let mut results = HashSet::new();

        match self {
            Rule::Simple(ch) => {
                if line.chars().nth(offset) == Some(*ch) {
                    results.insert(offset + 1);
                }
            }
            Rule::Compound(rules) => {
                match rules.split_first() {
                    Some((rule_key, rest)) => {
                        let rest_rule = Rule::Compound(rest.to_vec());
                        let rule = rules_map.get_rule(rule_key);

                        for next_offset in rule.match_line(rules_map, line, offset) {
                            let m = rest_rule.match_line(rules_map, line, next_offset);
                            results.extend(m);
                        }
                    }
                    None => {
                        results.insert(offset);
                    }
                };
            }
            Rule::Either(rules) => {
                for rule in rules {
                    results.extend(rule.match_line(rules_map, line, offset));
                }
            }
        }

        results
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
            let mut rules = Vec::new();
            for part in s.split(' ') {
                rules.push(part.parse::<usize>()?);
            }
            Ok(Rule::Compound(rules))
        }
    }
}
