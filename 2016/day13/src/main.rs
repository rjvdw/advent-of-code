use std::collections::{HashMap, HashSet};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(
        &["<number>", "<start x,y>", "<target x,y>", "<max steps>"],
        1,
    );
    let maze = Maze(args[1].parse().or_exit_with(1));
    let start = parse_xy(&args[2]).or_exit_with(1);
    let target = parse_xy(&args[3]).or_exit_with(1);
    let max_steps = args[4].parse::<u64>().or_exit_with(1);

    if maze.is_wall(target) {
        eprintln!(
            "The target ({}, {}) is a wall and can never be reached.",
            target.0, target.1
        );
    } else {
        match find_shortest_path(&maze, start, target) {
            Some(steps) => println!(
                "It takes {} steps to reach ({}, {}) from ({}, {}).",
                steps, target.0, target.1, start.0, start.1
            ),
            None => {
                eprintln!(
                    "It is impossible to reach ({}, {}) from ({}, {}).",
                    target.0, target.1, start.0, start.1
                )
            }
        }
    }

    println!(
        "In {} steps, we can reach {} locations, starting from ({}, {}).",
        max_steps,
        explore_maze(&maze, start, max_steps),
        start.0,
        start.1
    );
}

/// Find the shortest path using [A*](https://en.wikipedia.org/wiki/A*_search_algorithm).
fn find_shortest_path(maze: &Maze, start: (u64, u64), target: (u64, u64)) -> Option<u64> {
    let mut open_set: HashSet<(u64, u64)> = HashSet::new();
    open_set.insert(start);

    let mut came_from: HashMap<(u64, u64), (u64, u64)> = HashMap::new();

    let mut g_score: HashMap<(u64, u64), u64> = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score: HashMap<(u64, u64), u64> = HashMap::new();
    f_score.insert(start, distance(start, target));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by_key(|node| f_score.get(node).unwrap_or(&u64::MAX))
            .cloned()
            .unwrap();

        if current == target {
            return Some(reconstruct_path(&came_from, current));
        }

        open_set.remove(&current);

        for neighbour in maze.neighbours(current) {
            let score = *g_score.get(&current).unwrap_or(&u64::MAX) + 1;
            if score < *g_score.get(&neighbour).unwrap_or(&u64::MAX) {
                came_from.insert(neighbour, current);
                g_score.insert(neighbour, score);
                f_score.insert(neighbour, score + distance(neighbour, target));
                open_set.insert(neighbour);
            }
        }
    }

    None
}

/// Explore the maze. How many locations can be reached in at most `max_steps` steps?
fn explore_maze(maze: &Maze, start: (u64, u64), mut max_steps: u64) -> usize {
    let mut explored: HashSet<(u64, u64)> = HashSet::new();
    explored.insert(start);

    let mut to_explore: HashSet<(u64, u64)> = HashSet::new();
    to_explore.insert(start);

    while max_steps > 0 {
        max_steps -= 1;
        let mut next_to_explore: HashSet<(u64, u64)> = HashSet::new();
        for node in to_explore {
            for neighbour in maze.neighbours(node) {
                if !explored.contains(&neighbour) {
                    explored.insert(neighbour);
                    next_to_explore.insert(neighbour);
                }
            }
        }
        to_explore = next_to_explore;
    }

    explored.len()
}

fn reconstruct_path(came_from: &HashMap<(u64, u64), (u64, u64)>, mut node: (u64, u64)) -> u64 {
    let mut distance = 0;
    while let Some(n) = came_from.get(&node) {
        node = *n;
        distance += 1;
    }
    distance
}

fn distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    (a.0.max(b.0) - a.0.min(b.0)) + (a.1.max(b.1) - a.1.min(b.1))
}

fn parse_xy(xy: &str) -> Result<(u64, u64), ParseError> {
    let mut parsed = (0, 0);
    for (idx, part) in xy.split(',').enumerate() {
        match idx {
            0 => parsed.0 = part.trim().parse()?,
            1 => parsed.1 = part.trim().parse()?,
            _ => return Err(ParseError(format!("Invalid input: {}", xy))),
        }
    }
    Ok(parsed)
}

struct Maze(u64);

impl Maze {
    fn is_wall(&self, (x, y): (u64, u64)) -> bool {
        let mut code = x * x + 3 * x + 2 * x * y + y + y * y + self.0;
        let mut is_wall = false;
        while code > 0 {
            if code % 2 == 1 {
                is_wall = !is_wall;
            }
            code /= 2;
        }
        is_wall
    }

    fn neighbours(&self, (x, y): (u64, u64)) -> Vec<(u64, u64)> {
        let mut n = Vec::new();
        if x > 0 {
            n.push((x - 1, y));
        }
        if y > 0 {
            n.push((x, y - 1));
        }
        n.push((x + 1, y));
        n.push((x, y + 1));
        n.iter().filter(|&c| !self.is_wall(*c)).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wall() {
        let maze = Maze(10);
        assert!(!maze.is_wall((0, 0)));
        assert!(maze.is_wall((1, 0)));
        assert!(!maze.is_wall((2, 0)));
        assert!(maze.is_wall((3, 0)));
        assert!(maze.is_wall((4, 0)));
        assert!(maze.is_wall((5, 0)));
        assert!(maze.is_wall((6, 0)));
        assert!(!maze.is_wall((7, 0)));
        assert!(maze.is_wall((8, 0)));
        assert!(maze.is_wall((9, 0)));

        let mut count_walls_in_example = 0;
        for x in 0..10 {
            for y in 0..7 {
                if maze.is_wall((x, y)) {
                    count_walls_in_example += 1;
                }
            }
        }
        assert_eq!(count_walls_in_example, 33);
    }

    #[test]
    fn test_find_shortest_path() {
        let maze = Maze(10);
        assert_eq!(find_shortest_path(&maze, (1, 1), (7, 4)), Some(11));
    }
}
