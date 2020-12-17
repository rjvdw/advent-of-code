extern crate helpers;

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use helpers::{handle_result, ParseError};

const MAX_DIMENSIONS: usize = 5;

type Coords = (i32, i32, i32, i32, i32);

/// https://adventofcode.com/2020/day/17
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <input file> <steps> <dimensions>", &args[0]);
        exit(1);
    }

    let input = handle_result(read(&args[1]));
    let steps = handle_result(args[2].parse::<usize>());
    let dimensions = handle_result(args[3].parse::<usize>());

    if dimensions > MAX_DIMENSIONS {
        eprintln!(
            "Currently, a maximum of {} dimensions are supported.",
            MAX_DIMENSIONS
        );
        exit(1);
    }

    println!(
        "After {} steps, there are {} active cells.",
        steps,
        solve(&input, steps, dimensions)
    );
}

fn read(path: &str) -> Result<Vec<Coords>, ParseError> {
    let file = File::open(path)?;
    let mut active = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                active.push((x as i32, y as i32, 0, 0, 0));
            }
        }
    }

    Ok(active)
}

fn solve(input: &[Coords], steps: usize, dimensions: usize) -> usize {
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
fn do_prep(input: &[Coords], active: &mut HashSet<Coords>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for &v in input {
        active.insert(v);
        if v.0 < min { min = v.0 }
        if v.1 < min { min = v.1 }
        if v.2 < min { min = v.2 }
        if v.3 < min { min = v.3 }
        if v.4 < min { min = v.4 }
        if v.0 > max { max = v.0 }
        if v.1 > max { max = v.1 }
        if v.2 > max { max = v.2 }
        if v.3 > max { max = v.3 }
        if v.4 > max { max = v.4 }
    }

    (min, max)
}

fn next_step(
    active: &HashSet<Coords>,
    lower_bound: i32,
    upper_bound: i32,
    dimensions: usize,
) -> (HashSet<Coords>, i32, i32) {
    let min = lower_bound - 1;
    let max = upper_bound + 1;

    let mut next = HashSet::new();

    for coords in get_coords(min, max, dimensions) {
        if is_active(active, &coords) {
            next.insert(coords);
        }
    }

    (next, min, max)
}

fn get_coords(min: i32, max: i32, dimensions: usize) -> Vec<Coords> {
    let mut coords = Vec::new();

    for x in min..=max {
        coords.push((x, 0, 0, 0, 0));
    }

    for i in 1..dimensions {
        let mut new_coords = coords.clone();
        for coord in coords {
            match i {
                1 => {
                    for y in min..=max {
                        new_coords.push((coord.0, y, 0, 0, 0))
                    }
                }
                2 => {
                    for z in min..=max {
                        new_coords.push((coord.0, coord.1, z, 0, 0))
                    }
                }
                3 => {
                    for w in min..=max {
                        new_coords.push((coord.0, coord.1, coord.2, w, 0))
                    }
                }
                4 => {
                    for v in min..=max {
                        new_coords.push((coord.0, coord.1, coord.2, coord.3, v))
                    }
                }
                _ => panic!("Invalid dimension"),
            }
        }
        coords = new_coords;
    }

    coords
}

fn is_active(active: &HashSet<Coords>, coords: &Coords) -> bool {
    let active_neighbours = count_neighbours(active, coords);
    if active.contains(coords) {
        active_neighbours == 2 || active_neighbours == 3
    } else {
        active_neighbours == 3
    }
}

fn count_neighbours(active: &HashSet<Coords>, coords: &Coords) -> usize {
    active.iter().filter(|v| is_neighbour(v, coords)).count()
}

fn is_neighbour(c1: &Coords, c2: &Coords) -> bool {
    if c1 == c2 {
        false
    } else {
        let d1 = (c1.0 - c2.0).abs();
        let d2 = (c1.1 - c2.1).abs();
        let d3 = (c1.2 - c2.2).abs();
        let d4 = (c1.3 - c2.3).abs();
        let d5 = (c1.4 - c2.4).abs();

        !(d1 > 1 || d2 > 1 || d3 > 1 || d4 > 1 || d5 > 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let input = vec![
                (1, 0, 0, 0, 0),
                (2, 1, 0, 0, 0),
                (0, 2, 0, 0, 0),
                (1, 2, 0, 0, 0),
                (2, 2, 0, 0, 0),
            ];
            assert_eq!(solve(&input, 6, 3), 112);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let input = vec![
                (1, 0, 0, 0, 0),
                (2, 1, 0, 0, 0),
                (0, 2, 0, 0, 0),
                (1, 2, 0, 0, 0),
                (2, 2, 0, 0, 0),
            ];
            assert_eq!(solve(&input, 6, 4), 848);
        }
    }
}
