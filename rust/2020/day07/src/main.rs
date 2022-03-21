extern crate rdcl_aoc_helpers;

use std::collections::{HashMap, HashSet};
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use bag::Bag;

mod bag;

fn main() {
    let args = get_args(&["<input file>", "<color>"], 1);

    let bags = File::open(&args[1]).read_lines(1).collect::<Vec<Bag>>();
    let color = &args[2];

    println!(
        "Number of bag colors that can contain a {} bag: {}",
        color,
        find_bags_that_may_contain_color(&bags, color).len()
    );
    println!(
        "There must be {} bags inside your single {} bag",
        count_required_nr_of_bags(&bags, color),
        color
    );
}

fn find_bags_that_may_contain_color(bags: &[Bag], color: &str) -> HashSet<String> {
    bags.iter()
        .filter(|bag| bag.contains.contains_key(color))
        .fold(HashSet::new(), |mut acc, x| {
            acc.insert(x.color.clone());
            acc.extend(find_bags_that_may_contain_color(bags, &x.color));
            acc
        })
}

fn count_required_nr_of_bags(bags: &[Bag], color: &str) -> u32 {
    let mut count = 0;
    let mut color_to_bags_map: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for bag in bags {
        color_to_bags_map.insert(bag.color.clone(), bag.contains.clone());
    }

    while !color_to_bags_map[color].is_empty() {
        let mut new_bag: HashMap<String, u32> = HashMap::new();

        for (key1, value1) in color_to_bags_map[color].iter() {
            count += value1;

            for (key2, value2) in color_to_bags_map[key1].iter() {
                *new_bag.entry(key2.clone()).or_insert(0) += value2 * value1;
            }
        }

        color_to_bags_map.insert(color.to_string(), new_bag);
    }

    count
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_find_bags_that_may_contain_color() {
        let values = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .as_records::<Bag>()
        .unwrap();

        let matching_bags = find_bags_that_may_contain_color(&values, "shiny gold");
        assert_eq!(matching_bags.len(), 4);
        assert!(matching_bags.contains("bright white"));
        assert!(matching_bags.contains("muted yellow"));
        assert!(matching_bags.contains("light red"));
        assert!(matching_bags.contains("dark orange"));
    }

    #[test]
    fn test_count_required_nr_of_bags() {
        let values = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .as_records::<Bag>()
        .unwrap();

        assert_eq!(count_required_nr_of_bags(&values, "shiny gold"), 126);
    }

    #[test]
    fn test_count_required_nr_of_bags_extra() {
        let values = vec![
            "AAA bags contain 2 BBB bags, 2 CCC bags.",
            "BBB bags contain 2 CCC bags, 2 DDD bags.",
            "CCC bags contain 2 DDD bags.",
            "DDD bags contain no other bags.",
        ]
        .as_records::<Bag>()
        .unwrap();

        assert_eq!(count_required_nr_of_bags(&values, "AAA"), 24);
    }
}
