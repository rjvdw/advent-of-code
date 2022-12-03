extern crate rdcl_aoc_helpers;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::WithReadLines;
use std::collections::HashSet;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let rucksacks = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Rucksack>>();

    println!(
        "The sum of the priorities of all overlapping items is {}",
        rucksacks.iter().map(|r| r.find_overlap()).sum::<u32>()
    );

    let group_size = 3;
    match find_badges(&rucksacks, group_size) {
        Some(v) => println!("The sum of the priorities of all badges is {}", v),
        _ => eprintln!(
            "Unable to divide the elves in groups of size {}",
            group_size
        ),
    };
}

fn find_badges(rucksacks: &[Rucksack], group_size: usize) -> Option<u32> {
    let mut iter = rucksacks.iter();
    let mut sum = 0;

    while let Some(r) = iter.next() {
        let mut items = r.items();
        let mut i = group_size - 1;
        while i > 0 {
            i -= 1;
            items = items.intersection(&iter.next()?.items()).copied().collect();
        }
        if items.len() != 1 {
            panic!(
                "Unexpected number of items appearing in all rucksacks: {:?}",
                items
            );
        }
        sum += *items.iter().next()? as u32;
    }

    Some(sum)
}

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<u8>,
    compartment2: HashSet<u8>,
}

impl Rucksack {
    fn find_overlap(&self) -> u32 {
        let mut sum = 0;
        for &item in self.compartment1.intersection(&self.compartment2) {
            sum += item as u32;
        }
        sum
    }

    fn items(&self) -> HashSet<u8> {
        self.compartment1
            .union(&self.compartment2)
            .copied()
            .collect()
    }
}

impl FromStr for Rucksack {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let nr_items_per_compartment = line.len() / 2;
        let mut rucksack = Rucksack {
            compartment1: HashSet::with_capacity(nr_items_per_compartment),
            compartment2: HashSet::with_capacity(nr_items_per_compartment),
        };

        for (i, &ch) in line.as_bytes().iter().enumerate() {
            let item = if ch < b'a' {
                ch - b'A' + 27
            } else {
                ch - b'a' + 1
            };

            if i < nr_items_per_compartment {
                rucksack.compartment1.insert(item);
            } else {
                rucksack.compartment2.insert(item);
            }
        }

        Ok(rucksack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_overlap_1() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap();
        assert_eq!(rucksack.find_overlap(), 16);
    }

    #[test]
    fn test_find_overlap_2() {
        let rucksack = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
            .parse::<Rucksack>()
            .unwrap();
        assert_eq!(rucksack.find_overlap(), 38);
    }

    #[test]
    fn test_find_overlap_3() {
        let rucksack = "PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap();
        assert_eq!(rucksack.find_overlap(), 42);
    }

    #[test]
    fn test_find_overlap_4() {
        let rucksack = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
            .parse::<Rucksack>()
            .unwrap();
        assert_eq!(rucksack.find_overlap(), 22);
    }

    #[test]
    fn test_find_overlap_5() {
        let rucksack = "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap();
        assert_eq!(rucksack.find_overlap(), 20);
    }

    #[test]
    fn test_find_overlap_6() {
        let rucksack = "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksack>().unwrap();
        assert_eq!(rucksack.find_overlap(), 19);
    }

    #[test]
    fn test_find_badges_1() {
        let rucksacks = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                .parse::<Rucksack>()
                .unwrap(),
            "PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap(),
        ];
        assert_eq!(find_badges(&rucksacks, 3), Some(18));
    }

    #[test]
    fn test_find_badges_2() {
        let rucksacks = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap(),
            "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksack>().unwrap(),
        ];
        assert_eq!(find_badges(&rucksacks, 3), Some(52));
    }

    #[test]
    fn test_find_badges_3() {
        let rucksacks = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                .parse::<Rucksack>()
                .unwrap(),
            "PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap(),
            "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksack>().unwrap(),
        ];
        assert_eq!(find_badges(&rucksacks, 3), Some(70));
    }
}
