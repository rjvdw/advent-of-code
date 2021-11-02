use std::cmp::Ordering;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;

fn main() {
    let args = get_args(&["<input file>", "<amount>"], 1);
    let containers = File::open(&args[1]).read_lines(1).collect::<Vec<u32>>();
    let amount = args[2].parse::<u32>().or_exit_with(1);

    let combinations = find_combinations(&containers, amount, 0);
    println!(
        "There are {} combinations that will fit {} liters of eggnog.",
        combinations.len(),
        amount
    );

    let count = count_combinations_using_minimum_number(&combinations);
    println!(
        "There are {} combinations that use the smallest number of containers.",
        count
    );
}

fn find_combinations(containers: &[u32], amount: u32, used: usize) -> Vec<usize> {
    if let Some((&container, rest)) = containers.split_first() {
        let mut combinations = find_combinations(rest, amount, used);

        match container.cmp(&amount) {
            Ordering::Less => {
                combinations.extend(find_combinations(rest, amount - container, used + 1));
            }
            Ordering::Equal => {
                combinations.push(used + 1);
            }
            _ => {}
        }

        combinations
    } else {
        Vec::new()
    }
}

fn count_combinations_using_minimum_number(combinations: &[usize]) -> usize {
    let min = combinations.iter().min().cloned().unwrap();
    combinations.iter().filter(|&&x| x == min).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_combinations() {
        let containers = vec![20, 15, 10, 5, 5];
        assert_eq!(find_combinations(&containers, 25, 0).len(), 4);
    }

    #[test]
    fn test_count_combinations_using() {
        let containers = vec![20, 15, 10, 5, 5];
        let combinations = find_combinations(&containers, 25, 0);
        assert_eq!(count_combinations_using_minimum_number(&combinations), 3);
    }
}
