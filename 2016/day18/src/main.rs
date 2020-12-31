use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};

fn main() {
    let args = get_args(&["<input file>", "<nr rows part 1>", "<nr rows part 2>"], 1);
    let (first_row, bit_mask) = parse_input(&args[1]).or_exit_with(1);
    let nr_rows_p1 = args[2].parse::<usize>().or_exit_with(1);
    let nr_rows_p2 = args[3].parse::<usize>().or_exit_with(1);

    let min_bound = nr_rows_p1.min(nr_rows_p2);
    let max_bound = nr_rows_p1.max(nr_rows_p2);

    let mut row = first_row;
    let mut safe_tiles = count_safe_tiles(row, bit_mask);
    for _ in 1..min_bound {
        row = next_row(row, bit_mask);
        safe_tiles += count_safe_tiles(row, bit_mask);
    }
    println!(
        "There are {} safe tiles in the first {} rows.",
        safe_tiles, min_bound
    );
    for _ in min_bound..max_bound {
        row = next_row(row, bit_mask);
        safe_tiles += count_safe_tiles(row, bit_mask);
    }
    println!(
        "There are {} safe tiles in the first {} rows.",
        safe_tiles, max_bound
    );
}

fn next_row(row: u128, bit_mask: u128) -> u128 {
    ((row << 1) ^ (row >> 1)) & bit_mask
}

fn count_safe_tiles(row: u128, bit_mask: u128) -> u128 {
    let mut count = 0;
    let mut reg = row ^ bit_mask;
    while reg > 0 {
        count += reg % 2;
        reg >>= 1;
    }
    count
}

fn parse_input(path: &str) -> Result<(u128, u128), ParseError> {
    let file = File::open(path)?;
    let line = BufReader::new(file).lines().next().unwrap()?;
    let mut parsed = 0;
    for ch in line.chars() {
        parsed <<= 1;
        if ch == '^' {
            parsed += 1;
        }
    }
    Ok((parsed, 2u128.pow(line.len() as u32) - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const BIT_MASK_5: u128 = 0b11111;
    const BIT_MASK_10: u128 = 0b1111111111;

    #[test]
    fn test_next_row_1() {
        assert_eq!(next_row(0b00110, BIT_MASK_5), 0b01111);
        assert_eq!(next_row(0b01111, BIT_MASK_5), 0b11001);
    }

    #[test]
    fn test_next_row_2() {
        assert_eq!(next_row(0b0110101111, BIT_MASK_10), 0b1110001001);
        assert_eq!(next_row(0b1110001001, BIT_MASK_10), 0b1011010110);
        assert_eq!(next_row(0b1011010110, BIT_MASK_10), 0b0011000111);
        assert_eq!(next_row(0b0011000111, BIT_MASK_10), 0b0111101101);
        assert_eq!(next_row(0b0111101101, BIT_MASK_10), 0b1100101100);
        assert_eq!(next_row(0b1100101100, BIT_MASK_10), 0b1111001110);
        assert_eq!(next_row(0b1111001110, BIT_MASK_10), 0b1001111011);
        assert_eq!(next_row(0b1001111011, BIT_MASK_10), 0b0111001011);
        assert_eq!(next_row(0b0111001011, BIT_MASK_10), 0b1101110011);
    }

    #[test]
    fn test_count_safe_tiles() {
        assert_eq!(count_safe_tiles(0b0110101111, BIT_MASK_10), 3);
        assert_eq!(count_safe_tiles(0b1110001001, BIT_MASK_10), 5);
        assert_eq!(count_safe_tiles(0b1011010110, BIT_MASK_10), 4);
        assert_eq!(count_safe_tiles(0b0011000111, BIT_MASK_10), 5);
        assert_eq!(count_safe_tiles(0b0111101101, BIT_MASK_10), 3);
        assert_eq!(count_safe_tiles(0b1100101100, BIT_MASK_10), 5);
        assert_eq!(count_safe_tiles(0b1111001110, BIT_MASK_10), 3);
        assert_eq!(count_safe_tiles(0b1001111011, BIT_MASK_10), 3);
        assert_eq!(count_safe_tiles(0b0111001011, BIT_MASK_10), 4);
        assert_eq!(count_safe_tiles(0b1101110011, BIT_MASK_10), 3);
    }
}
