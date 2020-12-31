use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::math::solve_crt;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let mut discs = parse_input(&args[1]).or_exit_with(1);

    let time = find_correct_time_to_push_button(&discs);
    println!(
        "The correct time to push the button is after {} seconds.",
        time
    );

    discs.push((11, 0));
    let time = find_correct_time_to_push_button(&discs);
    println!(
        "With the extra disc, the correct time to push the button is after {} seconds.",
        time
    );
}

fn find_correct_time_to_push_button(discs: &[(u64, u64)]) -> u64 {
    let mut discs_iter = discs
        .iter()
        .enumerate()
        .map(|(idx, (n, p))| needs_to_rotate(*n, *p, (idx + 1) as u64));

    let first = discs_iter.next().unwrap();

    discs_iter
        .fold(first, |left, right| {
            (left.0 * right.0, solve_crt(left, right))
        })
        .1
}

fn needs_to_rotate(positions: u64, current_position: u64, time: u64) -> (u64, u64) {
    (
        positions,
        positions - ((current_position + time) % positions),
    )
}

fn parse_input(path: &str) -> Result<Vec<(u64, u64)>, ParseError> {
    let mut discs = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        match parse_line(&line) {
            Some(disc) => discs.push(disc?),
            None => return Err(ParseError(format!("Error parsing line: {}", line))),
        }
    }
    Ok(discs)
}

fn parse_line(line: &str) -> Option<Result<(u64, u64), ParseIntError>> {
    let idx1 = line.find(" has ")? + " has ".len();
    let idx2 = idx1 + line[idx1..].find(' ')?;
    let nr_positions = line[idx1..idx2].parse::<u64>();

    let idx1 = 1 + line.rfind(' ')?;
    let idx2 = line.len() - 1;
    let current_position = line[idx1..idx2].parse::<u64>();

    match (nr_positions, current_position) {
        (Ok(a), Ok(b)) => Some(Ok((a, b))),
        (Err(e), _) | (_, Err(e)) => Some(Err(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_correct_time_to_push_button() {
        let discs = vec![(5, 4), (2, 1)];
        assert_eq!(find_correct_time_to_push_button(&discs), 5);
    }
}
