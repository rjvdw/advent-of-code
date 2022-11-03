extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use crate::snail_number::SnailNumber;

mod snail_number;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let snail_numbers = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<SnailNumber>>();

    match do_homework_1(&snail_numbers) {
        Some(v) => println!("The magnitude of the sum of the inputs is {}.", v),
        None => eprintln!("Unable to find the sum of the inputs."),
    }

    println!(
        "The largest magnitude from any two numbers is {}.",
        do_homework_2(&snail_numbers)
    );
}

fn do_homework_1(snail_numbers: &[SnailNumber]) -> Option<u64> {
    let mut iter = snail_numbers.iter();
    let mut sum = iter.next()?.clone();

    for sn in iter {
        sum = sum + sn.clone();
    }

    Some(sum.magnitude())
}

fn do_homework_2(snail_numbers: &[SnailNumber]) -> u64 {
    let mut max = u64::MIN;

    for (i, sn1) in snail_numbers.iter().enumerate() {
        for (j, sn2) in snail_numbers.iter().enumerate() {
            if i != j {
                let m = (sn1.clone() + sn2.clone()).magnitude();
                if m > max {
                    max = m;
                }
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_do_homework_1() {
        assert_eq!(do_homework_1(&snail_numbers()), Some(4140));
    }

    #[test]
    fn test_do_homework_2() {
        assert_eq!(do_homework_2(&snail_numbers()), 3993);
    }

    fn snail_numbers() -> Vec<SnailNumber> {
        vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .as_records::<SnailNumber>()
        .unwrap()
    }
}
