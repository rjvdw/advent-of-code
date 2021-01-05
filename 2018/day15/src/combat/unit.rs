use std::cmp::Ordering;
use std::fmt;

use crate::combat::faction::Faction;
use crate::combat::point::cmp_points;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::combat) struct Unit {
    pub(in crate::combat) position: (usize, usize),
    pub(in crate::combat) faction: Faction,
    pub(in crate::combat) health: usize,
    pub(in crate::combat) attack_power: usize,
}

impl Unit {
    pub(in crate::combat) fn new(faction: Faction, position: (usize, usize)) -> Unit {
        Unit {
            position,
            faction,
            health: 200,
            attack_power: 3,
        }
    }

    pub(in crate::combat) fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub(in crate::combat) fn opposes(&self, other: &Unit) -> bool {
        self.faction != other.faction
    }

    pub(in crate::combat) fn take_damage(&mut self, damage: usize) {
        if damage > self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_points(&self.position, &other.position)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}[{}/200]@{},{}",
            self.faction, self.health, self.position.0, self.position.1
        )
    }
}
