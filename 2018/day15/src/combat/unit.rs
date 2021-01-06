use std::cmp::Ordering;
use std::fmt;

use crate::faction::Faction;
use crate::point::cmp_points;

/// A single unit participating in the combat.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Unit {
    /// The current position of the unit, in terms of (x, y).
    pub(crate) position: (usize, usize),

    /// The faction this unit is affiliated with.
    pub(crate) faction: Faction,

    /// The remaining number of hit points for this unit.
    pub(crate) health: usize,

    /// The amount of damage this unit does when attacking.
    pub(crate) attack_power: usize,
}

impl Unit {
    /// Creates a new unit at a given position.
    pub(crate) fn new(faction: Faction, position: (usize, usize)) -> Unit {
        Unit {
            position,
            faction,
            health: 200,
            attack_power: 3,
        }
    }

    /// Indicates whether this unit is still alive.
    pub(crate) fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Returns true if this unit is in an opposing faction from the other unit.
    pub(crate) fn opposes(&self, other: &Unit) -> bool {
        self.faction != other.faction
    }

    /// Take damage. The damage that is specified is subtracted from this unit's health.
    pub(crate) fn take_damage(&mut self, damage: usize) {
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
