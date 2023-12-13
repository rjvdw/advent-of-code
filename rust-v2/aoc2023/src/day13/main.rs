//! The solution for [advent of code 2023, day 13](https://adventofcode.com/2023/day/13)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::pattern::Pattern;

mod pattern;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 13")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let patterns = InputReader::from(args.input).parse_vec::<Pattern>();

    let mut summary = 0;
    for pattern in &patterns {
        match pattern.find_reflection().summarize() {
            Some(v) => {
                summary += v;
            }
            None => {
                eprintln!("No reflection found in pattern:");
                eprintln!("{pattern}");
                eprintln!();
            }
        }
    }
    println!("After summarizing the notes, you get {summary}");

    let mut summary = 0;
    for pattern in patterns {
        match pattern.find_smudge().summarize() {
            Some(v) => {
                summary += v;
            }
            None => {
                eprintln!("No reflection found in pattern:");
                eprintln!("{pattern}");
                eprintln!();
            }
        }
    }
    println!("Having found the smudges, after summarizing the notes, you get {summary}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::pattern::Reflection;

    use super::*;

    fn test_data() -> Vec<Pattern> {
        InputReader::from("./src/day13/test.txt").parse_vec()
    }

    #[test]
    fn test_find_reflections() {
        let patterns = test_data();

        let reflections = patterns
            .iter()
            .map(|p| p.find_reflection())
            .collect::<Vec<_>>();
        assert_eq!(reflections, vec![Reflection::Column(5), Reflection::Row(4)]);

        let summaries = reflections
            .iter()
            .map(|r| r.summarize())
            .collect::<Vec<_>>();
        assert_eq!(summaries, vec![Some(5), Some(400)]);
    }

    #[test]
    fn test_find_smudges() {
        let patterns = test_data();

        let reflections = patterns.iter().map(|p| p.find_smudge()).collect::<Vec<_>>();
        assert_eq!(reflections, vec![Reflection::Row(3), Reflection::Row(1)]);

        let summaries = reflections
            .iter()
            .map(|r| r.summarize())
            .collect::<Vec<_>>();
        assert_eq!(summaries, vec![Some(300), Some(100)]);
    }
}
