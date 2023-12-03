//! The solution for [advent of code 2023, day 3](https://adventofcode.com/2023/day/3)

use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 3")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let schematics: Vec<_> = input.read_lines().collect();

    let part_numbers = find_part_numbers(&schematics);
    println!(
        "The sum of all part numbers is {}",
        part_numbers.iter().sum::<u32>()
    );

    let gear_ratios = find_gear_ratios(&schematics);
    println!(
        "The sum of all gear ratios is {}",
        gear_ratios.iter().sum::<u32>()
    );
}

fn find_part_numbers(schematics: &[String]) -> Vec<u32> {
    let mut part_numbers = vec![];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in schematics.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' && !ch.is_ascii_digit() {
                push_part_numbers(schematics, row, col, &mut seen, &mut part_numbers);
            }
        }
    }

    part_numbers
}

fn find_gear_ratios(schematics: &[String]) -> Vec<u32> {
    let mut gear_ratios = vec![];
    let mut part_numbers = vec![];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in schematics.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '*' {
                seen.clear();
                part_numbers.clear();
                push_part_numbers(schematics, row, col, &mut seen, &mut part_numbers);
                if part_numbers.len() == 2 {
                    gear_ratios.push(part_numbers.iter().product::<u32>());
                }
            }
        }
    }

    gear_ratios
}

/// Convenience method for pushing all part numbers while stripping duplicates.
fn push_part_numbers(
    schematics: &[String],
    row: usize,
    col: usize,
    seen: &mut HashSet<(usize, usize)>,
    part_numbers: &mut Vec<u32>,
) {
    for (part_number, row, col) in find_part_numbers_around(schematics, row, col) {
        if !seen.contains(&(row, col)) {
            part_numbers.push(part_number);
            seen.insert((row, col));
        }
    }
}

/// Check the neighbouring cells for part numbers.
fn find_part_numbers_around(
    schematics: &[String],
    row: usize,
    col: usize,
) -> Vec<(u32, usize, usize)> {
    let mut part_numbers = vec![];

    if row > 0 {
        if col > 0 {
            if let Some(pn) = find_part_number_at(schematics, row - 1, col - 1) {
                part_numbers.push(pn);
            }
        }

        if let Some(pn) = find_part_number_at(schematics, row - 1, col) {
            part_numbers.push(pn);
        }

        if col + 1 < schematics[row].len() {
            if let Some(pn) = find_part_number_at(schematics, row - 1, col + 1) {
                part_numbers.push(pn);
            }
        }
    }

    if col > 0 {
        if let Some(pn) = find_part_number_at(schematics, row, col - 1) {
            part_numbers.push(pn);
        }
    }

    if col + 1 < schematics[row].len() {
        if let Some(pn) = find_part_number_at(schematics, row, col + 1) {
            part_numbers.push(pn);
        }
    }

    if row + 1 < schematics.len() {
        if col > 0 {
            if let Some(pn) = find_part_number_at(schematics, row + 1, col - 1) {
                part_numbers.push(pn);
            }
        }

        if let Some(pn) = find_part_number_at(schematics, row + 1, col) {
            part_numbers.push(pn);
        }

        if col + 1 < schematics[row].len() {
            if let Some(pn) = find_part_number_at(schematics, row + 1, col + 1) {
                part_numbers.push(pn);
            }
        }
    }

    part_numbers
}

/// Check if a part number is found at the given coordinates.
fn find_part_number_at(
    schematics: &[String],
    row: usize,
    col: usize,
) -> Option<(u32, usize, usize)> {
    if check_digit(schematics, row, col).is_none() {
        None
    } else {
        let mut start = col;
        while start > 0 && check_digit(schematics, row, start - 1).is_some() {
            start -= 1;
        }
        let mut end = col;
        let len = schematics[row].len();
        while end + 1 < len && check_digit(schematics, row, end + 1).is_some() {
            end += 1;
        }

        let part_number = schematics[row][start..=end].parse::<u32>().unwrap();
        Some((part_number, row, start))
    }
}

/// Check if there is a digit at the given coordinates.
fn check_digit(schematics: &[String], row: usize, col: usize) -> Option<u32> {
    let ch = schematics[row].as_bytes().get(col).copied()?;

    if (0x30..=0x39).contains(&ch) {
        Some((ch - 0x30) as u32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        InputReader::from("./src/day03/test.txt")
            .read_lines()
            .collect()
    }

    #[test]
    fn test_find_part_numbers() {
        assert_eq!(
            find_part_numbers(&test_data()),
            [467, 35, 633, 617, 592, 664, 755, 598]
        );
    }

    #[test]
    fn test_find_gear_ratios() {
        assert_eq!(find_gear_ratios(&test_data()), [467 * 35, 755 * 598]);
    }
}
