use std::collections::VecDeque;

use rdcl_aoc_core::error::ParseError;

use crate::operation::Operation;

#[derive(Debug, Default)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    target_if_true: usize,
    target_if_false: usize,
    inspected_items_count: usize,
}

impl Monkey {
    /// Returns the number of items the monkey has inspected so far.
    pub fn get_inspected_items_count(&self) -> usize {
        self.inspected_items_count
    }
}

/// Play one round of Keep Away.
pub fn play_round(monkeys: &mut [Monkey], worried: bool) {
    let modulus = monkeys.iter().map(|m| m.test).product::<u64>();

    for i in 0..monkeys.len() {
        while let Some(mut worry_level) = monkeys[i].items.pop_front() {
            monkeys[i].inspected_items_count += 1;
            worry_level = monkeys[i].operation.exec(worry_level, modulus);
            if !worried {
                worry_level /= 3;
            }
            let target = if worry_level % monkeys[i].test == 0 {
                monkeys[i].target_if_true
            } else {
                monkeys[i].target_if_false
            };
            monkeys[target].items.push_back(worry_level);
        }
    }
}

pub fn parse<T>(input: T) -> Result<Vec<Monkey>, ParseError>
where
    T: Iterator<Item = String>,
{
    let mut monkeys = vec![];
    let mut monkey = Monkey::default();

    for line in input {
        if line.is_empty() {
            monkeys.push(monkey);
            monkey = Monkey::default();
        } else if let Some(starting_items) = line.strip_prefix("  Starting items: ") {
            for item in starting_items.split(", ") {
                monkey.items.push_back(item.parse()?);
            }
        } else if let Some(operation) = line.strip_prefix("  Operation: ") {
            monkey.operation = operation.parse()?;
        } else if let Some(test) = line.strip_prefix("  Test: divisible by ") {
            monkey.test = test.parse()?;
        } else if let Some(if_true) = line.strip_prefix("    If true: throw to monkey ") {
            monkey.target_if_true = if_true.parse()?;
        } else if let Some(if_false) = line.strip_prefix("    If false: throw to monkey ") {
            monkey.target_if_false = if_false.parse()?;
        }
    }

    monkeys.push(monkey);

    Ok(monkeys)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_core::input::InputReader;

    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day11/test.txt").read_lines()
    }

    #[test]
    fn test_play_rounds_while_not_worried() {
        let mut monkeys = parse(test_data()).unwrap();
        let mut round = 0;

        play_round(&mut monkeys, false);
        round += 1;
        assert_eq!(
            get_worry_levels(&monkeys),
            vec![
                vec![20, 23, 27, 26],
                vec![2080, 25, 167, 207, 401, 1046],
                vec![],
                vec![],
            ]
        );

        while round < 20 {
            play_round(&mut monkeys, false);
            round += 1;
        }
        assert_eq!(
            get_worry_levels(&monkeys),
            vec![
                vec![10, 12, 14, 26, 34],
                vec![245, 93, 53, 199, 115],
                vec![],
                vec![],
            ]
        );
    }

    #[test]
    fn test_play_rounds_while_worried() {
        let mut monkeys = parse(test_data()).unwrap();
        let mut round = 0;

        play_round(&mut monkeys, true);
        round += 1;
        assert_eq!(get_counts(&monkeys), vec![2, 4, 3, 6]);

        while round < 20 {
            play_round(&mut monkeys, true);
            round += 1;
        }
        assert_eq!(get_counts(&monkeys), vec![99, 97, 8, 103]);
    }

    fn get_worry_levels(monkeys: &[Monkey]) -> Vec<Vec<u64>> {
        monkeys
            .iter()
            .map(|m| m.items.iter().copied().collect())
            .collect()
    }

    fn get_counts(monkeys: &[Monkey]) -> Vec<usize> {
        monkeys.iter().map(|m| m.inspected_items_count).collect()
    }
}
