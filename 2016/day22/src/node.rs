use std::hash;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Default, Copy, Clone)]
pub struct Node {
    xy: (usize, usize),
    size: u64,
    used: u64,
    available: u64,
}

impl Node {
    #[cfg(test)]
    pub fn new(xy: (usize, usize), size: u64, used: u64, available: u64) -> Node {
        Node {
            xy,
            size,
            used,
            available,
        }
    }

    pub fn is_node(line: &str) -> bool {
        line.starts_with("/dev/grid/node-")
    }

    pub fn is_empty(&self) -> bool {
        self.used == 0
    }

    pub fn fits_on(&self, other: &Self) -> bool {
        self.used.le(&other.available)
    }

    pub fn get_xy(&self) -> (usize, usize) {
        self.xy
    }

    pub fn distance(&self, other: &Self) -> usize {
        self.xy.0.max(other.xy.0) - self.xy.0.min(other.xy.0) + self.xy.1.max(other.xy.1)
            - self.xy.1.min(other.xy.1)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.xy.eq(&other.xy)
    }
}

impl Eq for Node {}

impl hash::Hash for Node {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.xy.hash(state);
    }
}

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_prefix("/dev/grid/node-") {
            let mut node = Node::default();
            for (idx, part) in r.split_whitespace().enumerate() {
                match idx {
                    0 => node.xy = parse_xy(part)?,
                    1 => node.size = parse_size(part)?,
                    2 => node.used = parse_size(part)?,
                    3 => node.available = parse_size(part)?,
                    _ => {}
                }
            }
            Ok(node)
        } else {
            Err(ParseError(format!("Invalid input: {}", s)))
        }
    }
}

fn parse_xy(part: &str) -> Result<(usize, usize), ParseError> {
    let find_x = part.rfind('x');
    let find_dash = part.rfind('-');
    let find_y = part.rfind('y');

    if let (Some(idx_x), Some(idx_dash), Some(idx_y)) = (find_x, find_dash, find_y) {
        let x = part[idx_x + 1..idx_dash].parse()?;
        let y = part[idx_y + 1..].parse()?;

        Ok((x, y))
    } else {
        Err(ParseError(format!("Invalid input: {}", part)))
    }
}

fn parse_size(part: &str) -> Result<u64, ParseError> {
    if let Some(r) = part.strip_suffix("T") {
        Ok(r.parse()?)
    } else {
        Err(ParseError(format!("Invalid input: {}", part)))
    }
}
