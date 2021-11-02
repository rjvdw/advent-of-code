use std::fs::File;
use std::str::FromStr;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::parse_error;

use shared::graph::{create_graph, find_longest_path, find_shortest_path, EdgeLike};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let edges = File::open(&args[1]).read_lines::<Edge>(1);
    let graph = create_graph(edges, true);

    match find_shortest_path(&graph, false) {
        Some((path, distance)) => println!(
            "The shortest path is {:?} with a distance of {}.",
            path, distance
        ),
        None => println!("Could not find a path through all locations."),
    }

    match find_longest_path(&graph, false) {
        Some((path, distance)) => println!(
            "The longest path is {:?} with a distance of {}.",
            path, distance
        ),
        None => println!("Could not find a path through all locations."),
    }
}

struct Edge(String, String, u32);

impl EdgeLike<String, u32> for Edge {
    fn from(&self) -> String {
        self.0.to_string()
    }

    fn to(&self) -> String {
        self.1.to_string()
    }

    fn value(&self) -> u32 {
        self.2
    }
}

impl FromStr for Edge {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let (Some(idx1), Some(idx2)) = (s.find("to"), s.find('=')) {
            let node1 = s[..idx1].trim().to_string();
            let node2 = s[idx1 + 2..idx2].trim().to_string();
            let distance = s[idx2 + 1..].trim().parse::<u32>()?;

            Ok(Edge(node1, node2, distance))
        } else {
            Err(parse_error!("Invalid input: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use shared::graph::Graph;

    use super::*;

    const LONDON: &str = "London";
    const DUBLIN: &str = "Dublin";
    const BELFAST: &str = "Belfast";

    #[test]
    fn test_shortest_path() {
        let graph = get_graph();
        let (_, distance) = find_shortest_path(&graph, false).unwrap();
        assert_eq!(distance, 605);
    }

    #[test]
    fn test_longest_path() {
        let graph = get_graph();
        let (_, distance) = find_longest_path(&graph, false).unwrap();
        assert_eq!(distance, 982);
    }

    fn get_graph() -> Graph<String, u32> {
        let mut graph: Graph<String, u32> = HashMap::new();
        define_edge(&mut graph, LONDON, DUBLIN, 464);
        define_edge(&mut graph, DUBLIN, LONDON, 464);
        define_edge(&mut graph, LONDON, BELFAST, 518);
        define_edge(&mut graph, BELFAST, LONDON, 518);
        define_edge(&mut graph, DUBLIN, BELFAST, 141);
        define_edge(&mut graph, BELFAST, DUBLIN, 141);
        graph
    }

    fn define_edge(graph: &mut Graph<String, u32>, from: &str, to: &str, distance: u32) {
        graph
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), distance);
    }
}
