use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

use crate::parts::{Part, Rating, NR_RATINGS};

pub const IN: &str = "in";
pub const ACCEPTED: &str = "A";
pub const REJECTED: &str = "R";

pub type Workflows = HashMap<String, Workflow>;

pub trait Acceptable {
    /// Check if a part is accepted.
    fn accepts(&self, part: &Part) -> bool;

    /// Count how many potential parts there are that will be accepted.
    /// The ratings of the part are limited by `bounds` (inclusive).
    fn count_acceptable(&self, bounds: (usize, usize)) -> usize;
}

impl Acceptable for Workflows {
    fn accepts(&self, part: &Part) -> bool {
        let mut workflow_label = IN.to_string();
        let accepted = ACCEPTED.to_string();
        let rejected = REJECTED.to_string();

        loop {
            let workflow = self
                .get(&workflow_label)
                .expect("encountered a non-existing workflow");

            workflow_label = workflow.check(part);

            if workflow_label == accepted {
                return true;
            }
            if workflow_label == rejected {
                return false;
            }
        }
    }

    fn count_acceptable(&self, bounds: (usize, usize)) -> usize {
        // Start by finding all possible ways a part can be accepted.
        // This is done by traversing all possible paths allowed by the
        // workflows and eliminating all paths that end in a rejection.
        let paths = find_all_acceptable_paths(self);

        // Given all possible paths that lead to an accepted outcome,
        // find which ratings will be accepted by the workflows. Every
        // path will produce a four-dimensional hypercube (where the
        // four dimensions are the ratings X, M, A and S). All parts
        // where the ratings fall within this hypercube will be accepted
        // by the workflows, all parts that fall outside of this
        // hypercube will not be accepted by following this specific
        // path. The part might of course still be accepted by a
        // different path.
        let rule_bounds = paths
            .iter()
            .map(|path| compute_part_bounds(path, bounds))
            .collect::<Vec<_>>();

        // Compute the number of distinct parts that will be accepted by
        // the workflows. Paths are always mutually exclusive, so there
        // no need to take duplicates into account.
        rule_bounds
            .iter()
            .map(|rb| rb.iter().map(|&(l, u)| u - l + 1).product::<usize>())
            .sum()
    }
}

/// Find all paths that lead to an A (accepted) outcome.
fn find_all_acceptable_paths(workflows: &Workflows) -> Vec<Vec<Rule>> {
    let accepted = ACCEPTED.to_string();
    let rejected = REJECTED.to_string();

    let mut traversing: VecDeque<(String, Vec<Rule>)> = VecDeque::new();
    traversing.push_back((IN.to_string(), vec![]));
    let mut found: Vec<Vec<Rule>> = vec![];

    while let Some((label, path)) = traversing.pop_front() {
        let workflow = workflows
            .get(&label)
            .expect("encountered a non-existing workflow");

        let mut path = path.clone();
        for rule in &workflow.rules {
            let to = rule.get_to();
            if to == accepted {
                let mut path = path.clone();
                path.push(rule.clone());
                found.push(path);
            } else if to != rejected {
                let mut next = path.clone();
                next.push(rule.clone());
                traversing.push_back((to, next));
            }
            path.push(rule.negate());
        }
    }

    found
}

type RatingBounds = [(usize, usize); NR_RATINGS];

/// Find which values will be accepted by this sequence of rules. That
/// is, find bounds for all ratings so that parts will be accepted iff
/// their ratings fall within these bounds (inclusive).
fn compute_part_bounds(rules: &[Rule], initial: (usize, usize)) -> RatingBounds {
    [
        find_bounds_for_rating(rules, initial, Rating::X),
        find_bounds_for_rating(rules, initial, Rating::M),
        find_bounds_for_rating(rules, initial, Rating::A),
        find_bounds_for_rating(rules, initial, Rating::S),
    ]
}

