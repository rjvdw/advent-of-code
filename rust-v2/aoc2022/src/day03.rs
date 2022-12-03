use clap::Parser;
use rdcl_aoc_core::input::InputReader;
use std::collections::HashSet;
use std::error;
use std::path::PathBuf;
use std::str::FromStr;

/// The solution for advent of code 2022, day 3
#[derive(Parser, Debug)]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The size of the groups.
    #[clap(long, short, value_parser, default_value_t = 3)]
    group_size: usize,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let rucksacks = input
        .parse_lines(Rucksack::from_str)
        .collect::<Vec<Rucksack>>();

    println!(
        "The sum of the priorities of all overlapping items is {}",
        rucksacks.iter().map(|r| r.find_overlap()).sum::<u32>()
    );

    match find_badges(&rucksacks, args.group_size) {
        Some(v) => println!("The sum of the priorities of all badges is {}", v),
        _ => eprintln!(
            "Unable to divide the elves in groups of size {}",
            args.group_size
        ),
    };

    Ok(())
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
    type Err = ();

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

    fn test_data() -> impl Iterator<Item = Rucksack> {
        InputReader::from(PathBuf::from("./test-inputs/day03.txt")).parse_lines(Rucksack::from_str)
    }

    #[test]
    fn test_find_overlap() {
        let overlaps = test_data()
            .map(|rucksack| rucksack.find_overlap())
            .collect::<Vec<u32>>();
        assert_eq!(overlaps, vec![16, 38, 42, 22, 20, 19]);
    }

    #[test]
    fn test_find_badges_1() {
        let rucksacks = test_data().take(3).collect::<Vec<Rucksack>>();
        assert_eq!(find_badges(&rucksacks, 3), Some(18));
    }

    #[test]
    fn test_find_badges_2() {
        let rucksacks = test_data().skip(3).take(3).collect::<Vec<Rucksack>>();
        assert_eq!(find_badges(&rucksacks, 3), Some(52));
    }

    #[test]
    fn test_find_badges_3() {
        let rucksacks = test_data().collect::<Vec<Rucksack>>();
        assert_eq!(find_badges(&rucksacks, 3), Some(70));
    }
}
