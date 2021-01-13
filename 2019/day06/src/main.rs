use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let orbital_map = parse_input(file).or_exit_with(1);

    println!(
        "The number of direct and indirect orbits is {}.",
        count_orbits(&orbital_map)
    );

    match find_shortest_path(&orbital_map, "YOU", "SAN") {
        Some(distance) => println!(
            "The minimal number of orbital transfers to reach santa is {}.",
            distance
        ),
        None => eprintln!("There is no path from you to santa."),
    }
}

fn count_orbits(orbital_map: &HashMap<String, String>) -> usize {
    let mut count = 0;
    // this could be optimized, by keeping track of previously computed values...
    for orbit in orbital_map.values() {
        count += 1;
        let mut center = orbit.to_string();
        while let Some(orbit) = orbital_map.get(&center) {
            center = orbit.to_string();
            count += 1;
        }
    }
    count
}

fn find_shortest_path(
    orbital_map: &HashMap<String, String>,
    from: &str,
    to: &str,
) -> Option<usize> {
    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut distance = 0;

    let mut from = from.to_string();
    let mut to = to.to_string();
    let mut working = true;

    macro_rules! check_parent {
        ($node:expr, $working:expr) => {
            if let Some(parent) = orbital_map.get(&$node) {
                if let Some(d2) = distances.get(parent) {
                    return Some(*d2 + distance);
                }
                distances.insert(parent.to_string(), distance);
                $node = parent.to_string();
                $working = true;
            }
        };
    }

    while working {
        working = false;
        check_parent!(from, working);
        check_parent!(to, working);
        distance += 1;
    }

    None
}

fn parse_input<R: Read>(r: R) -> Result<HashMap<String, String>, ParseError> {
    let mut orbital_map = HashMap::new();
    let mut center = String::new();
    for line in BufReader::new(r).lines() {
        let line = line?;
        for (idx, part) in line.split(')').enumerate() {
            match idx {
                0 => {
                    center = part.to_string();
                }
                1 => {
                    orbital_map.insert(part.to_string(), center.to_string());
                }
                _ => {
                    return Err(parse_error!("Invalid input line: {}", line));
                }
            }
        }
    }
    Ok(orbital_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_orbits() {
        let test_input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let orbital_map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(count_orbits(&orbital_map), 42);
    }

    #[test]
    fn test_find_shortest_path() {
        let test_input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        let orbital_map = parse_input(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(find_shortest_path(&orbital_map, "YOU", "SAN"), Some(4));
    }
}
