use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::math::taxi_cab_4d;

type P = (i32, i32, i32, i32);

fn main() {
    let args = get_args(&["<input file>", "<threshold>"], 1);
    let coordinates = parse_input(&args[1]).or_exit_with(1);
    let threshold = args[2].parse().or_exit_with(1);

    println!(
        "There are {} constellations.",
        find_constellations(&coordinates, threshold)
    );
}

fn find_constellations(coordinates: &[P], threshold: i32) -> usize {
    let mut constellations: Vec<HashSet<P>> = Vec::new();
    for &point in coordinates {
        let mut next_constellations = Vec::new();
        let mut new_constellation = HashSet::new();
        new_constellation.insert(point);
        for constellation in constellations {
            if constellation
                .iter()
                .any(|p| taxi_cab_4d(point, *p) <= threshold)
            {
                new_constellation.extend(constellation);
            } else {
                next_constellations.push(constellation);
            }
        }
        next_constellations.push(new_constellation);
        constellations = next_constellations;
    }
    constellations.len()
}

fn parse_input(path: &str) -> Result<Vec<P>, ParseError> {
    let mut coordinates = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let parts = line.split(',').collect::<Vec<&str>>();
        coordinates.push((
            parts[0].parse()?,
            parts[1].parse()?,
            parts[2].parse()?,
            parts[3].parse()?,
        ));
    }
    Ok(coordinates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let points = vec![
            (0, 0, 0, 0),
            (3, 0, 0, 0),
            (0, 3, 0, 0),
            (0, 0, 3, 0),
            (0, 0, 0, 3),
            (0, 0, 0, 6),
            (9, 0, 0, 0),
            (12, 0, 0, 0),
        ];

        assert_eq!(find_constellations(&points, 3), 2);
    }

    #[test]
    fn test_example_2() {
        let points = vec![
            (-1, 2, 2, 0),
            (0, 0, 2, -2),
            (0, 0, 0, -2),
            (-1, 2, 0, 0),
            (-2, -2, -2, 2),
            (3, 0, 2, -1),
            (-1, 3, 2, 2),
            (-1, 0, -1, 0),
            (0, 2, 1, -2),
            (3, 0, 0, 0),
        ];

        assert_eq!(find_constellations(&points, 3), 4);
    }

    #[test]
    fn test_example_3() {
        let points = vec![
            (1, -1, 0, 1),
            (2, 0, -1, 0),
            (3, 2, -1, 0),
            (0, 0, 3, 1),
            (0, 0, -1, -1),
            (2, 3, -2, 0),
            (-2, 2, 0, 0),
            (2, -2, 0, -1),
            (1, -1, 0, -1),
            (3, 2, 0, 2),
        ];

        assert_eq!(find_constellations(&points, 3), 3);
    }

    #[test]
    fn test_example_4() {
        let points = vec![
            (1, -1, -1, -2),
            (-2, -2, 0, 1),
            (0, 2, 1, 3),
            (-2, 3, -2, 1),
            (0, 2, 3, -2),
            (-1, -1, 1, -2),
            (0, -2, -1, 0),
            (-2, 2, 3, -1),
            (1, 2, 2, 0),
            (-1, -2, 0, -2),
        ];

        assert_eq!(find_constellations(&points, 3), 8);
    }
}
