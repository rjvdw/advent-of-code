use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let spreadsheet = parse_input(&args[1]).or_exit_with(1);

    println!(
        "The checksum of the spreadsheet is {}.",
        checksum(&spreadsheet)
    );
    println!(
        "The sum of the divisions is {}.",
        find_evenly_divisible(&spreadsheet).iter().sum::<i32>()
    )
}

fn checksum(spreadsheet: &[Vec<i32>]) -> i32 {
    spreadsheet
        .iter()
        .map(|row| {
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            for &nr in row {
                if nr < min {
                    min = nr;
                }
                if nr > max {
                    max = nr;
                }
            }
            max - min
        })
        .sum()
}

fn find_evenly_divisible(spreadsheet: &[Vec<i32>]) -> Vec<i32> {
    let mut divisions = Vec::new();
    for row in spreadsheet {
        'inner: for (i, &nr1) in row.iter().enumerate() {
            for (_, &nr2) in row.iter().enumerate().filter(|(j, _)| i != *j) {
                if nr2 != 0 && nr1 > nr2 && nr1 % nr2 == 0 {
                    divisions.push(nr1 / nr2);
                    break 'inner;
                }
            }
        }
    }
    divisions
}

fn parse_input(path: &str) -> Result<Vec<Vec<i32>>, ParseError> {
    let mut rows = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let mut row = Vec::new();
        for number in line.split_whitespace() {
            row.push(number.parse()?);
        }
        rows.push(row);
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let spreadsheet = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(checksum(&spreadsheet), 18);
    }

    #[test]
    fn test_find_evenly_divisible() {
        let spreadsheet = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(find_evenly_divisible(&spreadsheet), vec![4, 3, 2]);
    }
}
