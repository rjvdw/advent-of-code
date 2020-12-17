extern crate helpers;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use helpers::{handle_result, ParseError};

/// https://adventofcode.com/2020/day/17
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <input file> <steps> <dimensions>", &args[0]);
        exit(1);
    }

    let steps = handle_result(args[2].parse::<usize>());
    let dimensions = handle_result(args[3].parse::<usize>());
    let input = handle_result(read(&args[1], dimensions));

    println!(
        "After {} steps, there are {} active cells.",
        steps,
        solve(&input, steps, dimensions)
    );
}

fn read(path: &str, dimensions: usize) -> Result<Vec<Vec<i32>>, ParseError> {
    let file = File::open(path)?;
    let mut active = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let mut coords = vec![x as i32, y as i32];
                coords.resize(dimensions, 0);
                active.push(coords);
            }
        }
    }

    Ok(active)
}

fn solve(input: &[Vec<i32>], steps: usize, dimensions: usize) -> usize {
    let mut active = HashSet::new();
    let (mut lower_bound, mut upper_bound) = do_prep(input, &mut active);

    for _ in 0..steps {
        let next = next_step(&active, lower_bound, upper_bound, dimensions);
        active = next.0;
        lower_bound = next.1;
        upper_bound = next.2;
    }

    active.len()
}

#[rustfmt::skip]
fn do_prep(input: &[Vec<i32>], active: &mut HashSet<Vec<i32>>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for v in input {
        active.insert(v.to_vec());
        for &i in v {
            if i < min {
                min = i;
            }
            if i > max {
                max = i;
            }
        }
    }

    (min, max)
}

fn next_step(
    active: &HashSet<Vec<i32>>,
    lower_bound: i32,
    upper_bound: i32,
    dimensions: usize,
) -> (HashSet<Vec<i32>>, i32, i32) {
    let min = lower_bound - 1;
    let max = upper_bound + 1;

    let mut next = HashSet::new();

    for coords in get_coords(min, max, dimensions) {
        if is_active(active, &coords, dimensions) {
            next.insert(coords);
        }
    }

    (next, min, max)
}

fn get_coords(min: i32, max: i32, dimensions: usize) -> Vec<Vec<i32>> {
    let mut coords = Vec::new();

    for x in min..=max {
        let mut coord = vec![x];
        coord.resize(dimensions, 0);
        coords.push(coord);
    }

    for i in 1..dimensions {
        let mut new_coords = coords.clone();
        for coord in coords {
            for v in min..=max {
                let mut new_coord = coord.clone();
                new_coord[i] = v;
                new_coords.push(new_coord);
            }
        }
        coords = new_coords;
    }

    coords
}

fn is_active(active: &HashSet<Vec<i32>>, coords: &[i32], dimensions: usize) -> bool {
    let active_neighbours = count_neighbours(active, coords, dimensions);
    if active.contains(coords) {
        active_neighbours == 2 || active_neighbours == 3
    } else {
        active_neighbours == 3
    }
}

fn count_neighbours(active: &HashSet<Vec<i32>>, coords: &[i32], dimensions: usize) -> usize {
    active
        .iter()
        .filter(|v| is_neighbour(v, coords, dimensions))
        .count()
}

fn is_neighbour(c1: &[i32], c2: &[i32], dimensions: usize) -> bool {
    if c1 == c2 {
        false
    } else {
        for i in 0..dimensions {
            if (c1[i] - c2[i]).abs() > 1 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_is_neighbour {
        use super::*;

        #[test]
        fn test() {
            // noone is their own neighbour
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 0, 0], 3), false);

            // verify +/- 1 in every dimension
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 0, -1], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 0, 1], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, -1, 0], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 1, 0], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![-1, 0, 0], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![1, 0, 0], 3), true);

            // check the diagonals
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![1, 0, -1], 3), true);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![-1, 0, 1], 3), true);

            // check if the distance is 2
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 0, 2], 3), false);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 1, 2], 3), false);
            assert_eq!(is_neighbour(&vec![0, 0, 0], &vec![0, 2, 2], 3), false);
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let input = vec![
                vec![1, 0, 0],
                vec![2, 1, 0],
                vec![0, 2, 0],
                vec![1, 2, 0],
                vec![2, 2, 0],
            ];
            assert_eq!(solve(&input, 6, 3), 112);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let input = vec![
                vec![1, 0, 0, 0],
                vec![2, 1, 0, 0],
                vec![0, 2, 0, 0],
                vec![1, 2, 0, 0],
                vec![2, 2, 0, 0],
            ];
            assert_eq!(solve(&input, 6, 4), 848);
        }
    }
}
