use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::faction::Faction;
use crate::unit_group::{parse_input, UnitGroup};

mod attack_type;
mod faction;
mod unit_group;

fn main() {
    let args = get_args(&["<input  file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let unit_groups = parse_input(file).or_exit_with(1);

    match battle(&unit_groups, 0) {
        Some((faction, count)) => {
            println!(
                "The battle is won by the {:?}, with {} units remaining.",
                faction, count
            )
        }
        None => eprintln!("The battle ends in a stale-mate."),
    }

    let (boost, count) = find_minimum_boost(&unit_groups);
    println!("The minimum boost for the immunity system to win is {}. The immunity system will win with {} units remaining.", boost, count);
}

fn find_minimum_boost(unit_groups: &[UnitGroup]) -> (usize, usize) {
    let mut units;
    let mut b0 = 0;
    let mut b1 = 1;

    // Find a viable boost. Keep doubling the boost until we win.
    loop {
        if let Some((Faction::ImmuneSystem, count)) = battle(unit_groups, b1) {
            units = count;
            break;
        }

        b0 = b1;
        b1 *= 2;
    }

    // Now find the minimum boost.
    while b0 + 1 < b1 {
        let m = (b0 + b1) / 2;
        if let Some((Faction::ImmuneSystem, count)) = battle(unit_groups, m) {
            units = count;
            b1 = m;
        } else {
            b0 = m;
        }
    }

    (b1, units)
}

fn battle(unit_groups: &[UnitGroup], boost: usize) -> Option<(Faction, usize)> {
    let mut unit_groups = unit_groups.to_vec();

    for unit_group in &mut unit_groups {
        if unit_group.get_faction() == Faction::ImmuneSystem {
            unit_group.boost(boost);
        }
    }

    let mut remaining_units: HashMap<Faction, usize> = HashMap::new();
    let mut selected = vec![false; unit_groups.len()];
    let mut selections: Vec<(usize, usize)> = Vec::with_capacity(unit_groups.len());

    loop {
        // find selection order
        unit_groups.sort_unstable_by(|a, b| a.selection_phase_cmp(b));

        // clear stuff in preparation for a new round
        remaining_units.clear();
        selected.iter_mut().for_each(|v| *v = false);
        selections.clear();

        // determine how many units are left, and which of the remaining units will attack who
        for (idx, unit_group) in unit_groups.iter().enumerate() {
            *remaining_units.entry(unit_group.get_faction()).or_insert(0) +=
                unit_group.get_unit_count();

            if let Some(target) = unit_group.pick_target(&unit_groups, &selected) {
                selected[target] = true;
                selections.push((idx, target));
            }
        }

        // if either of the factions has no units left, then they have been defeated
        let immune_system_count = *remaining_units.get(&Faction::ImmuneSystem).unwrap_or(&0);
        let infection_count = *remaining_units.get(&Faction::Infection).unwrap_or(&0);
        if immune_system_count == 0 {
            return Some((Faction::Infection, infection_count));
        }
        if infection_count == 0 {
            return Some((Faction::ImmuneSystem, immune_system_count));
        }

        selections.sort_unstable_by(|(ia, _), (ib, _)| {
            let a = &unit_groups[*ia];
            let b = &unit_groups[*ib];
            a.attack_phase_cmp(b)
        });

        let mut total_units_lost = 0;
        for &(attacker_idx, target_idx) in &selections {
            let attacker = unit_groups[attacker_idx].clone();
            let target = &mut unit_groups[target_idx];

            if !attacker.is_defeated() && !target.is_defeated() {
                let (_, units_lost) = attacker.attack(target);
                total_units_lost += units_lost;
            }
        }
        if total_units_lost == 0 {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle_with_boost_0() {
        let input = vec![
            "Immune System:",
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2",
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3",
            "",
            "Infection:",
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1",
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4",
        ];
        let unit_groups = parse_input(input.join("\n").as_bytes()).unwrap();

        assert_eq!(battle(&unit_groups, 0), Some((Faction::Infection, 5216)));
    }

    #[test]
    fn test_battle_with_boost_1570() {
        let input = vec![
            "Immune System:",
            "17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2",
            "989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3",
            "",
            "Infection:",
            "801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1",
            "4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4",
        ];
        let unit_groups = parse_input(input.join("\n").as_bytes()).unwrap();

        assert_eq!(
            battle(&unit_groups, 1570),
            Some((Faction::ImmuneSystem, 51))
        );
    }
}
