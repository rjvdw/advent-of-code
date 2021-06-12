use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;
use rdcl_aoc_helpers::part::Part;

use crate::game_objects::boss::Boss;
use crate::game_objects::player::Player;
use crate::game_objects::spell::Spell;

mod game_objects;

fn main() {
    let args = get_args(&["<input file>", "<hit points>", "<mana>"], 1);
    let boss = File::open(&args[1])
        .read_multi_lines::<Boss>(1)
        .next()
        .or_exit_with(1);
    let hit_points = args[2].parse().or_exit_with(1);
    let mana = args[3].parse().or_exit_with(1);
    let player = Player::new(hit_points, mana);

    match fight(&player, &boss, &Part::One) {
        Some(min_mana_cost) => println!(
            "[Normal Mode] The minimum mana cost to win is: {}.",
            min_mana_cost
        ),
        None => println!("The player will always lose."),
    };

    match fight(&player, &boss, &Part::Two) {
        Some(min_mana_cost) => println!(
            "[Hard Mode] The minimum mana cost to win is: {}.",
            min_mana_cost
        ),
        None => println!("The player will always lose."),
    };
}

fn fight(player: &Player, boss: &Boss, part: &Part) -> Option<u32> {
    let mut game_states: Vec<(Player, Boss, u32)> = vec![(player.clone(), boss.clone(), 0)];

    let mut best_so_far: Option<u32> = None;

    while !game_states.is_empty() {
        let mut next_game_states: Vec<(Player, Boss, u32)> = Vec::new();
        for (player, boss, cost_so_far) in game_states {
            for spell in &Spell::SPELLS {
                let cost = cost_so_far + spell.get_cost();

                if let Some(best_so_far) = best_so_far {
                    if cost >= best_so_far {
                        continue;
                    }
                }

                if let Some((player, boss, won, lost)) =
                    turn(player.clone(), boss.clone(), spell, part)
                {
                    if won {
                        best_so_far = Some(cost);
                    } else if !lost {
                        next_game_states.push((player, boss, cost));
                    }
                }
            }
        }
        game_states = next_game_states;
    }

    best_so_far
}

fn turn(
    mut player: Player,
    mut boss: Boss,
    spell: &Spell<'static>,
    part: &Part,
) -> Option<(Player, Boss, bool, bool)> {
    if let Part::Two = part {
        let (result, is_defeated) = player.take_damage(1);
        player = result;
        if is_defeated {
            return Some((player, boss, false, true));
        }
    }

    // apply effects
    let (result, damage) = player.apply_effects();
    player = result;
    let (result, is_defeated) = boss.take_damage(damage);
    boss = result;
    if is_defeated {
        return Some((player, boss, true, false));
    }

    // player turn
    let (result, damage) = player.cast(spell)?;
    player = result;
    let (result, is_defeated) = boss.take_damage(damage);
    boss = result;
    if is_defeated {
        return Some((player, boss, true, false));
    }

    // apply effects
    let (result, damage) = player.apply_effects();
    player = result;
    let (result, is_defeated) = boss.take_damage(damage);
    boss = result;
    if is_defeated {
        return Some((player, boss, true, false));
    }

    // boss turn
    let (result, is_defeated) = boss.attack(&player);
    player = result;

    // end of turn
    Some((player, boss, false, is_defeated))
}
