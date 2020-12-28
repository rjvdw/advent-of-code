use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::{MappedLines, WithReadLines};

type Graph = HashMap<String, HashMap<String, u32>>;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let edges = File::open(&args[1]).read_lines::<Edge>(1);
    let graph = create_graph(edges);

    match find_shortest_path(&graph) {
        Some((path, distance)) => println!(
            "The shortest path is {:?} with a distance of {}.",
            path, distance
        ),
        None => println!("Could not find a path through all locations."),
    }

    match find_longest_path(&graph) {
        Some((path, distance)) => println!(
            "The longest path is {:?} with a distance of {}.",
            path, distance
        ),
        None => println!("Could not find a path through all locations."),
    }
}

fn find_shortest_path(graph: &Graph) -> Option<(Vec<String>, u32)> {
    find_path(graph, &MinOrMax::Min)
}

fn find_longest_path(graph: &Graph) -> Option<(Vec<String>, u32)> {
    find_path(graph, &MinOrMax::Max)
}

fn find_path(graph: &Graph, distance_req: &MinOrMax) -> Option<(Vec<String>, u32)> {
    let iter = graph
        .keys()
        .map(|starting_node| traverse_graph(graph, &[starting_node.to_string()], 0, distance_req))
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap());

    match distance_req {
        MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
        MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
    }
}

fn traverse_graph(
    graph: &Graph,
    traveled: &[String],
    distance_so_far: u32,
    distance_req: &MinOrMax,
) -> Option<(Vec<String>, u32)> {
    let iter = graph
        .get(traveled.iter().last()?)?
        .iter()
        .filter(|(node, _)| !traveled.contains(node))
        .map(|(node, distance)| {
            let mut new_traveled = traveled.to_vec();
            new_traveled.push(node.to_string());
            (new_traveled, distance_so_far + distance)
        });

    if traveled.len() + 1 == graph.len() {
        match distance_req {
            MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
            MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
        }
    } else {
        let iter = iter
            .map(|(traveled, distance)| traverse_graph(graph, &traveled, distance, distance_req))
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap());

        match distance_req {
            MinOrMax::Min => iter.min_by_key(|(_, distance)| *distance),
            MinOrMax::Max => iter.max_by_key(|(_, distance)| *distance),
        }
    }
}

fn create_graph(edges: MappedLines<Edge, File>) -> Graph {
    let mut graph = HashMap::new();
    for Edge(from, to, distance) in edges {
        graph
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), distance);

        graph
            .entry(to.to_string())
            .or_insert_with(HashMap::new)
            .insert(from.to_string(), distance);
    }
    graph
}

enum MinOrMax {
    Min,
    Max,
}

struct Edge(String, String, u32);

impl FromStr for Edge {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let (Some(idx1), Some(idx2)) = (s.find("to"), s.find('=')) {
            let node1 = s[..idx1].trim().to_string();
            let node2 = s[idx1 + 2..idx2].trim().to_string();
            let distance = s[idx2 + 1..].trim().parse::<u32>()?;

            Ok(Edge(node1, node2, distance))
        } else {
            Err(ParseError(format!("Invalid input: {}", s)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LONDON: &str = "London";
    const DUBLIN: &str = "Dublin";
    const BELFAST: &str = "Belfast";

    #[test]
    fn test_shortest_path() {
        let graph = get_graph();
        let (_, distance) = find_shortest_path(&graph).unwrap();
        assert_eq!(distance, 605);
    }

    #[test]
    fn test_longest_path() {
        let graph = get_graph();
        let (_, distance) = find_longest_path(&graph).unwrap();
        assert_eq!(distance, 982);
    }

    fn get_graph() -> Graph {
        let mut graph: Graph = HashMap::new();
        define_edge(&mut graph, LONDON, DUBLIN, 464);
        define_edge(&mut graph, DUBLIN, LONDON, 464);
        define_edge(&mut graph, LONDON, BELFAST, 518);
        define_edge(&mut graph, BELFAST, LONDON, 518);
        define_edge(&mut graph, DUBLIN, BELFAST, 141);
        define_edge(&mut graph, BELFAST, DUBLIN, 141);
        graph
    }

    fn define_edge(graph: &mut Graph, from: &str, to: &str, distance: u32) {
        graph
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), distance);
    }
}
