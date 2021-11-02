use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

use crate::attack_type::AttackType;
use crate::faction::Faction;

#[derive(Debug, Clone)]
pub struct UnitGroup {
    faction: Faction,
    group_size: usize,
    hit_points: usize,
    weak_to: HashSet<AttackType>,
    immune_to: HashSet<AttackType>,
    attack_type: AttackType,
    attack_power: usize,
    initiative: usize,
}

impl UnitGroup {
    /// Returns the faction of this group.
    pub fn get_faction(&self) -> Faction {
        self.faction
    }

    /// Returns how many units are left in this group.
    pub fn get_unit_count(&self) -> usize {
        self.group_size
    }

    /// Returns true if this group has been defeated.
    pub fn is_defeated(&self) -> bool {
        self.group_size == 0
    }

    /// Boosts this group's attack power.
    pub fn boost(&mut self, value: usize) {
        self.attack_power += value;
    }

    /// Determines in which order units get to select their targets.
    pub fn selection_phase_cmp(&self, other: &UnitGroup) -> Ordering {
        self.determine_effective_power()
            .cmp(&other.determine_effective_power())
            .then_with(|| self.initiative.cmp(&other.initiative))
            .reverse()
    }

    /// Determines in which order attacks are carried out.
    pub fn attack_phase_cmp(&self, other: &UnitGroup) -> Ordering {
        self.initiative.cmp(&other.initiative).reverse()
    }

    /// Select a target to attack.
    pub fn pick_target(&self, others: &[UnitGroup], selected: &[bool]) -> Option<usize> {
        if self.is_defeated() {
            None
        } else {
            others
                .iter()
                .enumerate()
                .filter(|(_, other)| other.faction != self.faction)
                .filter(|(idx, _)| !selected[*idx])
                .filter(|(_, other)| !other.is_defeated())
                .map(|(idx, other)| {
                    (
                        self.determine_damage_against(other),
                        other.determine_effective_power(),
                        other.initiative,
                        idx,
                    )
                })
                .filter(|&(dmg, _, _, _)| dmg > 0)
                .max_by(|&a, &b| {
                    a.0.cmp(&b.0)
                        .then_with(|| a.1.cmp(&b.1))
                        .then_with(|| a.2.cmp(&b.2))
                })
                .map(|(_, _, _, idx)| idx)
        }
    }

    /// Attack some other group.
    pub fn attack(&self, other: &mut UnitGroup) -> (usize, usize) {
        assert_ne!(self.faction, other.faction);

        let damage_dealt = self.determine_damage_against(other);
        let units_lost = other.group_size.min(damage_dealt / other.hit_points);
        other.group_size -= units_lost;

        (damage_dealt, units_lost)
    }

    /// Determine this group's effective power.
    fn determine_effective_power(&self) -> usize {
        self.group_size * self.attack_power
    }

    /// Determine how much group would be dealt against another group.
    fn determine_damage_against(&self, against: &UnitGroup) -> usize {
        if against.immune_to.contains(&self.attack_type) {
            0
        } else if against.weak_to.contains(&self.attack_type) {
            self.determine_effective_power() * 2
        } else {
            self.determine_effective_power()
        }
    }
}

impl Default for UnitGroup {
    fn default() -> Self {
        UnitGroup {
            faction: Faction::ImmuneSystem,
            group_size: 0,
            hit_points: 0,
            weak_to: Default::default(),
            immune_to: Default::default(),
            attack_type: AttackType::Bludgeoning,
            attack_power: 0,
            initiative: 0,
        }
    }
}

impl fmt::Display for UnitGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} units each with {} hit points",
            self.group_size, self.hit_points
        )?;
        if !self.immune_to.is_empty() || !self.weak_to.is_empty() {
            write!(f, " (")?;
            if !self.immune_to.is_empty() {
                let mut immunities: Vec<String> =
                    self.immune_to.iter().map(|at| at.to_string()).collect();
                immunities.sort_unstable();
                write!(f, "immune to {}", immunities.join(", "))?;

                if !self.weak_to.is_empty() {
                    write!(f, "; ")?;
                }
            }
            if !self.weak_to.is_empty() {
                let mut weaknesses: Vec<String> =
                    self.weak_to.iter().map(|at| at.to_string()).collect();
                weaknesses.sort_unstable();
                write!(f, "weak to {}", weaknesses.join(", "))?;
            }
            write!(f, ")")?;
        }
        write!(
            f,
            " with an attack that does {} {} damage at initiative {}",
            self.attack_power, self.attack_type, self.initiative
        )
    }
}

impl FromStr for UnitGroup {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut unit = UnitGroup::default();
        let mut parsing = Parsing::GroupSize;

