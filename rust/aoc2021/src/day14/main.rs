use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::{fmt, io};

use itertools::{Itertools, MinMaxResult};
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

type InstructionMap = HashMap<Pair, (Pair, Pair)>;

fn main() {
    let args = get_args(&["<input file>", "<steps>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let steps = args[2].parse::<usize>().or_exit_with(1);

    let (polymer, instructions) = parse(lines).or_exit_with(1);
    if polymer.len() < 2 {
        eprintln!("The initial polymer is too short to do anything with.");
        exit(1);
    }

    let counts = process(&polymer, &instructions, steps);

    match counts.values().copied().minmax() {
        MinMaxResult::MinMax(min, max) => println!(
            "The most common element occurs {} times and the least common element occurs {} times. The final answer is {}.",
            max,
            min,
            max - min
        ),
        _ => eprintln!("Unable to determine the min and max."),
    }
}

fn process(polymer: &str, instructions: &InstructionMap, steps: usize) -> HashMap<char, usize> {
    let mut iter = polymer.chars();
    let mut ch1 = iter.next().unwrap();

    let mut frequencies: HashMap<Pair, usize> = HashMap::new();
    for ch2 in iter {
        *frequencies.entry(Pair(ch1, ch2)).or_insert(0) += 1;
        ch1 = ch2;
    }

    for _ in 1..=steps {
        let mut next = HashMap::new();
        for (pair, count) in frequencies {
            let (p1, p2) = *instructions.get(&pair).unwrap();
            *next.entry(p1).or_insert(0) += count;
            *next.entry(p2).or_insert(0) += count;
        }
        frequencies = next;
    }

    let mut counts = HashMap::new();
    *counts.entry(ch1).or_insert(0) += 1;
    for (pair, count) in frequencies {
        *counts.entry(pair.0).or_insert(0) += count;
    }
    counts
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pair(char, char);

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair({}, {})", self.0, self.1)
    }
}

fn parse<I>(mut lines: I) -> Result<(String, InstructionMap), ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    // first line contains the input polymer
    let polymer = match lines.next() {
        Some(Ok(v)) => v,
        _ => return Err(parse_error!("Invalid input")),
    };

    // second line must be blank
    match lines.next() {
        Some(Ok(v)) if v.is_empty() => {}
        _ => return Err(parse_error!("Invalid input")),
    };

    // rest of the lines contain the instructions
    let mut instructions = HashMap::new();
    for line in lines {
        let chars = line?.chars().collect::<Vec<char>>();
        let from = Pair(chars[0], chars[1]);
        let to1 = Pair(chars[0], chars[6]);
        let to2 = Pair(chars[6], chars[1]);
        instructions.insert(from, (to1, to2));
    }

    Ok((polymer, instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_1() {
        let (polymer, instructions) = get_test_data();
        let counts = process(&polymer, &instructions, 1);

        assert_eq!(counts.get(&'B'), Some(&2usize));
        assert_eq!(counts.get(&'C'), Some(&2usize));
        assert_eq!(counts.get(&'H'), Some(&1usize));
        assert_eq!(counts.get(&'N'), Some(&2usize));
    }

    #[test]
    fn test_process_2() {
        let (polymer, instructions) = get_test_data();
        let counts = process(&polymer, &instructions, 2);

        assert_eq!(counts.get(&'B'), Some(&6usize));
        assert_eq!(counts.get(&'C'), Some(&4usize));
        assert_eq!(counts.get(&'H'), Some(&1usize));
        assert_eq!(counts.get(&'N'), Some(&2usize));
    }

    #[test]
    fn test_process_3() {
        let (polymer, instructions) = get_test_data();
        let counts = process(&polymer, &instructions, 3);

        assert_eq!(counts.get(&'B'), Some(&11usize));
        assert_eq!(counts.get(&'C'), Some(&5usize));
        assert_eq!(counts.get(&'H'), Some(&4usize));
        assert_eq!(counts.get(&'N'), Some(&5usize));
    }

    #[test]
    fn test_process_4() {
        let (polymer, instructions) = get_test_data();
        let counts = process(&polymer, &instructions, 4);

        assert_eq!(counts.get(&'B'), Some(&23usize));
        assert_eq!(counts.get(&'C'), Some(&10usize));
        assert_eq!(counts.get(&'H'), Some(&5usize));
        assert_eq!(counts.get(&'N'), Some(&11usize));
    }

    fn get_test_data() -> (String, InstructionMap) {
        let lines = vec![
            Ok("NNCB".to_string()),
            Ok("".to_string()),
            Ok("CH -> B".to_string()),
            Ok("HH -> N".to_string()),
            Ok("CB -> H".to_string()),
            Ok("NH -> C".to_string()),
            Ok("HB -> C".to_string()),
            Ok("HC -> B".to_string()),
            Ok("HN -> C".to_string()),
            Ok("NN -> C".to_string()),
            Ok("BH -> H".to_string()),
            Ok("NC -> B".to_string()),
            Ok("NB -> B".to_string()),
            Ok("BN -> B".to_string()),
            Ok("BB -> N".to_string()),
            Ok("BC -> B".to_string()),
            Ok("CC -> N".to_string()),
            Ok("CN -> C".to_string()),
        ];

        parse(lines.into_iter()).unwrap()
    }
}
