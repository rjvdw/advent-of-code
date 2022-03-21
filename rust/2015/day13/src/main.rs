use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::str::FromStr;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::parse_error;

use shared::graph::{create_graph, find_longest_path, EdgeLike, Graph};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let scores = File::open(&args[1]).read_lines::<Edge>(1);
    let mut graph = create_graph(scores, false);

    fix_distances(&mut graph);

    match find_longest_path(&graph, true) {
        Some((path, score)) => println!(
            "The optimal seating arrangement is {:?}, and has a score of {}",
            path, score
        ),
        None => println!("Could not seat everyone at the table."),
    }

    add_yourself_to_graph(&mut graph);

    match find_longest_path(&graph, true) {
        Some((path, score)) => println!(
            "After including yourself, the optimal seating arrangement is {:?}, and has a score of {}",
            path, score
        ),
        None => println!("Could not seat everyone at the table after including yourself."),
    }
}

fn fix_distances(graph: &mut Graph<String, i32>) {
    let nodes: Vec<String> = graph.keys().cloned().collect();

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let node1 = &nodes[i];
            let node2 = &nodes[j];

            let score1 = *graph.get(node1).unwrap().get(node2).unwrap();
            let score2 = *graph.get(node2).unwrap().get(node1).unwrap();

            let new_score = score1 + score2;

            graph.entry(node1.to_string()).and_modify(|sg| {
                sg.insert(node2.to_string(), new_score);
            });

            graph.entry(node2.to_string()).and_modify(|sg| {
                sg.insert(node1.to_string(), new_score);
            });
        }
    }
}

fn add_yourself_to_graph(graph: &mut Graph<String, i32>) {
    let you = "You".to_string();
    let mut you_graph: HashMap<String, i32> = HashMap::new();
    for (key, value) in graph.iter_mut() {
        you_graph.insert(key.to_string(), 0);
        value.insert(you.to_string(), 0);
    }
    graph.insert(you, you_graph);
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Edge(String, String, i32);

impl EdgeLike<String, i32> for Edge {
    fn from(&self) -> String {
        self.0.to_string()
    }

    fn to(&self) -> String {
        self.1.to_string()
    }

    fn value(&self) -> i32 {
        self.2
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} would {} {} happiness units by sitting next to {}.",
            self.0,
            if self.2 < 0 { "lose" } else { "gain" },
            self.2.abs(),
            self.1
        )
    }
}

impl FromStr for Edge {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let person1 = if let Some(idx) = s.find(' ') {
            s[..idx].to_string()
        } else {
            return Err(parse_error!("Invalid input: {}", s));
        };

        let person2 = if let Some(idx) = s.rfind(' ') {
            s[idx + 1..s.len() - 1].to_string()
        } else {
            return Err(parse_error!("Invalid input: {}", s));
        };

        let (sign, score_idx) = if let Some(idx) = s.find("gain") {
            (1, idx + 4)
        } else if let Some(idx) = s.find("lose") {
            (-1, idx + 4)
        } else {
            return Err(parse_error!("Invalid input: {}", s));
        };

        let score = if let Some(idx) = s[score_idx..].find("happiness units") {
            sign * s[score_idx..score_idx + idx].trim().parse::<i32>()?
        } else {
            return Err(parse_error!("Invalid input: {}", s));
        };

        Ok(Edge(person1, person2, score))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use shared::graph::Graph;

    use super::*;

    #[test]
    fn test() {
        let mut graph = get_graph();
        fix_distances(&mut graph);
        let (seating, distance) = find_longest_path(&graph, true).unwrap();
        println!("{:?}", seating);
        assert_eq!(distance, 330);
    }

    fn get_graph() -> Graph<String, i32> {
        let mut graph: Graph<String, i32> = HashMap::new();
        define_edge(&mut graph, "Alice", "Bob", 54);
        define_edge(&mut graph, "Alice", "Carol", -79);
        define_edge(&mut graph, "Alice", "David", -2);
        define_edge(&mut graph, "Bob", "Alice", 83);
        define_edge(&mut graph, "Bob", "Carol", -7);
        define_edge(&mut graph, "Bob", "David", -63);
        define_edge(&mut graph, "Carol", "Alice", -62);
        define_edge(&mut graph, "Carol", "Bob", 60);
        define_edge(&mut graph, "Carol", "David", 55);
        define_edge(&mut graph, "David", "Alice", 46);
        define_edge(&mut graph, "David", "Bob", -7);
        define_edge(&mut graph, "David", "Carol", 41);
        graph
    }

    fn define_edge(graph: &mut Graph<String, i32>, from: &str, to: &str, distance: i32) {
        graph
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), distance);
    }
}
