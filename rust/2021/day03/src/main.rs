extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let mut len = 0;
    let values = File::open(&args[1])
        .read_lines::<String>(1)
        .map(|line| {
            len = line.len();
            u16::from_str_radix(&line, 2).or_exit_with(1)
        })
        .collect::<Vec<u16>>();

    let (gamma, epsilon) = compute_gamma_and_epsilon_rate(&values, len);
    println!(
        "The gamma rate is {}, and the epsilon rate is {}, so the final answer is {}.",
        gamma,
        epsilon,
        (gamma as u32) * (epsilon as u32)
    );

    match compute_life_support_rating(&values, len) {
        Some(life_support_rating) => {
            println!("The life support rating is {}.", life_support_rating)
        }
        None => eprintln!("No life support rating could be determined."),
    };
}

fn compute_gamma_and_epsilon_rate(values: &[u16], len: usize) -> (u16, u16) {
    let mut mask = 0b1;
    let mut gamma = 0;
    let mut epsilon = 0;

    for _ in 0..len {
        let (count_zero, count_one) = count_frequencies_at(values, mask);
        if count_one > count_zero {
            gamma |= mask;
        }
        if count_one < count_zero {
            epsilon |= mask;
        }
        mask *= 2;
    }

    (gamma, epsilon)
}

fn compute_life_support_rating(values: &[u16], len: usize) -> Option<u32> {
    let mut oxy = values.to_vec();
    let mut co2 = values.to_vec();

    let mut mask = 2_u16.pow((len - 1) as u32);
    for _ in 0..len {
        oxy = filter_rating(oxy, mask, |c0, c1| c1 >= c0);
        co2 = filter_rating(co2, mask, |c0, c1| c1 < c0);

        if oxy.len() == 1 && co2.len() == 1 {
            return Some((oxy[0] as u32) * (co2[0] as u32));
        }

        mask /= 2;
    }

    None
}

fn count_frequencies_at(values: &[u16], mask: u16) -> (usize, usize) {
    let mut count_zero = 0;
    let mut count_one = 0;

    for value in values {
        if value & mask == 0 {
            count_zero += 1;
        } else {
            count_one += 1;
        }
    }

    (count_zero, count_one)
}

fn filter_rating<B>(values: Vec<u16>, mask: u16, determine_bit: B) -> Vec<u16>
where
    B: FnOnce(usize, usize) -> bool,
{
    if values.len() > 1 {
        let (count_zero, count_one) = count_frequencies_at(&values, mask);
        let bit = if determine_bit(count_zero, count_one) {
            mask
        } else {
            0
        };

        values
            .iter()
            .copied()
            .filter(|value| *value & mask == bit)
            .collect()
    } else {
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_gamma_and_epsilon_rate() {
        let values = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(compute_gamma_and_epsilon_rate(&values, 5), (22, 9));
    }

    #[test]
    fn test_compute_life_support_rating() {
        let values = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        assert_eq!(compute_life_support_rating(&values, 5), Some(230));
    }
}
