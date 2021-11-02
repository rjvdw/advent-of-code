use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::parse_error;

const STR_ROW: &str = "row";
const STR_COL: &str = "column";

fn main() {
    let args = get_args(
        &[
            "<input file>",
            "<starting code>",
            "<multiplier>",
            "<modulus>",
        ],
        1,
    );
    let (row, column) = parse_input(&args[1]).or_exit_with(1);
    let manual = Manual {
        starting_code: args[2].parse::<u64>().or_exit_with(1),
        multiplier: args[3].parse::<u64>().or_exit_with(1),
        modulus: args[4].parse::<u64>().or_exit_with(1),
    };

    println!(
        "The code at row {}, column {} is {}.",
        row,
        column,
        consult_manual(row, column, &manual)
    );
}

fn consult_manual(row: usize, column: usize, manual: &Manual) -> u64 {
    let mut code = manual.starting_code;
    for _ in 0..compute_index(row, column) {
        code *= manual.multiplier;
        code %= manual.modulus;
    }
    code
}

fn compute_index(row: usize, column: usize) -> usize {
    let mut index = 0;
    for i in 0..row - 1 {
        index += i + 1;
    }
    for i in 0..column - 1 {
        index += i + row + 1;
    }
    index
}

fn parse_input(path: &str) -> Result<(usize, usize), ParseError> {
    let file = File::open(path)?;
    let line = BufReader::new(file).lines().next().or_exit_with(1)?;

    let row = parse_slice(&line, STR_ROW, ',')?;
    let col = parse_slice(&line, STR_COL, '.')?;

    Ok((row, col))
}

fn parse_slice(line: &str, start: &str, end: char) -> Result<usize, ParseError> {
    if let Some(idx) = line.find(start) {
        let start_idx = idx + start.len() + 1;
        if let Some(idx) = line[start_idx..].find(end) {
            let end_idx = start_idx + idx;
            return Ok(line[start_idx..end_idx].parse()?);
        }
    }

    Err(parse_error!("Could not find row in error message."))
}

struct Manual {
    starting_code: u64,
    multiplier: u64,
    modulus: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual() {
        let manual = Manual {
            starting_code: 20151125,
            multiplier: 252533,
            modulus: 33554393,
        };

        assert_eq!(consult_manual(1, 1, &manual), 20151125);
        assert_eq!(consult_manual(3, 1, &manual), 16080970);
        assert_eq!(consult_manual(5, 4, &manual), 6899651);
    }
}
