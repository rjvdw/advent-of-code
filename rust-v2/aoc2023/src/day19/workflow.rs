use std::fmt;
use std::str::FromStr;

use rdcl_aoc2023::str_encoder::{decode_str, encode_str};
use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

use crate::parts::Part;

#[derive(Debug, Clone)]
pub struct Workflow {
    label: Label,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn label(&self) -> Label {
        self.label
    }

    pub fn eval(&self, part: &Part) -> Label {
        for rule in &self.rules {
            if rule.accepts(part) {
                return rule.target();
            }
        }

        unreachable!("not a single rule matched")
    }
}

impl FromStr for Workflow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = s.find('{').ok_or(())?;
        let label = s[0..idx].parse()?;
        let mut rules = vec![];

        for rule in s[idx + 1..s.len() - 1].split(',') {
            rules.push(rule.parse()?);
        }

        Ok(Workflow { label, rules })
    }
}

impl fmt::Display for Workflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rules =
            self.rules
                .iter()
                .map(|rule| format!("{rule}"))
                .fold(String::new(), |acc, v| {
                    let mut acc = acc.clone();
                    if !acc.is_empty() {
                        acc.push(',');
                    }
                    acc.push_str(&v);
                    acc
                });

        write!(f, "{}{{{rules}}}", self.label)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rule {
    If(Label, Condition, Label),
    Else(Label),
}

impl Rule {
    fn accepts(&self, part: &Part) -> bool {
        match self {
            Rule::If(source, condition, _) => {
                if let Some(rating) = part.value(*source) {
                    condition.eval(rating)
                } else {
                    false
                }
            }
            Rule::Else(_) => true,
        }
    }

    fn target(&self) -> Label {
        match self {
            Rule::If(_, _, target) => *target,
            Rule::Else(target) => *target,
        }
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx_colon) = s.find(':') {
            let idx_condition = if let Some(idx) = s.find('<') {
                idx
            } else if let Some(idx) = s.find('>') {
                idx
            } else {
                return err_parse_error!("invalid rule encountered: {}", s);
            };

            let source = s[..idx_condition].parse()?;
            let condition = s[idx_condition..idx_colon].parse()?;
            let target = s[idx_colon + 1..].parse()?;

            Ok(Rule::If(source, condition, target))
        } else {
            Ok(Rule::Else(s.parse()?))
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::If(source, condition, target) => write!(f, "{source}{condition}:{target}"),
            Rule::Else(target) => write!(f, "{target}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Label {
    Workflow(u32),
    Accepted,
    Rejected,
}

impl FromStr for Label {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Label::Accepted),
            "R" => Ok(Label::Rejected),
            _ if s.len() <= 4 => Ok(Label::Workflow(encode_str(s))),
            _ => err_parse_error!("Received a label that is more than 4 characters: {}", s),
        }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Label::Accepted => write!(f, "A"),
            Label::Rejected => write!(f, "R"),
            Label::Workflow(encoded) => {
                write!(f, "{}", decode_str(*encoded))
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Condition {
    Greater(u32),
    Less(u32),
}

impl Condition {
    fn eval(&self, value: u32) -> bool {
        match self {
            Condition::Greater(v) => value > *v,
            Condition::Less(v) => value < *v,
        }
    }
}

impl FromStr for Condition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse()?;
        match &s[0..1] {
            ">" => Ok(Condition::Greater(value)),
            "<" => Ok(Condition::Less(value)),
            _ => err_parse_error!("Invalid condition: {}", s),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Greater(v) => write!(f, ">{v}"),
            Condition::Less(v) => write!(f, "<{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod workflow_tests {
        use super::*;

        #[test]
        fn test_from_string() {
            let workflow = Workflow::from_str("ex{x>10:one,m<20:two,a>30:R,A}").unwrap();

            assert_eq!(workflow.label, label("ex"));
            assert_eq!(
                workflow.rules,
                vec![
                    rule("x>10:one"),
                    rule("m<20:two"),
                    rule("a>30:R"),
                    rule("A"),
                ]
            );
        }

        #[test]
        fn test_to_string() {
            let workflow = Workflow::from_str("ex{x>10:one,m<20:two,a>30:R,A}").unwrap();
            assert_eq!(format!("{workflow}"), "ex{x>10:one,m<20:two,a>30:R,A}");
        }
    }

    mod rule_tests {
        use super::*;

        #[test]
        fn test_from_string() {
            assert_eq!(rule("x>10:one"), rule_if("x", '>', 10, "one"));
            assert_eq!(rule("m<20:two"), rule_if("m", '<', 20, "two"));
            assert_eq!(rule("a>30:R"), rule_if("a", '>', 30, "R"));
            assert_eq!(rule("A"), rule_else("A"));
        }

        #[test]
        fn test_to_string() {
            assert_eq!(format!("{}", rule("x>10:one")), "x>10:one");
            assert_eq!(format!("{}", rule("m<20:two")), "m<20:two");
            assert_eq!(format!("{}", rule("a>30:R")), "a>30:R");
            assert_eq!(format!("{}", rule("A")), "A");
        }
    }

    mod label_tests {
        use super::*;

        #[test]
        fn test_from_string() {
            assert_eq!(label("A"), Label::Accepted);
            assert_eq!(label("R"), Label::Rejected);
            assert_eq!(
                label("a"),
                Label::Workflow(0b00000000_00000000_00000000_01100001)
            );
            assert_eq!(
                label("ab"),
                Label::Workflow(0b00000000_00000000_01100001_01100010)
            );
            assert_eq!(
                label("abc"),
                Label::Workflow(0b00000000_01100001_01100010_01100011)
            );
            assert_eq!(
                label("abcd"),
                Label::Workflow(0b01100001_01100010_01100011_01100100)
            );
            assert!(Label::from_str("abcde").is_err());
        }

        #[test]
        fn test_to_string() {
            assert_eq!(format!("{}", label("A")), "A");
            assert_eq!(format!("{}", label("R")), "R");
            assert_eq!(format!("{}", label("a")), "a");
            assert_eq!(format!("{}", label("ab")), "ab");
            assert_eq!(format!("{}", label("abc")), "abc");
            assert_eq!(format!("{}", label("abcd")), "abcd");
        }
    }

    mod condition_tests {
        use super::*;

        #[test]
        fn test_from_string() {
            assert_eq!(condition(">10"), Condition::Greater(10));
            assert_eq!(condition("<20"), Condition::Less(20));
            assert!(Condition::from_str("=30").is_err());
        }

        #[test]
        fn test_to_string() {
            assert_eq!(format!("{}", condition(">10")), ">10");
            assert_eq!(format!("{}", condition("<20")), "<20");
        }
    }

    // --- helpers ---

    fn rule(str: &str) -> Rule {
        Rule::from_str(str).unwrap()
    }

    fn rule_if(source: &str, condition: char, value: u32, target: &str) -> Rule {
        let condition = if condition == '<' {
            Condition::Less(value)
        } else {
            Condition::Greater(value)
        };

        Rule::If(source.parse().unwrap(), condition, target.parse().unwrap())
    }

    fn rule_else(target: &str) -> Rule {
        Rule::Else(target.parse().unwrap())
    }

    fn label(s: &str) -> Label {
        Label::from_str(s).unwrap()
    }

    fn condition(s: &str) -> Condition {
        Condition::from_str(s).unwrap()
    }
}
