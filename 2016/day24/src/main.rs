use std::collections::HashSet;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::blueprints::{Blueprints, Point};

mod blueprints;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let blueprints = File::open(&args[1])
        .read_multi_lines::<Blueprints>(1)
        .next()
        .or_exit_with(1);

    match find_shortest_path(&blueprints, false) {
        Some(length) => println!(
            "The shortest path which visits all points of interest, has length {}.",
            length
        ),
        None => eprintln!("There is no path which visits all points of interest."),
    }

    match find_shortest_path(&blueprints, true) {
        Some(length) => println!(
            "The shortest path which visits all points of interest and then returns back to the start, has length {}.",
            length
        ),
        None => eprintln!("There is no path which visits all points of interest and then returns back to the start."),
    }
}

fn find_shortest_path(blueprints: &Blueprints, should_return: bool) -> Option<usize> {
    let start = blueprints.starting_point()?;
    let return_to = if should_return { Some(start) } else { None };
    flood_fill(blueprints, &[start], 0, return_to)
}

fn flood_fill(
    blueprints: &Blueprints,
    path: &[Point],
    distance: usize,
    return_to: Option<Point>,
) -> Option<usize> {
    let from = *path.last().unwrap();
    let mut exclude = HashSet::new();
    for point in path {
        exclude.insert(*point);
    }

    let mut found: Vec<usize> = Vec::new();
    for (next_point, d) in blueprints.find_closest_points_of_interest(from, &exclude) {
        let mut path = path.to_vec();
        path.push(next_point);
        if blueprints.visited_all_points_of_interest(&path) {
            return match return_to {
                Some(start) => {
                    Some(d + distance + blueprints.find_shortest_path(next_point, start)?)
                }
                None => Some(d + distance),
            };
        } else if let Some(distance) = flood_fill(blueprints, &path, d + distance, return_to) {
            found.push(distance);
        }
    }
    found.iter().min().cloned()
}

#[cfg(test)]
mod tests {
    use crate::blueprints::tests::get_test_blueprints;

    use super::*;

    #[test]
    fn test_without_return() {
        let blueprints = get_test_blueprints();
        assert_eq!(find_shortest_path(&blueprints, false), Some(14));
    }

    #[test]
    fn test_with_return() {
        let blueprints = get_test_blueprints();
        assert_eq!(find_shortest_path(&blueprints, true), Some(20));
    }
}
