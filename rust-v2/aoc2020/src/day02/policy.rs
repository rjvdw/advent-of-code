use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

use crate::policy_v1::PolicyV1;
use crate::policy_v2::PolicyV2;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Policy(usize, usize, char);

impl PolicyV1 for Policy {
    fn check(&self, pw: &str) -> bool {
        let count = pw.chars().filter(|&ch| ch == self.2).count();
        self.0 <= count && count <= self.1
    }
}

impl PolicyV2 for Policy {
    fn check(&self, pw: &str) -> bool {
        let c1 = pw.chars().nth(self.0 - 1) == Some(self.2);
        let c2 = pw.chars().nth(self.1 - 1) == Some(self.2);

        (c1 || c2) && !(c1 && c2)
    }
}

impl FromStr for Policy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut policy = Policy::default();

        let s = match s.find('-') {
            Some(pos) => {
                policy.0 = s[..pos].parse()?;
                &s[pos + 1..]
            }
            None => {
                return err_parse_error!("Invalid input: {}", s);
            }
        };

        let s = match s.find(' ') {
            Some(pos) => {
                policy.1 = s[..pos].parse()?;
                &s[pos + 1..]
            }
            None => {
                return err_parse_error!("Invalid input: {}", s);
            }
        };

        policy.2 = s.parse()?;

        Ok(policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!("1-3 a".parse::<Policy>(), Ok(Policy(1, 3, 'a')));

        assert!("1-3".parse::<Policy>().is_err());
        assert!("1-3 as".parse::<Policy>().is_err());
        assert!("x-y".parse::<Policy>().is_err());
        assert!("x".parse::<Policy>().is_err());
        assert!("".parse::<Policy>().is_err());
    }

    #[test]
    fn test_check_v1() {
        let policy = Policy(1, 3, 'a');

        assert!(PolicyV1::check(&policy, "bababa"));
        assert!(PolicyV1::check(&policy, "aaa"));
        assert!(PolicyV1::check(&policy, "abbabbb"));
        assert!(PolicyV1::check(&policy, "bbbbabbbb"));
        assert!(!PolicyV1::check(&policy, "bbb"));
        assert!(!PolicyV1::check(&policy, "bbbaaaaa"));
        assert!(!PolicyV1::check(&policy, "aaaaaaa"));
    }

    #[test]
    fn test_check_v2() {
        let policy = Policy(1, 3, 'a');

        assert!(PolicyV2::check(&policy, "abbbb"));
        assert!(PolicyV2::check(&policy, "ab"));
        assert!(PolicyV2::check(&policy, "bba"));
        assert!(PolicyV2::check(&policy, "bbabbb"));
        assert!(!PolicyV2::check(&policy, "aba"));
        assert!(!PolicyV2::check(&policy, "bbb"));
        assert!(!PolicyV2::check(&policy, ""));
    }
}