fn find_bounds_for_rating(
    rules: &[Rule],
    initial: (usize, usize),
    rating: Rating,
) -> (usize, usize) {
    rules
        .iter()
        .filter(|rule| rule.applies_to(rating))
        .fold(initial, |bounds, rule| rule.restrict(bounds))
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    fn check(&self, part: &Part) -> String {
        for rule in &self.rules {
            if let Some(to) = rule.check(part) {
                return to;
            }
        }

        unreachable!()
    }

    /// Verify that this workflow will always match something.
    fn is_valid(&self) -> bool {
        !self.rules.is_empty()
            && self
                .rules
                .iter()
                .take(self.rules.len() - 1)
                .all(|rule| !rule.is_else())
            && self.rules[self.rules.len() - 1].is_else()
    }
}

impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.find('{').ok_or(())?;
        let mut workflow = Workflow {
            name: s[0..idx].to_string(),
            rules: vec![],
        };

        for rule in s[idx + 1..s.len() - 1].split(',') {
            workflow.rules.push(rule.parse()?);
        }

        if workflow.is_valid() {
            Ok(workflow)
        } else {
            err_parse_error!("Invalid workflow: {}", s)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rule {
    Less(Rating, usize, String),
    Greater(Rating, usize, String),
    Always(String),
}

impl Rule {
    fn check(&self, part: &Part) -> Option<String> {
        match self {
            Rule::Less(rating, v, to) if part[*rating] < *v => Some(to.clone()),
            Rule::Greater(rating, v, to) if part[*rating] > *v => Some(to.clone()),
            Rule::Always(to) => Some(to.clone()),
            _ => None,
        }
    }

    fn is_else(&self) -> bool {
        matches![self, Rule::Always(_)]
    }

    fn negate(&self) -> Rule {
        match self {
            Rule::Less(rating, value, to) => Rule::Greater(*rating, *value - 1, to.clone()),
            Rule::Greater(rating, value, to) => Rule::Less(*rating, *value + 1, to.clone()),
            Rule::Always(to) => Rule::Always(to.clone()),
        }
    }

    fn applies_to(&self, rating: Rating) -> bool {
        match self {
            Rule::Less(r, _, _) if *r != rating => false,
            Rule::Greater(r, _, _) if *r != rating => false,
            _ => true,
        }
    }

    fn get_to(&self) -> String {
        match self {
            Rule::Less(_, _, to) => to.clone(),
            Rule::Greater(_, _, to) => to.clone(),
            Rule::Always(to) => to.clone(),
        }
    }

    fn restrict(&self, bounds: (usize, usize)) -> (usize, usize) {
        match self {
            Rule::Less(_, value, _) if bounds.1 >= *value => (bounds.0, *value - 1),
            Rule::Greater(_, value, _) if bounds.0 <= *value => (*value + 1, bounds.1),
            _ => bounds,
        }
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find(':') {
            let rating = s[0..1].parse::<Rating>()?;
            let operator = s.as_bytes().get(1).copied().ok_or(())?;

            if operator == b'<' {
                Ok(Rule::Less(
                    rating,
                    s[2..idx].parse()?,
                    s[idx + 1..].to_string(),
                ))
            } else if operator == b'>' {
                Ok(Rule::Greater(
                    rating,
                    s[2..idx].parse()?,
                    s[idx + 1..].to_string(),
                ))
            } else {
                err_parse_error!("Invalid operator: {}", operator)
            }
        } else {
            Ok(Rule::Always(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parts::parse_part;

    use super::*;

    #[test]
    fn test_workflow_check() {
        let part = part("{x=5}");

        assert_eq!(workflow("foo{x<10:A,R}").check(&part), ACCEPTED.to_string());
        assert_eq!(workflow("foo{x>10:A,R}").check(&part), REJECTED.to_string());
    }

    #[test]
    fn test_rule_check() {
        let part = part("{x=5}");
        let expected = Some(ACCEPTED.to_string());

        assert_eq!(rule("x<10:A").check(&part), expected);
        assert_eq!(rule("x>0:A").check(&part), expected);
        assert_eq!(rule("x>10:A").check(&part), None);
        assert_eq!(rule("A").check(&part), expected);
    }

    fn workflow(s: &str) -> Workflow {
        s.parse().unwrap()
    }

    fn rule(s: &str) -> Rule {
        s.parse().unwrap()
    }

    fn part(s: &str) -> Part {
        parse_part(s).unwrap()
    }
}
