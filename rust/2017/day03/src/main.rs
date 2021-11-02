use std::collections::HashMap;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::math::taxi_cab_2d;

use crate::spiral::Spiral;

mod spiral;

fn main() {
    let args = get_args(&["<memory address>"], 1);
    let memory_address = args[1].parse::<usize>().or_exit_with(1);

    println!(
        "The distance from address {} to address 1 is {}.",
        memory_address,
        find_distance_to_1(memory_address)
    );

    println!(
        "The first value larger than {} is {}.",
        memory_address,
        find_first_value_larger_than(memory_address)
    );
}

fn find_distance_to_1(memory_address: usize) -> i32 {
    let position = Spiral::new().nth(memory_address - 1).unwrap();
    taxi_cab_2d(position, (0, 0))
}

fn find_first_value_larger_than(memory_address: usize) -> usize {
    let mut values: HashMap<(i32, i32), usize> = HashMap::new();
    values.insert((0, 0), 1);
    for position in Spiral::new().skip(1) {
        let value = get_neighbours(position)
            .iter()
            .map(|n| values.get(n).unwrap_or(&0))
            .sum::<usize>();

        if value > memory_address {
            return value;
        }

        values.insert(position, value);
    }
    unreachable!()
}

fn get_neighbours((x, y): (i32, i32)) -> [(i32, i32); 8] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_distance_to_1() {
        assert_eq!(find_distance_to_1(1), 0);
        assert_eq!(find_distance_to_1(12), 3);
        assert_eq!(find_distance_to_1(23), 2);
        assert_eq!(find_distance_to_1(1024), 31);
    }

    #[test]
    fn test_find_first_value_larger_than() {
        assert_eq!(find_first_value_larger_than(4), 5);
        assert_eq!(find_first_value_larger_than(20), 23);
        assert_eq!(find_first_value_larger_than(100), 122);
        assert_eq!(find_first_value_larger_than(500), 747);
    }
}
