use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

use crate::generator::Generator;

mod generator;

const BITMASK: u64 = 0b1111_1111_1111_1111;

fn main() {
    let args = get_args(&["<input file>", "<iterations v1>", "<iterations v2>"], 1);
    let (generator_a, generator_b) = parse_input(&args[1]).or_exit_with(1);
    let iterations_v1 = args[2].replace('_', "").parse::<usize>().or_exit_with(1);
    let iterations_v2 = args[3].replace('_', "").parse::<usize>().or_exit_with(1);

    println!(
        "The judge's final count after {} pairs is {}.",
        iterations_v1,
        judge(generator_a, generator_b, iterations_v1)
    );

    println!(
        "If the generators are more picky, then the judge's final count after {} pairs is {}.",
        iterations_v2,
        judge(
            generator_a.filter(|v| *v % 4 == 0),
            generator_b.filter(|v| *v % 8 == 0),
            iterations_v2,
        )
    )
}

fn judge<A, B>(generator_a: A, generator_b: B, iterations: usize) -> usize
where
    A: Iterator<Item = u64>,
    B: Iterator<Item = u64>,
{
    generator_a
        .zip(generator_b)
        .take(iterations)
        .filter(|(a, b)| (a ^ b) & BITMASK == 0)
        .count()
}

fn parse_input(path: &str) -> Result<(Generator, Generator), ParseError> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    if let (Some(Ok(la)), Some(Ok(lb))) = (lines.next(), lines.next()) {
        Ok((la.parse()?, lb.parse()?))
    } else {
        Err(parse_error!("Invalid input."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_v1() {
        let generator_a = Generator::new_a(65);
        let generator_b = Generator::new_b(8921);
        assert_eq!(judge(generator_a, generator_b, 40_000_000), 588);
    }

    #[test]
    fn test_judge_v2() {
        let generator_a = Generator::new_a(65).filter(|v| *v % 4 == 0);
        let generator_b = Generator::new_b(8921).filter(|v| *v % 8 == 0);
        assert_eq!(judge(generator_a, generator_b, 5_000_000), 309);
    }
}
