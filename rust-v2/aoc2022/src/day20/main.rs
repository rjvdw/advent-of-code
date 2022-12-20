//! The solution for [advent of code 2022, day 20](https://adventofcode.com/2022/day/20)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 20")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The encryption key.
    #[clap(long, value_parser, default_value_t = 811_589_153)]
    encryption_key: i64,

    /// The number of rounds of encryption to apply.
    #[clap(long, value_parser, default_value_t = 10)]
    rounds: usize,
}

fn main() {
    let args: Args = Args::parse();
    let mut input: Vec<i64> = InputReader::from(args.input)
        .parse_lines(|l| l.parse::<i64>())
        .map(|nr| nr * args.encryption_key)
        .collect();

    encrypt(&mut input, args.rounds);
    let (x, y, z) = grove_coordinates(&input);
    println!(
        "The grove coordinates are: {} + {} + {} = {}",
        x,
        y,
        z,
        x + y + z
    );
}

fn encrypt(input: &mut Vec<i64>, rounds: usize) {
    // indices are where each element in the original list can currently be found
    let mut indices = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        indices.push(i);
    }

    // origins are where each element originally came from
    let mut origins = indices.clone();

    let modulus = input.len() as i64 - 1;

    for i in 0..rounds * input.len() {
        let mut idx = indices[i % input.len()];
        let mut steps = input[idx] % modulus;
        while steps != 0 {
            let next_idx = if steps > 0 {
                steps -= 1;
                (idx + 1) % input.len()
            } else {
                steps += 1;
                idx.checked_sub(1).unwrap_or(input.len() - 1)
            };

            // input and origins should stay in sync
            input.swap(idx, next_idx);
            origins.swap(idx, next_idx);

            // while the indices should be updated by swapping the originating indices
            indices.swap(origins[idx], origins[next_idx]);

            idx = next_idx;
        }
    }

    // ensure the value 0 is sorted to the start of the list
    while input[0] != 0 {
        for i in 1..input.len() {
            input.swap(i - 1, i);
        }
    }
}

fn grove_coordinates(input: &[i64]) -> (i64, i64, i64) {
    let x = input[1000 % input.len()];
    let y = input[2000 % input.len()];
    let z = input[3000 % input.len()];

    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<i64> {
        InputReader::from("./src/day20/test.txt")
            .parse_lines(|l| l.parse::<i64>())
            .collect()
    }

    #[test]
    fn test_encrypt() {
        let mut input = test_data();
        encrypt(&mut input, 1);
        assert_eq!(input, vec![0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_encrypt_large() {
        let mut input = test_data().iter().map(|v| v * 811_589_153).collect();
        encrypt(&mut input, 10);
        assert_eq!(
            input,
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153,
            ]
        );
    }

    #[test]
    fn test_grove_coordinates() {
        let input = vec![0, 3, -2, 1, 2, -3, 4];
        assert_eq!(grove_coordinates(&input), (4, -3, 2))
    }
}
