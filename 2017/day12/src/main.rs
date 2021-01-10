use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

type Network<T> = HashMap<T, HashSet<T>>;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let pipes = parse_input(file).or_exit_with(1);

    println!(
        "There are {} programs in the group that contains program ID 0.",
        find_reachable(0, &pipes).len()
    );

    println!("There are {} groups.", count_groups(&pipes))
}

fn find_reachable(from: usize, pipes: &Network<usize>) -> HashSet<usize> {
    let mut exploring: VecDeque<usize> = VecDeque::new();
    exploring.push_back(from);
    let mut reachable: HashSet<usize> = HashSet::new();
    reachable.insert(from);

    while let Some(point) = exploring.pop_front() {
        if let Some(neighbours) = pipes.get(&point) {
            for neighbour in neighbours {
                if !reachable.contains(neighbour) {
                    exploring.push_back(*neighbour);
                    reachable.insert(*neighbour);
                }
            }
        }
    }

    reachable
}

fn count_groups(pipes: &Network<usize>) -> usize {
    let mut groups = 0;
    let mut remaining: HashSet<usize> = HashSet::new();
    remaining.extend(pipes.keys());

    while let Some(point) = remaining.iter().next() {
        groups += 1;
        let reachable = find_reachable(*point, pipes);
        for point in &reachable {
            remaining.remove(point);
        }
    }

    groups
}

fn parse_input<R: Read>(readable: R) -> Result<Network<usize>, ParseError> {
    let mut pipes: Network<usize> = HashMap::new();
    for line in BufReader::new(readable).lines() {
        let line = line?;
        let mut key = 0;
        for (idx, part) in line.split(" <-> ").enumerate() {
            match idx {
                0 => {
                    key = part.parse()?;
                }
                1 => {
                    let mut value = HashSet::new();
                    for v in part.split(", ") {
                        value.insert(v.parse()?);
                    }
                    pipes.insert(key, value);
                }
                _ => {
                    return Err(parse_error!("Invalid input line: {}", line));
                }
            }
        }
    }
    Ok(pipes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_reachable() {
        let test_input = vec![
            "0 <-> 2",
            "1 <-> 1",
            "2 <-> 0, 3, 4",
            "3 <-> 2, 4",
            "4 <-> 2, 3, 6",
            "5 <-> 6",
            "6 <-> 4, 5",
        ];
        let pipes = parse_input(test_input.join("\n").as_bytes()).unwrap();

        assert_eq!(find_reachable(0, &pipes).len(), 6);
    }

    #[test]
    fn test_count_groups() {
        let test_input = vec![
            "0 <-> 2",
            "1 <-> 1",
            "2 <-> 0, 3, 4",
            "3 <-> 2, 4",
            "4 <-> 2, 3, 6",
            "5 <-> 6",
            "6 <-> 4, 5",
        ];
        let pipes = parse_input(test_input.join("\n").as_bytes()).unwrap();

        assert_eq!(count_groups(&pipes), 2);
    }
}
