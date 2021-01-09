use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::math::abs_diff;

use crate::direction::Direction;

mod direction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let directions = parse_input(&args[1]).or_exit_with(1);

    let (distance, max_distance) = walk(&directions);
    println!("The child process is {} steps from the origin. It never got more than {} steps from the origin", distance, max_distance);
}

fn walk(directions: &[Direction]) -> (i64, i64) {
    let mut position = (0, 0);
    let mut max_distance = 0;
    for direction in directions {
        position = direction.walk(position);
        let distance = hex_distance(position, (0, 0));
        if distance > max_distance {
            max_distance = distance;
        }
    }
    (hex_distance(position, (0, 0)), max_distance)
}

fn hex_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (abs_diff(a.0, b.0) + abs_diff(a.0 + a.1, b.0 + b.1) + abs_diff(a.1, b.1)) / 2
}

fn parse_input(path: &str) -> Result<Vec<Direction>, ParseError> {
    let mut directions = vec![];
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        for d in line.split(',') {
            directions.push(d.parse()?);
        }
    }
    Ok(directions)
}

#[cfg(test)]
mod tests {
    use direction::Direction::{
        North as N, NorthEast as Ne, NorthWest as Nw, South as S, SouthEast as Se, SouthWest as Sw,
    };

    use super::*;

    #[test]
    fn test_walk() {
        assert_eq!(walk(&[S, S, S]), (3, 3));
        assert_eq!(walk(&[Ne, Ne, Sw, Sw]), (0, 2));
        assert_eq!(walk(&[N, N, Se, Se]), (2, 2));
        assert_eq!(walk(&[S, Nw, S, Nw, Nw]), (3, 3));
    }
}
