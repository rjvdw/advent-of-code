use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut list = vec![];
            let s = &s[1..s.len() - 1];
            let mut i = 0;
            let mut depth = 0;

            for (j, ch) in s.chars().enumerate() {
                match ch {
                    '[' => {
                        depth += 1;
                    }
                    ']' => {
                        depth -= 1;
                    }
                    ',' if depth == 0 => {
                        list.push(s[i..j].parse()?);
                        i = j + 1;
                    }
                    _ => {}
                }
            }
            if !s[i..].is_empty() {
                list.push(s[i..].parse()?);
            }

            Ok(Packet::List(list))
        } else {
            Ok(Packet::Value(s.parse()?))
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::List(v) => write!(
                f,
                "[{}]",
                v.iter().fold(String::new(), |acc, v| if acc.is_empty() {
                    format!("{}", v)
                } else {
                    format!("{},{}", acc, v)
                })
            ),
            Packet::Value(v) => write!(f, "{}", v),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(v1), Packet::Value(v2)) => v1.cmp(v2),
            (Packet::List(v1), Packet::List(v2)) => v1.cmp(v2),
            (Packet::Value(v1), Packet::List(v2)) => vec![Packet::Value(*v1)].cmp(v2),
            (Packet::List(v1), Packet::Value(v2)) => v1.cmp(&vec![Packet::Value(*v2)]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!("1".parse::<Packet>().unwrap(), Packet::Value(1));
    }

    #[test]
    fn test_parse_simple_list() {
        assert_eq!(
            "[1,1,3,1,1]".parse::<Packet>().unwrap(),
            Packet::List(vec![
                Packet::Value(1),
                Packet::Value(1),
                Packet::Value(3),
                Packet::Value(1),
                Packet::Value(1),
            ])
        );
    }

    #[test]
    fn test_parse_list_with_sublist() {
        assert_eq!(
            "[1,[2,3,4]]".parse::<Packet>().unwrap(),
            Packet::List(vec![
                Packet::Value(1),
                Packet::List(vec![Packet::Value(2), Packet::Value(3), Packet::Value(4)]),
            ])
        );
    }

    #[test]
    fn test_parse_empty_list() {
        assert_eq!("[]".parse::<Packet>().unwrap(), Packet::List(vec![]));
    }

    #[test]
    fn test_parse_empty_sublist() {
        assert_eq!(
            "[[]]".parse::<Packet>().unwrap(),
            Packet::List(vec![Packet::List(vec![])])
        );
    }

    #[test]
    fn test_ordering_1() {
        let p1 = "[1,1,3,1,1]".parse::<Packet>().unwrap();
        let p2 = "[1,1,5,1,1]".parse::<Packet>().unwrap();

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_2() {
        let p1 = "[[1],[2,3,4]]".parse::<Packet>().unwrap();
        let p2 = "[[1],4]".parse::<Packet>().unwrap();

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_3() {
        let p1 = "[9]".parse::<Packet>().unwrap();
        let p2 = "[[8,7,6]]".parse::<Packet>().unwrap();

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordering_4() {
        let p1 = "[[4,4],4,4]".parse::<Packet>().unwrap();
        let p2 = "[[4,4],4,4,4]".parse::<Packet>().unwrap();

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_5() {
        let p1 = "[7,7,7,7]".parse::<Packet>().unwrap();
        let p2 = "[7,7,7]".parse::<Packet>().unwrap();

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordering_6() {
        let p1 = "[]".parse::<Packet>().unwrap();
        let p2 = "[3]".parse::<Packet>().unwrap();

        assert!(p1 < p2);
    }

    #[test]
    fn test_ordering_7() {
        let p1 = "[[[]]]".parse::<Packet>().unwrap();
        let p2 = "[[]]".parse::<Packet>().unwrap();

        assert!(p1 > p2);
    }

    #[test]
    fn test_ordering_8() {
        let p1 = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>().unwrap();
        let p2 = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>().unwrap();

        assert!(p1 > p2);
    }
}
