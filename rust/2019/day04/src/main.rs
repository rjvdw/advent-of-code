use std::ops::RangeInclusive;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

fn main() {
    let args = get_args(&["<password range>"], 1);

    let range = parse_input(&args[1]).or_exit_with(1);
    println!(
        "[v1] A total of {} passwords meet the criteria.",
        range.filter(meets_criteria_v1).count()
    );

    let range = parse_input(&args[1]).or_exit_with(1);
    println!(
        "[v2] A total of {} passwords meet the criteria.",
        range.filter(meets_criteria_v2).count()
    );
}

fn meets_criteria_v1(password: &usize) -> bool {
    let mut has_double = false;
    let mut prev = 0;

    for &ch in password.to_string().as_bytes() {
        if ch < prev {
            return false;
        }
        if ch == prev {
            has_double = true;
        }
        prev = ch;
    }

    has_double
}

fn meets_criteria_v2(password: &usize) -> bool {
    let mut has_double = false;
    let mut repeats = 0;
    let mut prev = 0;

    for &ch in password.to_string().as_bytes() {
        if ch < prev {
            return false;
        }

        if ch == prev {
            repeats += 1;
        } else {
            if repeats == 1 {
                has_double = true;
            }
            repeats = 0;
        }

        prev = ch;
    }

    has_double || repeats == 1
}

fn parse_input(range: &str) -> Result<RangeInclusive<usize>, ParseError> {
    let mut parts = range.split('-');
    let left = parse_part(&mut parts)?;
    let right = parse_part(&mut parts)?;
    Ok(left..=right)
}

fn parse_part<'a, T: Iterator<Item = &'a str>>(iter: &mut T) -> Result<usize, ParseError> {
    if let Some(part) = iter.next() {
        Ok(part.parse()?)
    } else {
        Err(parse_error!("Invalid input."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criteria_v1() {
        assert!(meets_criteria_v1(&111111));
        assert!(!meets_criteria_v1(&223450));
        assert!(!meets_criteria_v1(&123789));
    }

    #[test]
    fn test_criteria_v2() {
        assert!(meets_criteria_v2(&112233));
        assert!(!meets_criteria_v2(&123444));
        assert!(meets_criteria_v2(&111122));
    }
}
