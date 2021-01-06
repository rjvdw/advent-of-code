use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use combat::cave::Cave;
use combat::faction::Faction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let cave = File::open(&args[1])
        .read_multi_lines::<Cave>(1)
        .next()
        .or_exit_with(1);

    let (faction, outcome) = do_combat(&cave);
    println!(
        "The {} will win with an outcome of {}.",
        faction.plural(),
        outcome
    );
    let (attack_power, outcome) = manipulate_combat(&cave);
    println!(
        "If we increase the elves' attack power to {}, they will win with an outcome of {}.",
        attack_power, outcome
    );
}

/// Given a starting situation, let the combat play out until a faction has won.
fn do_combat(cave: &Cave) -> (Faction, usize) {
    let mut cave = cave.clone();
    let mut outcome = None;
    while outcome.is_none() {
        outcome = cave.take_turns();
    }
    outcome.unwrap()
}

/// Manipulate the battle, by giving elves higher attack power. Keep trying different attack powers
/// until we find one that allows the elves to win without suffering any losses.
fn manipulate_combat(cave: &Cave) -> (usize, usize) {
    let nr_elves = cave.get_remaining_units_count(Faction::Elf);
    let mut attack_power = 3;
    loop {
        attack_power += 1;
        let mut cave = cave.clone();
        cave.set_attack_power(Faction::Elf, attack_power);
        let mut outcome = None;
        while outcome.is_none() {
            outcome = cave.take_turns();
        }
        if let Some((Faction::Elf, outcome)) = outcome {
            if cave.get_remaining_units_count(Faction::Elf) == nr_elves {
                return (attack_power, outcome);
            }
        }
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test_do_combat_example_1() {
        let cave = get_test_input(vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Goblin, 27730));
    }

    #[test]
    fn test_do_combat_example_2() {
        let cave = get_test_input(vec![
            "#######",
            "#G..#E#",
            "#E#E.E#",
            "#G.##.#",
            "#...#E#",
            "#...E.#",
            "#######",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Elf, 36334));
    }

    #[test]
    fn test_do_combat_example_3() {
        let cave = get_test_input(vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Elf, 39514));
    }

    #[test]
    fn test_do_combat_example_4() {
        let cave = get_test_input(vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Goblin, 27755));
    }

    #[test]
    fn test_do_combat_example_5() {
        let cave = get_test_input(vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Goblin, 28944));
    }

    #[test]
    fn test_do_combat_example_6() {
        let cave = get_test_input(vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ]);
        assert_eq!(do_combat(&cave), (Faction::Goblin, 18740));
    }

    #[test]
    fn test_manipulate_combat_example_1() {
        let cave = get_test_input(vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]);
        assert_eq!(manipulate_combat(&cave), (15, 4988));
    }

    #[test]
    fn test_manipulate_combat_example_2() {
        let cave = get_test_input(vec![
            "#######",
            "#E..EG#",
            "#.#G.E#",
            "#E.##E#",
            "#G..#.#",
            "#..E#.#",
            "#######",
        ]);
        assert_eq!(manipulate_combat(&cave), (4, 31284));
    }

    #[test]
    fn test_manipulate_combat_example_3() {
        let cave = get_test_input(vec![
            "#######",
            "#E.G#.#",
            "#.#G..#",
            "#G.#.G#",
            "#G..#.#",
            "#...E.#",
            "#######",
        ]);
        assert_eq!(manipulate_combat(&cave), (15, 3478));
    }

    #[test]
    fn test_manipulate_combat_example_4() {
        let cave = get_test_input(vec![
            "#######",
            "#.E...#",
            "#.#..G#",
            "#.###.#",
            "#E#G#G#",
            "#...#G#",
            "#######",
        ]);
        assert_eq!(manipulate_combat(&cave), (12, 6474));
    }

    #[test]
    fn test_manipulate_combat_example_5() {
        let cave = get_test_input(vec![
            "#########",
            "#G......#",
            "#.E.#...#",
            "#..##..G#",
            "#...##..#",
            "#...#...#",
            "#.G...G.#",
            "#.....G.#",
            "#########",
        ]);
        assert_eq!(manipulate_combat(&cave), (34, 1140));
    }

    fn get_test_input(lines: Vec<&str>) -> Cave {
        lines
            .as_multiline_records::<Cave>()
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }
}
