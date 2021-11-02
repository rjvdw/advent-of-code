use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>"], 1);
    let numbers = parse_input(&args[1]).or_exit_with(1);

    let (_, checksum, value) = parse_license(&numbers, 0);
    println!("The checksum is {}.", checksum);
    println!("The value of the root node is {}.", value);
}

fn parse_license(numbers: &[usize], mut offset: usize) -> (usize, usize, usize) {
    let nr_children = numbers[offset];
    let nr_metadata = numbers[offset + 1];
    offset += 2;
    let mut checksum = 0;
    let mut children = vec![0; nr_children];
    for child in &mut children {
        let (next_offset, child_checksum, value) = parse_license(numbers, offset);
        offset = next_offset;
        checksum += child_checksum;
        *child = value;
    }
    let sum = numbers.iter().skip(offset).take(nr_metadata).sum::<usize>();
    let value = if nr_children == 0 {
        sum
    } else {
        numbers
            .iter()
            .skip(offset)
            .take(nr_metadata)
            .map(|i| children.get(i - 1).unwrap_or(&0))
            .sum::<usize>()
    };
    checksum += sum;
    (offset + nr_metadata, checksum, value)
}

fn parse_input(path: &str) -> Result<Vec<usize>, ParseError> {
    let mut numbers = Vec::new();
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        for item in line?.split_whitespace() {
            numbers.push(item.parse()?);
        }
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_license() {
        let numbers = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let license = parse_license(&numbers, 0);
        assert_eq!(license, (numbers.len(), 138, 66));
    }
}
