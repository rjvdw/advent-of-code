use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

use crate::node::Node;

mod node;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let nodes = parse_input(&args[1]).or_exit_with(1);

    println!(
        "There are {} viable pairs.",
        find_viable_pairs(&nodes).len()
    );

    match find_path(&nodes) {
        Some(steps) => println!(
            "It takes {} steps to move the target node to the top left.",
            steps
        ),
        None => eprintln!("It's impossible to move the target node to the top left."),
    }
}

fn find_path(nodes: &[Node]) -> Option<usize> {
    // Time for some pretty big assumptions. At any given time:
    //   1. there is exactly one node that is empty,
    //   2. this empty node is the only node to which other nodes can be moved, and
    //   3. there are no neighbouring nodes in the top row that are not a viable pair.
    // These assumptions seem to hold true for the puzzle input, and for the example that is given.
    //
    // With these assumptions, the algorithm becomes pretty simple:
    //   1. Find the least number of moves that are needed to bring the empty node to the top right.
    //   2. While the target node (which is now to the left of the top right node) is not at the top
    //      left, move the empty node one down, two left, one up, and one right. This moves the
    //      target node one position to the left.
    // For the first step of the algorithm we actually need to find this path, as there might be
    // "blockades" in the way. However, due to assumption #3, we can simply compute the number of
    // moves in the second step, which is 5 * (width - 1).

    let mut width = 0; // placeholder
    let mut start = nodes.get(0).unwrap(); // placeholder
    let mut target = start; // placeholder

    for node in nodes {
        let (x, y) = node.get_xy();
        if node.is_empty() {
            start = node;
        }
        if y == 0 && x + 1 > width {
            width = x + 1;
            target = node;
        }
    }

    find_shortest_path(nodes, start, target).map(|v| v + 5 * (width - 2))
}

/// Find the shortest path using [A*](https://en.wikipedia.org/wiki/A*_search_algorithm).
fn find_shortest_path(nodes: &[Node], start: &Node, target: &Node) -> Option<usize> {
    let mut nodes_map: HashMap<(usize, usize), Node> = HashMap::new();
    for node in nodes {
        nodes_map.insert(node.get_xy(), node.clone());
    }

    let mut open_set: HashSet<Node> = HashSet::new();
    open_set.insert(start.clone());

    let mut came_from: HashMap<Node, Node> = HashMap::new();

    let mut g_score: HashMap<Node, usize> = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score: HashMap<Node, usize> = HashMap::new();
    f_score.insert(start.clone(), start.distance(target));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&usize::MAX))
            .cloned()
            .unwrap();

        if current.eq(target) {
            let mut node = current;
            let mut distance = 0;
            while let Some(n) = came_from.get(&node) {
                node = n.clone();
                distance += 1;
            }
            return Some(distance);
        }

        open_set.remove(&current);

        for neighbour in get_neighbours(&nodes_map, start, &current) {
            let score = *g_score.get(&current).unwrap_or(&usize::MAX) + 1;
            if score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), score);
                f_score.insert(neighbour.clone(), score + neighbour.distance(target));
                open_set.insert(neighbour.clone());
            }
        }
    }

    None
}

fn get_neighbours(nodes: &HashMap<(usize, usize), Node>, start: &Node, node: &Node) -> Vec<Node> {
    let (x, y) = node.get_xy();
    let mut neighbours: Vec<(usize, usize)> = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        neighbours.push((x - 1, y))
    }
    if y > 0 {
        neighbours.push((x, y - 1))
    }

    neighbours
        .iter()
        .map(|xy| nodes.get(xy))
        .filter(|node_opt| node_opt.is_some())
        .map(|node_opt| node_opt.unwrap())
        .filter(|node1| node1.fits_on(start))
        .cloned()
        .collect()
}

fn find_viable_pairs(nodes: &[Node]) -> Vec<(Node, Node)> {
    let mut viable_pairs = Vec::new();
    for node1 in nodes {
        for node2 in nodes {
            if is_viable(node1, node2) {
                viable_pairs.push((node1.clone(), node2.clone()))
            }
        }
    }
    viable_pairs
}

fn is_viable(node1: &Node, node2: &Node) -> bool {
    node1 != node2 && !node1.is_empty() && node1.fits_on(node2)
}

fn parse_input(path: &str) -> Result<Vec<Node>, ParseError> {
    let mut nodes = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if Node::is_node(&line) {
            nodes.push(line.parse()?);
        }
    }
    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_viable() {
        let node1 = Node::new((0, 0), 10, 8, 2);
        let node2 = Node::new((1, 0), 15, 4, 11);
        assert!(is_viable(&node1, &node2));
    }

    #[test]
    fn test_not_viable_if_same_node() {
        let node1 = Node::new((0, 0), 10, 8, 2);
        let node2 = Node::new((0, 0), 15, 4, 11);
        assert!(!is_viable(&node1, &node2));
    }

    #[test]
    fn test_not_viable_if_empty() {
        let node1 = Node::new((0, 0), 10, 0, 10);
        let node2 = Node::new((1, 0), 15, 4, 11);
        assert!(!is_viable(&node1, &node2));
    }

    #[test]
    fn test_not_viable_if_too_large() {
        let node1 = Node::new((0, 0), 10, 8, 2);
        let node2 = Node::new((1, 0), 15, 12, 3);
        assert!(!is_viable(&node1, &node2));
    }

    #[test]
    fn test_find_path() {
        let nodes = vec![
            Node::new((0, 0), 10, 8, 2),
            Node::new((0, 1), 11, 6, 5),
            Node::new((0, 2), 32, 28, 4),
            Node::new((1, 0), 9, 7, 2),
            Node::new((1, 1), 8, 0, 8),
            Node::new((1, 2), 11, 7, 4),
            Node::new((2, 0), 10, 6, 4),
            Node::new((2, 1), 9, 8, 1),
            Node::new((2, 2), 9, 6, 3),
        ];
        assert_eq!(find_path(&nodes), Some(7));
    }
}
