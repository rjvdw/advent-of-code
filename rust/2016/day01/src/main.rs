use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = parse_input(&args[1]).or_exit_with(1);
    let (distance, hq_distance) = walk(&instructions);

    println!("The total distance traveled is {}.", distance);
    match hq_distance {
        Some(distance) => println!("The distance to the HQ is {}.", distance),
        None => println!("There is no HQ."),
    }
}

fn walk(instructions: &[Instruction]) -> (i32, Option<i32>) {
    let mut heading = 0u8;
    let mut pos: (i32, i32) = (0, 0);
    let mut hq: Option<i32> = None;
    let mut seen = HashSet::new();
    for instruction in instructions {
        let distance = match instruction {
            Instruction::Right(distance) => {
                heading = (heading + 1) % 4;
                *distance
            }
            Instruction::Left(distance) => {
                heading = (heading + 3) % 4;
                *distance
            }
        };

        for _ in 0..(usize::try_from(distance).unwrap()) {
            match heading {
                0 => pos.1 += 1, // north
                1 => pos.0 += 1, // east
                2 => pos.1 -= 1, // south
                3 => pos.0 -= 1, // west
                _ => unreachable!(),
            }

            if hq.is_none() {
                if seen.contains(&pos) {
                    hq = Some(pos.0.abs() + pos.1.abs());
                } else {
                    seen.insert(pos);
                }
            }
        }
    }

    (pos.0.abs() + pos.1.abs(), hq)
}

fn parse_input(path: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut parsed = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        for instruction in line?.split(", ") {
            if let Some(distance) = instruction.strip_prefix('R') {
                parsed.push(Instruction::Right(distance.parse()?));
            } else if let Some(distance) = instruction.strip_prefix('L') {
                parsed.push(Instruction::Left(distance.parse()?));
            } else {
                return Err(parse_error!("Invalid instruction: {}", instruction));
            }
        }
    }
    Ok(parsed)
}

enum Instruction {
    Right(i32),
    Left(i32),
}

#[cfg(test)]
mod tests {
    use crate::Instruction::{Left as L, Right as R};

    use super::*;

    #[test]
    fn test_walk_1() {
        let instructions = vec![R(2), L(3)];
        assert_eq!(walk(&instructions), (5, None));
    }

    #[test]
    fn test_walk_2() {
        let instructions = vec![R(2), R(2), R(2)];
        assert_eq!(walk(&instructions), (2, None));
    }

    #[test]
    fn test_walk_3() {
        let instructions = vec![R(5), L(5), R(5), R(3)];
        assert_eq!(walk(&instructions), (12, None));
    }

    #[test]
    fn test_walk_4() {
        let instructions = vec![R(8), R(4), R(4), R(8)];
        assert_eq!(walk(&instructions), (8, Some(4)));
    }
}
