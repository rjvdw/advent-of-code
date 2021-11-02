use std::collections::HashSet;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;
use rdcl_aoc_helpers::search::Navigable;

use crate::maze::Maze;

mod maze;

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
        match maze.find_shortest_path(&start, &target) {
            Some(path) => println!(
                "It takes {} steps to reach ({}, {}) from ({}, {}).",
                path.len() - 1,
                target.0,
                target.1,
                start.0,
                start.1
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
            for (_, neighbour) in maze.get_neighbours(&node) {
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

fn parse_xy(xy: &str) -> Result<(u64, u64), ParseError> {
    let mut parsed = (0, 0);
    for (idx, part) in xy.split(',').enumerate() {
        match idx {
            0 => parsed.0 = part.trim().parse()?,
            1 => parsed.1 = part.trim().parse()?,
            _ => return Err(parse_error!("Invalid input: {}", xy)),
        }
    }
    Ok(parsed)
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
        let path = Maze(10).find_shortest_path(&(1, 1), &(7, 4)).unwrap();
        assert_eq!(path.len() - 1, 11);
    }
}
