use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;

#[derive(Debug, Clone)]
pub struct WiringDiagram {
    connections: HashMap<String, Vec<String>>,
}

impl WiringDiagram {
    pub fn find_bisection(&self, cuts: usize) -> Option<(usize, usize)> {
        if cuts == 0 {
            self.check_partitioning()
        } else {
            let mut clone = self.clone();

            for (from, values) in &self.connections {
                let it = values.iter().filter(|to| from < to);

                for to in it {
                    let i = clone.remove_connection(from, to);
                    let j = clone.remove_connection(to, from);
                    let result = clone.find_bisection(cuts - 1);
                    if result.is_some() {
                        return result;
                    }
                    clone.restore_connection(from, to, i);
                    clone.restore_connection(to, from, j);
                }
            }

            None
        }
    }

    fn remove_connection(&mut self, from: &str, to: &str) -> usize {
        let list = self.connections.get_mut(from).unwrap();
        let i = list.iter().position(|s| s == to).unwrap();
        list.remove(i);
        i
    }

    fn restore_connection(&mut self, from: &str, to: &str, i: usize) {
        let list = self.connections.get_mut(from).unwrap();
        list.insert(i, to.to_string());
    }

    fn check_partitioning(&self) -> Option<(usize, usize)> {
        let start = self
            .connections
            .keys()
            .next()
            .expect("wiring diagram is empty");

        let mut exploring: VecDeque<String> = VecDeque::new();
        exploring.push_back(start.clone());
        let mut seen: HashSet<String> = HashSet::new();

        while let Some(component) = exploring.pop_front() {
            if seen.contains(&component) {
                continue;
            }
            seen.insert(component.clone());
            for component in self.connections.get(&component).unwrap() {
                exploring.push_back(component.clone());
            }
        }

        let partition_size = seen.len();
        let nr_components = self.connections.len();

        if partition_size == nr_components {
            None
        } else {
            Some((partition_size, nr_components - partition_size))
        }
    }
}

impl FromInput for WiringDiagram {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut diagram = WiringDiagram {
            connections: HashMap::new(),
        };

        for line in input {
            let i = line.find(": ").ok_or(())?;
            let from = &line[..i].to_string();
            for to in line[i + 2..].split(' ') {
                let to = to.to_string();
                diagram
                    .connections
                    .entry(from.clone())
                    .or_default()
                    .push(to.clone());
                diagram
                    .connections
                    .entry(to)
                    .or_default()
                    .push(from.clone());
            }
        }

        Ok(diagram)
    }
}

impl fmt::Display for WiringDiagram {
    /// Print all connections in the wiring diagram in a format that is
    /// compatible with <https://csacademy.com/app/graph_editor/>.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys = self.connections.keys().collect::<Vec<_>>();
        keys.sort();

        for from in keys {
            let values = self
                .connections
                .get(from)
                .unwrap()
                .iter()
                .filter(|to| from < to);

            for to in values {
                writeln!(f, "{from} {to}")?;
            }
        }

        Ok(())
    }
}
