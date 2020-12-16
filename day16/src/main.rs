extern crate helpers;

use std::collections::HashMap;
use std::env;
use std::process::exit;

use helpers::{handle_result, read_multiline_input_as_single};

use crate::puzzle_input::PuzzleInput;

mod puzzle_input;

/// https://adventofcode.com/2020/day/16
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input file>", &args[0]);
        exit(1);
    }

    let mut input = handle_result(read_multiline_input_as_single::<PuzzleInput>(&args[1]));
    let invalid_values = input.filter_out_invalid_nearby_tickets();
    println!("The sum of all invalid values is: {}", invalid_values);

    let mapping = solve(&input);
    let mut prd = 1;
    for (k, v) in mapping {
        if k.starts_with("departure") {
            prd *= input.your_ticket[v] as u64;
        }
    }
    println!("The product of all the departure information is: {}", prd);
}

fn solve(input: &PuzzleInput) -> HashMap<String, usize> {
    let mut mapping: HashMap<String, usize> = HashMap::new();

    while mapping.keys().len() != input.possible_field_values.keys().len() {
        let mut found_at_least_one = false;
        for key in input.possible_field_values.keys() {
            if !mapping.contains_key(key) {
                if let Some(value) = find_unique_solution(input, key, &mapping) {
                    found_at_least_one = true;
                    mapping.insert(key.to_string(), value);
                }
            }
        }
        if !found_at_least_one {
            break;
        }
    }

    mapping
}

fn find_unique_solution(
    input: &PuzzleInput,
    key: &str,
    exclude: &HashMap<String, usize>,
) -> Option<usize> {
    let mut solution = None;

    for i in 0..(input.your_ticket.len()) {
        if exclude.values().all(|&v| v != i) && input.matches_all_tickets(key, i) {
            match solution {
                Some(_) => {
                    // solution is not unique
                    solution = None;
                    break;
                }
                None => solution = Some(i),
            }
        }
    }

    solution
}

#[cfg(test)]
mod tests {
    use helpers::parse_multiline_input_as_single;

    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test() {
            let mut input = parse_multiline_input_as_single::<PuzzleInput>(vec![
                "class: 1-3 or 5-7",
                "row: 6-11 or 33-44",
                "seat: 13-40 or 45-50",
                "",
                "your ticket:",
                "7,1,14",
                "",
                "nearby tickets:",
                "7,3,47",
                "40,4,50",
                "55,2,20",
                "38,6,12",
            ])
            .unwrap();

            let invalid_values = input.filter_out_invalid_nearby_tickets();

            assert_eq!(invalid_values, 71);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn test() {
            let mut input = parse_multiline_input_as_single::<PuzzleInput>(vec![
                "class: 0-1 or 4-19",
                "row: 0-5 or 8-19",
                "seat: 0-13 or 16-19",
                "",
                "your ticket:",
                "11,12,13",
                "",
                "nearby tickets:",
                "3,9,18",
                "15,1,5",
                "5,14,9",
            ])
            .unwrap();

            input.filter_out_invalid_nearby_tickets();
            let mapping = solve(&input);

            assert_eq!(mapping.get("row"), Some(&0));
            assert_eq!(mapping.get("class"), Some(&1));
            assert_eq!(mapping.get("seat"), Some(&2));
        }
    }
}
