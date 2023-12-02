use std::collections::HashMap;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Clone)]
pub struct Grab(Vec<(usize, String)>);

impl FromStr for Grab {
    type Err = ParseError;

    fn from_str(grab: &str) -> Result<Self, Self::Err> {
        let mut set = vec![];

        for cubes in grab.split(", ") {
            let pos = match cubes.find(' ') {
                Some(pos) => pos,
                None => return err_parse_error!("Invalid input: {}", grab),
            };

            let count = cubes[0..pos].parse()?;
            let color = cubes[pos + 1..].to_string();
            set.push((count, color));
        }

        Ok(Grab(set))
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    counts: HashMap<String, usize>,
}

impl Game {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn test(&self, cubes: &Grab) -> bool {
        cubes.0.iter().all(|(count, color)| {
            self.counts.contains_key(color) && self.counts.get(color).unwrap() <= count
        })
    }

    pub fn power(&self) -> usize {
        self.counts.values().product()
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = match line.strip_prefix("Game ") {
            Some(rest) => rest,
            None => return err_parse_error!("Invalid input: {}", line),
        };

        let pos = match line.find(':') {
            Some(pos) => pos,
            None => return err_parse_error!("Invalid input: {}", line),
        };

        let id = line[0..pos].parse()?;
        let mut counts: HashMap<String, usize> = HashMap::new();

        for grab in line[pos + 2..].split("; ") {
            let cubes = grab.parse::<Grab>()?;
            for (count, color) in &cubes.0 {
                if let Some(c) = counts.get(color) {
                    if count > c {
                        counts.insert(color.to_string(), *count);
                    }
                } else {
                    counts.insert(color.to_string(), *count);
                }
            }
        }

        Ok(Game { id, counts })
    }
}
