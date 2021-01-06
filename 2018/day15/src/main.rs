use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use crate::combat::cave::Cave;
use crate::combat::faction::Faction;

mod combat;

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
    fn test_movement() {
        let mut cave = get_test_input(vec![
            "#########",
            "#G..G..G#",
            "#.......#",
            "#.......#",
            "#G..E..G#",
            "#.......#",
            "#.......#",
            "#G..G..G#",
            "#########",
        ]);

        cave.take_turns();
        println!("{}", cave);
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#.G...G.#",
                "#...G...#",
                "#...E..G#",
                "#.G.....#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );

        cave.take_turns();
        println!("{}", cave);
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#..G.G..#",
                "#...G...#",
                "#.G.E.G.#",
                "#.......#",
                "#G..G..G#",
                "#.......#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );

        cave.take_turns();
        println!("{}", cave);
        assert_eq!(
            format!("{}", cave),
            vec![
                "#########",
                "#.......#",
                "#..GGG..#",
                "#..GEG..#",
                "#G..G...#",
                "#......G#",
                "#.......#",
                "#.......#",
                "#########",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_attack() {
        let mut cave = get_test_input(vec![
            "#######",
            "#.G...#",
            "#...EG#",
            "#.#.#G#",
            "#..G#E#",
            "#.....#",
            "#######",
        ]);

        for _ in 0..23 {
            cave.take_turns();
        }

        println!("{}", cave);
        assert_eq!(
            format!("{}", cave),
            vec![
                "#######",
                "#...G.#",
                "#..G.G#",
                "#.#.#G#",
                "#...#E#",
                "#.....#",
                "#######",
            ]
            .join("\n")
        );
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
