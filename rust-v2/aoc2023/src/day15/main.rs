//! The solution for [advent of code 2023, day 15](https://adventofcode.com/2023/day/15)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_math::mul_mod;

const HASH_MUL: usize = 17;
const HASH_MOD: usize = 256;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 15")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).read_line();

    println!("The hash of the input is {}", verify_input(&input));
    println!(
        "The focusing power of the lenses is {}",
        arrange_lenses(&input)
    );
}

fn arrange_lenses(str: &str) -> usize {
    let mut hashmap: Vec<Vec<(Label, u8)>> =
        std::iter::repeat_with(Vec::new).take(HASH_MOD).collect();

    for s in str.split(',') {
        let instruction = s.parse::<Instruction>().unwrap();
        match instruction.instruction_type {
            InstructionType::Assign(focal_length) => {
                let entries = &mut hashmap[instruction.label.hash()];
                if let Some(i) = entries.iter().position(|(l, _)| l == &instruction.label) {
                    entries[i] = (instruction.label.clone(), focal_length);
                } else {
                    entries.push((instruction.label.clone(), focal_length));
                }
            }
            InstructionType::Remove => {
                let entries = &mut hashmap[instruction.label.hash()];
                if let Some(i) = entries.iter().position(|(l, _)| l == &instruction.label) {
                    entries.remove(i);
                }
            }
        }
    }

    let mut power = 0;

    for (i, bucket) in hashmap.iter().enumerate() {
        for (j, (_, focal_length)) in bucket.iter().enumerate() {
            let focal_length = (*focal_length) as usize;
            power += (i + 1) * (j + 1) * focal_length;
        }
    }

    power
}

fn verify_input(str: &str) -> usize {
    str.split(',').map(hash).sum()
}

fn hash(str: &str) -> usize {
    str.bytes()
        .map(|ch| ch as usize)
        .fold(0, |acc, v| mul_mod(acc + v, HASH_MUL, HASH_MOD))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Label(String);

impl Label {
    fn hash(&self) -> usize {
        hash(&self.0)
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    label: Label,
    instruction_type: InstructionType,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        if let Some(v) = instruction.find('=') {
            let value = instruction[v + 1..].parse::<u8>()?;
            Ok(Instruction {
                label: Label(instruction[0..v].to_string()),
                instruction_type: InstructionType::Assign(value),
            })
        } else if let Some(v) = instruction.find('-') {
            Ok(Instruction {
                label: Label(instruction[0..v].to_string()),
                instruction_type: InstructionType::Remove,
            })
        } else {
            err_parse_error!("Invalid instruction: {}", instruction)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum InstructionType {
    Assign(u8),
    Remove,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        InputReader::from("./src/day15/test.txt").read_line()
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_input(&test_data()), 1320);
    }

    #[test]
    fn test_arrange_lenses() {
        assert_eq!(arrange_lenses(&test_data()), 145);
    }
}