        for word in line.split_whitespace() {
            match parsing {
                Parsing::GroupSize => {
                    unit.group_size = word.parse()?;
                    parsing = Parsing::HitPoints;
                }
                Parsing::HitPoints => match word {
                    "(weak" => {
                        parsing = Parsing::Weaknesses;
                    }
                    "(immune" => {
                        parsing = Parsing::Immunities;
                    }
                    "attack" => {
                        parsing = Parsing::AttackPower;
                    }
                    _ => {
                        if let Ok(hit_points) = word.parse::<usize>() {
                            unit.hit_points = hit_points;
                        }
                    }
                },
                Parsing::Weaknesses => match word {
                    "immune" => {
                        parsing = Parsing::Immunities;
                    }
                    "attack" => {
                        parsing = Parsing::AttackPower;
                    }
                    _ => {
                        let trimmed = &word[..word.len() - 1];
                        if let Ok(attack_type) = trimmed.parse::<AttackType>() {
                            unit.weak_to.insert(attack_type);
                        }
                    }
                },
                Parsing::Immunities => match word {
                    "weak" => {
                        parsing = Parsing::Weaknesses;
                    }
                    "attack" => {
                        parsing = Parsing::AttackPower;
                    }
                    _ => {
                        let trimmed = &word[..word.len() - 1];
                        if let Ok(attack_type) = trimmed.parse::<AttackType>() {
                            unit.immune_to.insert(attack_type);
                        }
                    }
                },
                Parsing::AttackPower => {
                    if word != "that" && word != "does" {
                        unit.attack_power = word.parse()?;
                        parsing = Parsing::AttackType;
                    }
                }
                Parsing::AttackType => {
                    unit.attack_type = word.parse()?;
                    parsing = Parsing::Initiative;
                }
                Parsing::Initiative => {
                    if let Ok(initiative) = word.parse::<usize>() {
                        unit.initiative = initiative;
                    }
                }
            }
        }

        Ok(unit)
    }
}

pub fn parse_input<R: Read>(read: R) -> Result<Vec<UnitGroup>, ParseError> {
    let mut unit_groups = Vec::new();
    let mut faction = Faction::ImmuneSystem;

    for line in BufReader::new(read).lines() {
        let line = line?;
        if line.is_empty() {
            // noop
        } else if line.starts_with("Immune System:") {
            faction = Faction::ImmuneSystem;
        } else if line.starts_with("Infection") {
            faction = Faction::Infection;
        } else {
            let mut unit_group = line.parse::<UnitGroup>()?;
            unit_group.faction = faction;
            unit_groups.push(unit_group);
        }
    }

    Ok(unit_groups)
}

#[derive(Debug)]
enum Parsing {
    GroupSize,
    HitPoints,
    Weaknesses,
    Immunities,
    AttackPower,
    AttackType,
    Initiative,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_damage_against() {
        let attackers = UnitGroup {
            group_size: 2,
            attack_power: 5,
            ..Default::default()
        };
        let defenders = UnitGroup::default();

        assert_eq!(attackers.determine_damage_against(&defenders), 10);
    }

    #[test]
    fn test_determine_damage_against_with_immunities() {
        let attackers = UnitGroup {
            group_size: 2,
            attack_power: 5,
            attack_type: AttackType::Radiation,
            ..Default::default()
        };
        let mut defenders = UnitGroup::default();
        defenders.immune_to.insert(AttackType::Radiation);

        assert_eq!(attackers.determine_damage_against(&defenders), 0);
    }

    #[test]
    fn test_determine_damage_against_with_weaknesses() {
        let attackers = UnitGroup {
            group_size: 2,
            attack_power: 5,
            attack_type: AttackType::Radiation,
            ..Default::default()
        };
        let mut defenders = UnitGroup::default();
        defenders.weak_to.insert(AttackType::Radiation);

        assert_eq!(attackers.determine_damage_against(&defenders), 20);
    }

    #[test]
    fn test_attack() {
        let attackers = UnitGroup {
            faction: Faction::ImmuneSystem,
            group_size: 100,
            attack_power: 5,
            ..Default::default()
        };

        let mut defenders = UnitGroup {
            faction: Faction::Infection,
            group_size: 250,
            hit_points: 5,
            ..Default::default()
        };

        attackers.attack(&mut defenders);

        assert_eq!(defenders.group_size, 150);
    }

    #[test]
    fn test_attack_where_ap_and_hp_are_coprime() {
        let attackers = UnitGroup {
            faction: Faction::ImmuneSystem,
            group_size: 100,
            attack_power: 7,
            ..Default::default()
        };

        let mut defenders = UnitGroup {
            faction: Faction::Infection,
            group_size: 250,
            hit_points: 3,
            ..Default::default()
        };

        attackers.attack(&mut defenders);

        assert_eq!(defenders.group_size, 17);
    }

    #[test]
    fn test_attack_with_immunities() {
        let attackers = UnitGroup {
            faction: Faction::ImmuneSystem,
            group_size: 100,
            attack_type: AttackType::Bludgeoning,
            attack_power: 5,
            ..Default::default()
        };

        let mut defenders = UnitGroup {
            faction: Faction::Infection,
            group_size: 250,
            hit_points: 5,
            ..Default::default()
        };
        defenders.immune_to.insert(AttackType::Bludgeoning);

        attackers.attack(&mut defenders);

        assert_eq!(defenders.group_size, 250);
    }

    #[test]
    fn test_attack_with_weaknesses() {
        let attackers = UnitGroup {
            faction: Faction::ImmuneSystem,
            group_size: 100,
            attack_type: AttackType::Bludgeoning,
            attack_power: 5,
            ..Default::default()
        };

        let mut defenders = UnitGroup {
            faction: Faction::Infection,
            group_size: 250,
            hit_points: 5,
            ..Default::default()
        };
        defenders.weak_to.insert(AttackType::Bludgeoning);

        attackers.attack(&mut defenders);

        assert_eq!(defenders.group_size, 50);
    }
}
