use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

type CaveName = (u8, u8); // we assume the cave name is always at most 2 ascii bytes long

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Cave {
    Start,
    End,
    Small(CaveName),
    Large(CaveName),
}

impl Cave {
    fn encode(s: &str) -> Result<CaveName, ParseError> {
        match s.as_bytes() {
            [a] => Ok((*a, 0)),
            [a, b] => Ok((*a, *b)),
            _ => Err(parse_error!("Invalid cave name: {}", s)),
        }
    }

    fn decode(n: CaveName) -> String {
        match n {
            (a, 0) => String::from(a as char),
            (a, b) => String::from_utf8(vec![a, b]).unwrap(),
        }
    }
}

impl FromStr for Cave {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            _ if s.to_uppercase() == s => Ok(Cave::Large(Cave::encode(s)?)),
            _ => Ok(Cave::Small(Cave::encode(s)?)),
        }
    }
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cave::Start => write!(f, "Cave::Start"),
            Cave::End => write!(f, "Cave::End"),
            Cave::Small(s) => write!(f, "Cave::Small({:02x} {:02x})", s.0, s.1),
            Cave::Large(s) => write!(f, "Cave::Large({:02x} {:02x})", s.0, s.1),
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cave::Start => write!(f, "start"),
            Cave::End => write!(f, "end"),
            Cave::Small(s) | Cave::Large(s) => write!(f, "{}", Cave::decode(*s)),
        }
    }
}

#[derive(Debug, Default)]
pub struct CaveMap {
    nodes: HashSet<Cave>,
    edges: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
    pub fn count_paths(&self, max_revisits: usize) -> usize {
        let mut count = 0;
        let mut to_explore: Vec<(Cave, HashSet<Cave>, usize)> =
            vec![(Cave::Start, HashSet::new(), max_revisits)];

        while let Some((from, seen, revisits_left)) = to_explore.pop() {
            for &to in self.edges.get(&from).unwrap() {
                if to == Cave::End {
                    count += 1;
                } else if revisits_left > 0 || !seen.contains(&to) {
                    let next_revisits_left = if seen.contains(&to) {
                        revisits_left - 1
                    } else {
                        revisits_left
                    };
                    let mut next_seen = seen.clone();
                    if matches!(to, Cave::Small(_)) {
                        next_seen.insert(to);
                    }
                    to_explore.push((to, next_seen, next_revisits_left));
                }
            }
        }

        count
    }

    fn add_edge(&mut self, from: Cave, to: Cave) {
        if to != Cave::Start && from != Cave::End {
            self.edges
                .entry(from)
                .or_insert_with(HashSet::default)
                .insert(to);
        }
    }
}

impl MultilineFromStr for CaveMap {
    type Err = ParseError;

    fn new() -> Self {
        CaveMap::default()
    }

    fn indicates_new_record(&self, _: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if let Some(p) = line.find('-') {
            let n1 = line[..p].parse::<Cave>()?;
            let n2 = line[p + 1..].parse::<Cave>()?;

            self.nodes.insert(n1);
            self.nodes.insert(n2);

            self.add_edge(n1, n2);
            self.add_edge(n2, n1);

            Ok(())
        } else {
            Err(parse_error!("Invalid line: {}", line))
        }
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_count_paths_1() {
        let map = vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"]
            .as_multiline_records::<CaveMap>()
            .unwrap();
        let map = map.first().unwrap();

        assert_eq!(map.count_paths(0), 10);
        assert_eq!(map.count_paths(1), 36);
    }

    #[test]
    fn test_count_paths_2() {
        let map = vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ]
        .as_multiline_records::<CaveMap>()
        .unwrap();
        let map = map.first().unwrap();

        assert_eq!(map.count_paths(0), 19);
        assert_eq!(map.count_paths(1), 103);
    }

    #[test]
    fn test_count_paths_3() {
        let map = vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
        .as_multiline_records::<CaveMap>()
        .unwrap();
        let map = map.first().unwrap();

        assert_eq!(map.count_paths(0), 226);
        assert_eq!(map.count_paths(1), 3509);
    }
}
