use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

/// https://adventofcode.com/2021/day/14
fn main() {
    let args = get_args(&["<input file>", "<steps>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let steps = args[2].parse::<usize>().or_exit_with(1);

    let (polymer, instructions) = parse(lines).or_exit_with(1);

    let processed = process(&polymer, &instructions, steps);
    let (min, max) = analyze(&processed);
    println!(
        "The most common element occurs {} times and the least common element occurs {} times. The final answer is {}.",
        max,
        min,
        max - min
    );
}

fn parse<I>(mut lines: I) -> Result<(String, Vec<(String, char)>), ParseError>
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
    let mut instructions = vec![];
    for line in lines {
        let line = line?;
        let (pattern, insert) = match line.find(" -> ") {
            Some(p) => (line[..p].to_string(), line.chars().nth(p + 4).unwrap()),
            None => return Err(parse_error!("Invalid input line: {}", line)),
        };
        instructions.push((pattern, insert));
    }

    Ok((polymer, instructions))
}

fn process(polymer: &str, instructions: &[(String, char)], steps: usize) -> String {
    let mut polymer = polymer.to_string();

    for _ in 1..steps + 1 {
        let mut insertions: Vec<(usize, char)> = vec![];
        for (pattern, insert) in instructions {
            let mut offset = 0;
            while let Some(p) = polymer[offset..].find(pattern) {
                insertions.push((offset + p + 1, *insert));
                offset = offset + p + 1;
            }
        }

        insertions.sort_unstable_by_key(|&(i, _)| i);
        for (offset, (i, ch)) in insertions.into_iter().enumerate() {
            polymer.insert(i + offset, ch);
        }
    }

    polymer
}

fn analyze(str: &str) -> (usize, usize) {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for ch in str.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let mut min = usize::MAX;
    let mut max = usize::MIN;

    for &count in counts.values() {
        if count < min {
            min = count;
        }
        if count > max {
            max = count;
        }
    }

    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let (polymer, instructions) = get_test_data();

        let polymer = process(&polymer, &instructions, 1);
        assert_eq!(&polymer, "NCNBCHB");

        let polymer = process(&polymer, &instructions, 1);
        assert_eq!(&polymer, "NBCCNBBBCBHCB");

        let polymer = process(&polymer, &instructions, 1);
        assert_eq!(&polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        let polymer = process(&polymer, &instructions, 1);
        assert_eq!(
            &polymer,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
    }

    #[test]
    fn test_analyze() {
        assert_eq!(analyze("NNCB"), (1, 2));
        assert_eq!(analyze("NCNBCHB"), (1, 2));
        assert_eq!(analyze("NBCCNBBBCBHCB"), (1, 6));
        assert_eq!(analyze("NBBBCNCCNBBNBNBBCHBHHBCHB"), (4, 11));
        assert_eq!(
            analyze("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"),
            (5, 23)
        );
    }

    fn get_test_data() -> (String, Vec<(String, char)>) {
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
