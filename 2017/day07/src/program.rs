use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub weight: usize,
    pub children: Vec<String>,
}

impl Program {
    pub fn get_weight(
        &self,
        programs: &HashMap<String, Program>,
        weights: &mut HashMap<String, usize>,
    ) -> usize {
        // *weights.entry(self.name.to_string()).or_insert_with(|| {
        //     self.weight + self.children.iter()
        //         .map(|c| programs.get(c))
        //         .map(|p| p.unwrap())
        //         .map(|p| p.get_weight(programs, weights))
        //         .sum::<usize>()
        // })
        if let Some(weight) = weights.get(&self.name) {
            *weight
        } else {
            let weight = self.weight
                + self
                    .children
                    .iter()
                    .map(|c| programs.get(c))
                    .map(|p| p.unwrap())
                    .map(|p| p.get_weight(programs, weights))
                    .sum::<usize>();

            weights.insert(self.name.to_string(), weight);

            weight
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.children.is_empty() {
            write!(f, "{} ({})", self.name, self.weight)
        } else {
            write!(
                f,
                "{} ({}) -> {}",
                self.name,
                self.weight,
                self.children.join(", ")
            )
        }
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut program = Program {
            name: String::new(),
            weight: 0,
            children: vec![],
        };

        if let Some(idx) = line.find(' ') {
            program.name = line[..idx].to_string();
        } else {
            return err_parse_error!("Could not get name: {}", line);
        }

        if let (Some(start), Some(end)) = (line.find('('), line.find(')')) {
            program.weight = line[start + 1..end].parse()?;
        } else {
            return err_parse_error!("Could not get weight: {}", line);
        }

        if let Some(idx) = line.find(" -> ") {
            for child in line[idx + 4..].split(", ") {
                program.children.push(child.to_string());
            }
        }

        Ok(program)
    }
}
