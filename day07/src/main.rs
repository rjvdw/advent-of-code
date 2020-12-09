extern crate helpers;

use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

use helpers::{handle_result, read_input};
use input_record::InputRecord;

mod input_record;

/// https://adventofcode.com/2020/day/7
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input file> <color>", &args[0]);
        exit(1);
    }

    let path = &args[1];
    let values: Vec<InputRecord> = handle_result(read_input(path));

    println!(
        "Solution to part 1: {}",
        solve_part_1(&values, &args[2]).len()
    );
    println!("Solution to part 2: {}", solve_part_2(&values, &args[2]));
}

fn solve_part_1(values: &[InputRecord], needle: &str) -> HashSet<String> {
    values
        .iter()
        .filter(|v| v.contains.contains_key(needle))
        .fold(HashSet::new(), |mut acc, x| {
            acc.insert(x.color.clone());
            acc.extend(solve_part_1(values, &x.color));
            acc
        })
}

fn solve_part_2(values: &[InputRecord], needle: &str) -> u32 {
    let mut count = 0;
    let mut values_map: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for record in values {
        values_map.insert(record.color.clone(), record.contains.clone());
    }

    while !values_map[needle].is_empty() {
        let mut new_value: HashMap<String, u32> = HashMap::new();

        for (key1, value1) in values_map[needle].iter() {
            count += value1;

            for (key2, value2) in values_map[key1].iter() {
                *new_value.entry(key2.clone()).or_insert(0) += value2 * value1;
            }
        }

        values_map.insert(needle.to_string(), new_value);
    }

    count
}

#[cfg(test)]
mod tests {
    use helpers::parse_input;

    use super::*;

    #[test]
    fn test_part_1() {
        let values = parse_input::<InputRecord>(vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ])
        .unwrap();

        let matching_bags = solve_part_1(&values, &"shiny gold".to_string());
        assert_eq!(matching_bags.len(), 4);
        assert!(matching_bags.contains("bright white"));
        assert!(matching_bags.contains("muted yellow"));
        assert!(matching_bags.contains("light red"));
        assert!(matching_bags.contains("dark orange"));
    }

    #[test]
    fn test_part_2() {
        let values = parse_input::<InputRecord>(vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ])
        .unwrap();

        assert_eq!(solve_part_2(&values, &"shiny gold".to_string()), 126);
    }

    #[test]
    fn test_part_2_extra() {
        let values = parse_input::<InputRecord>(vec![
            "AAA bags contain 2 BBB bags, 2 CCC bags.",
            "BBB bags contain 2 CCC bags, 2 DDD bags.",
            "CCC bags contain 2 DDD bags.",
            "DDD bags contain no other bags.",
        ])
        .unwrap();

        assert_eq!(solve_part_2(&values, &"AAA".to_string()), 24);
    }
}
