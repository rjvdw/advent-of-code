use std::cmp::Ordering;

use crate::Amphipod;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct State {
    pub amphipods: Vec<Amphipod>,
    pub side_room_depth: usize,
}

impl State {
    /// Ensures that states that are the same also look the same. This prevents us from doing
    /// unnecessary work.
    pub fn normalize(&mut self) {
        self.amphipods.sort_unstable();
    }

    /// Checks whether all amphipods are home.
    pub fn finished(&self) -> bool {
        self.amphipods.iter().all(|amphipod| amphipod.is_home())
    }

    /// Try to find an amphipod that can move directly home.
    pub fn find_move_to_side_room(&self) -> Option<(State, usize)> {
        todo!()
    }

    /// Try to find amphipods that can move to the hallway.
    pub fn find_moves_to_hallway(&self) -> Vec<(State, usize)> {
        todo!()
    }

    /// Count the number of amphipods that are currently exhausted (i.e. no longer able to make any
    /// moves).
    fn nr_exhausted(&self) -> usize {
        self.amphipods
            .iter()
            .filter(|amphipod| amphipod.exhausted)
            .count()
    }
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nr_exhausted().cmp(&other.nr_exhausted())
    }
}
