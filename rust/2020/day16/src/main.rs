extern crate rdcl_aoc_helpers;

use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::puzzle_input::PuzzleInput;

mod puzzle_input;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let mut input = File::open(&args[1])
        .read_multi_lines::<PuzzleInput>(1)
        .next()
        .or_exit_with(1);
    let invalid_values = input.filter_out_invalid_nearby_tickets();
    println!("The sum of all invalid values is: {}", invalid_values);

    match decode_train_ticket(&input) {
        Some(mapping) => {
            let mut prd = 1;
            for (k, v) in mapping {
                if k.starts_with("departure") {
                    prd *= input.your_ticket[v] as u64;
                }
            }
            println!("The product of all the departure information is: {}", prd);
        }
        None => eprintln!("Could not decode the train ticket"),
    }
}

fn decode_train_ticket(input: &PuzzleInput) -> Option<HashMap<String, usize>> {
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
            return None;
        }
    }

    Some(mapping)
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
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_filter_out_invalid_nearby_tickets() {
        let mut input = vec![
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
        ]
        .as_multiline_records::<PuzzleInput>()
        .unwrap()
        .first()
        .cloned()
        .unwrap();

        let invalid_values = input.filter_out_invalid_nearby_tickets();

        assert_eq!(invalid_values, 71);
    }

    #[test]
    fn test_decode_train_ticket() {
        let mut input = vec![
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
        ]
        .as_multiline_records::<PuzzleInput>()
        .unwrap()
        .first()
        .cloned()
        .unwrap();

        input.filter_out_invalid_nearby_tickets();
        let mapping = decode_train_ticket(&input).unwrap();

        assert_eq!(mapping.get("row"), Some(&0));
        assert_eq!(mapping.get("class"), Some(&1));
        assert_eq!(mapping.get("seat"), Some(&2));
    }
}
