use std::fs::File;
use std::ops::RangeInclusive;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;
use rdcl_aoc_helpers::part::Part;

use crate::game_objects::item::{parse_inventory_file, Item, ItemCategory};
use crate::game_objects::player::Player;

mod game_objects;

fn main() {
    let args = get_args(&["<input file>", "<inventory file>", "<hit points>"], 1);
    let boss = File::open(&args[1])
        .read_multi_lines::<Player>(1)
        .next()
        .or_exit_with(1);

    let inventory = parse_inventory_file(&args[2]).or_exit_with(1);

    let player = Player::new(args[3].parse::<u32>().or_exit_with(1));

    let min_cost = find_optimal_gear(&player, &boss, &inventory, Part::One);
    println!("The minimal cost to defeat the boss is: {}", min_cost);

    let max_cost = find_optimal_gear(&player, &boss, &inventory, Part::Two);
    println!(
        "The highest cost at which the boss will still win: {}",
        max_cost
    );
}

fn find_optimal_gear(player: &Player, boss: &Player, inventory: &[Item], part: Part) -> u32 {
    let weapons: Vec<Item> = inventory
        .iter()
        .filter(|item| item.get_category() == ItemCategory::Weapon)
        .cloned()
        .collect();

    let armor: Vec<Item> = inventory
        .iter()
        .filter(|item| item.get_category() == ItemCategory::Armor)
        .cloned()
        .collect();

    let rings: Vec<Item> = inventory
        .iter()
        .filter(|item| item.get_category() == ItemCategory::Ring)
        .cloned()
        .collect();

    let mut final_cost = match part {
        Part::One => u32::MAX,
        Part::Two => u32::MIN,
    };

    for weapon in &weapons {
        let player = player.equip(weapon);
        let cost = weapon.get_cost();
        for armor in combinations_of_range(&armor, 0..=1) {
            let mut player = player.clone();
            let mut cost = cost;
            for item in &armor {
                player = player.equip(item);
                cost += item.get_cost();
            }
            for rings in combinations_of_range(&rings, 0..=2) {
                let mut player = player.clone();
                let mut cost = cost;
                for item in &rings {
                    player = player.equip(item);
                    cost += item.get_cost();
                }

                match part {
                    Part::One => {
                        if fight(&player, boss) && cost < final_cost {
                            final_cost = cost
                        }
                    }
                    Part::Two => {
                        if !fight(&player, boss) && cost > final_cost {
                            final_cost = cost
                        }
                    }
                }
            }
        }
    }

    final_cost
}

fn fight(player: &Player, boss: &Player) -> bool {
    player.fight(boss) <= boss.fight(player)
}

fn combinations_of_range(items: &[Item], range: RangeInclusive<usize>) -> Vec<Vec<Item>> {
    let mut result = Vec::new();
    for nr in range {
        if nr == 0 {
            result.push(Vec::new());
        } else if items.len() >= nr {
            result.extend(combinations(items, nr));
        }
    }
    result
}

fn combinations(items: &[Item], nr: usize) -> Vec<Vec<Item>> {
    let mut result = Vec::new();
    if nr == 1 {
        for item in items {
            result.push(vec![item.clone()]);
        }
    } else {
        for (idx, item) in items.iter().enumerate() {
            let rest: Vec<Item> = items
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(_, item)| item)
                .cloned()
                .collect();

            for mut combination in combinations(&rest, nr - 1) {
                combination.push(item.clone());
                result.push(combination);
            }
        }
    }
    result
}
