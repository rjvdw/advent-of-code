use std::cmp::Ordering;

use crate::{Amphipod, Node};

const HALLWAY_X: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Burrow {
    /// The amphipods that live in this burrow.
    pub amphipods: Vec<Amphipod>,

    /// The depth of the side rooms in this burrow.
    pub side_room_depth: usize,

    /// An optimistic estimate of the minimum cost for all amphipods to reach their target burrows.
    pub cost_estimate: usize,
}

impl Burrow {
    /// Ensure that equivalent burrows are all in the same state.
    pub fn normalize(&mut self) {
        self.amphipods.sort_unstable();
    }

    /// Checks whether all amphipods are home.
    pub fn finished(&self) -> bool {
        self.amphipods.iter().all(Amphipod::is_home)
    }

    /// Try to find an amphipod that can move directly home.
    pub fn find_move_to_side_room(&self) -> Option<(Burrow, usize)> {
        for (idx, amphipod) in self.amphipods.iter().enumerate() {
            if amphipod.exhausted || amphipod.is_home() || !self.can_leave_side_room(amphipod) {
                continue;
            }

            if let Some(node) = self.find_place_in_side_room(amphipod) {
                if self.path_through_hallway_is_free(amphipod, &node) {
                    // this amphipod has a route home
                    return Some(self.with_updated_amphipod(idx, amphipod, node));
                }
            }
        }

        None
    }

    /// Try to find amphipods that can move to the hallway.
    pub fn find_moves_to_hallway(&self) -> Vec<(Burrow, usize)> {
        let mut moves = vec![];
        for (idx, amphipod) in self.amphipods.iter().enumerate() {
            if amphipod.exhausted
                || !amphipod.is_in_side_room()
                || !self.can_leave_side_room(amphipod)
            {
                continue;
            }

            let neighbours = HALLWAY_X
                .iter()
                .map(|&x| Node { y: 1, x })
                .filter(|node| self.path_through_hallway_is_free(amphipod, node))
                .filter(|node| !self.creates_deadlock(amphipod, node));

            for neighbour in neighbours {
                moves.push(self.with_updated_amphipod(idx, amphipod, neighbour));
            }
        }
        moves
    }

    /// Creates an updated state where the specified amphipod has been moved to their new location.
    fn with_updated_amphipod(&self, idx: usize, amphipod: &Amphipod, to: Node) -> (Burrow, usize) {
        let cost = amphipod.compute_cost(&to);
        let mut next_state = self.clone();
        next_state.amphipods[idx] = amphipod.with_location(to);
        next_state.cost_estimate -= amphipod.estimate_cost();
        next_state.cost_estimate += next_state.amphipods[idx].estimate_cost();
        (next_state, cost)
    }

    /// Returns true if this node is already taken up by another amphipod.
    fn is_occupied(&self, node: &Node) -> bool {
        self.amphipods
            .binary_search_by_key(node, |amphipod| amphipod.location)
            .is_ok()
    }

    /// Returns the occupant of a specific node if it exists.
    fn get_occupant(&self, node: &Node) -> Option<Amphipod> {
        let result = self
            .amphipods
            .binary_search_by_key(node, |amphipod| amphipod.location);

        if let Ok(idx) = result {
            Some(self.amphipods[idx])
        } else {
            None
        }
    }

    /// Checks if an amphipod can leave the side room they are in.
    fn can_leave_side_room(&self, amphipod: &Amphipod) -> bool {
        if amphipod.location.y > 2 {
            !self.is_occupied(&Node {
                y: amphipod.location.y - 1,
                x: amphipod.location.x,
            })
        } else {
            true
        }
    }

    /// Checks if the amphipod can find a place in their desired side room.
    fn find_place_in_side_room(&self, amphipod: &Amphipod) -> Option<Node> {
        let x = amphipod.home();
        let mut y = self.side_room_depth + 1;

        while y > 1 {
            let room = Node { y, x };
            if let Some(occupant) = self.get_occupant(&room) {
                if occupant.color != amphipod.color {
                    return None;
                } else {
                    y -= 1;
                }
            } else {
                return Some(room);
            }
        }

        None
    }

    /// Checks whether all nodes in the hallway that this amphipod would have to cross are not
    /// already occupied by other amphipods.
    fn path_through_hallway_is_free(&self, amphipod: &Amphipod, to: &Node) -> bool {
        let range = if amphipod.location.x < to.x {
            amphipod.location.x..=to.x
        } else {
            to.x..=amphipod.location.x
        };

        range
            .map(|x| Node { y: 1, x })
            .filter(|node| *node != amphipod.location)
            .all(|node| !self.is_occupied(&node))
    }

    /// If there are two amphipods in the hallway that are both blocking the other from reaching
    /// their side room, we've reached a deadlock situation.
    fn creates_deadlock(&self, amphipod: &Amphipod, to: &Node) -> bool {
        for other in &self.amphipods {
            if other.is_in_side_room() {
                continue;
            }

            let a_x = to.x;
            let a_side_room = amphipod.home();

            let b_x = other.location.x;
            let b_side_room = other.home();

            let a_blocks_b = (b_x < a_x && a_x < b_side_room) || (b_x > a_x && a_x > b_side_room);

            let b_blocks_a = (a_x < b_x && b_x < a_side_room) || (a_x > b_x && b_x > a_side_room);

            if a_blocks_b && b_blocks_a {
                return true;
            }
        }

        false
    }
}

impl PartialOrd<Self> for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost_estimate.cmp(&self.cost_estimate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let mut burrow = unfinished_burrow();
        burrow.normalize();
        assert_eq!(burrow, unfinished_burrow());
    }

    #[test]
    fn test_finished() {
        assert!(finished_burrow().finished());
        assert!(!unfinished_burrow().finished());
    }

    #[test]
    fn test_is_occupied() {
        let burrow = unfinished_burrow();

        assert!(burrow.is_occupied(&Node { y: 3, x: 3 }));
        assert!(!burrow.is_occupied(&Node { y: 1, x: 1 }));
    }

    #[test]
    fn test_get_occupant() {
        let burrow = unfinished_burrow();
        let c = Amphipod::new('C', Node { y: 3, x: 3 });

        assert_eq!(burrow.get_occupant(&Node { y: 3, x: 3 }), Some(c));
        assert_eq!(burrow.get_occupant(&Node { y: 1, x: 1 }), None);
    }

    #[test]
    fn test_can_leave_side_room() {
        let burrow = unfinished_burrow();
        let c = Amphipod::new('C', Node { y: 3, x: 3 });
        let d = Amphipod::new('D', Node { y: 2, x: 3 });

        assert!(!burrow.can_leave_side_room(&c));
        assert!(burrow.can_leave_side_room(&d));
    }

    #[test]
    fn test_find_place_in_side_room() {
        let burrow = in_progress_burrow_1();
        let a = Amphipod::new('A', Node { y: 2, x: 5 });
        let b = Amphipod::new('B', Node { y: 3, x: 7 });

        assert_eq!(
            burrow.find_place_in_side_room(&a),
            Some(Node { y: 3, x: 3 })
        );
        assert_eq!(burrow.find_place_in_side_room(&b), None);
    }

    #[test]
    fn test_path_through_hallway_is_free() {
        let burrow = in_progress_burrow_2();
        let a = Amphipod::new('A', Node { y: 2, x: 5 });

        assert!(burrow.path_through_hallway_is_free(&a, &Node { y: 1, x: 2 }));
        assert!(!burrow.path_through_hallway_is_free(&a, &Node { y: 1, x: 11 }));
    }

    #[test]
    fn test_creates_deadlock() {
        let burrow = in_progress_burrow_2();
        let b = Amphipod::new('B', Node { y: 2, x: 7 });

        assert!(burrow.creates_deadlock(&b, &Node { y: 1, x: 8 }));
        assert!(!burrow.creates_deadlock(&b, &Node { y: 1, x: 11 }));
    }

    fn finished_burrow() -> Burrow {
        Burrow {
            amphipods: vec![
                Amphipod::new('A', Node { y: 2, x: 3 }),
                Amphipod::new('B', Node { y: 2, x: 5 }),
                Amphipod::new('C', Node { y: 2, x: 7 }),
                Amphipod::new('D', Node { y: 2, x: 9 }),
                Amphipod::new('A', Node { y: 3, x: 3 }),
                Amphipod::new('B', Node { y: 3, x: 5 }),
                Amphipod::new('C', Node { y: 3, x: 7 }),
                Amphipod::new('D', Node { y: 3, x: 9 }),
            ],
            side_room_depth: 2,
            cost_estimate: 0,
        }
    }

    fn unfinished_burrow() -> Burrow {
        Burrow {
            amphipods: vec![
                Amphipod::new('D', Node { y: 2, x: 3 }),
                Amphipod::new('A', Node { y: 2, x: 5 }),
                Amphipod::new('B', Node { y: 2, x: 7 }),
                Amphipod::new('A', Node { y: 2, x: 9 }),
                Amphipod::new('C', Node { y: 3, x: 3 }),
                Amphipod::new('C', Node { y: 3, x: 5 }),
                Amphipod::new('B', Node { y: 3, x: 7 }),
                Amphipod::new('D', Node { y: 3, x: 9 }),
            ],
            side_room_depth: 2,
            cost_estimate: 0,
        }
    }

    fn in_progress_burrow_1() -> Burrow {
        Burrow {
            amphipods: vec![
                Amphipod::new('D', Node { y: 1, x: 1 }),
                Amphipod::new('C', Node { y: 1, x: 2 }),
                Amphipod::new('A', Node { y: 2, x: 5 }),
                Amphipod::new('B', Node { y: 2, x: 7 }),
                Amphipod::new('A', Node { y: 2, x: 9 }),
                Amphipod::new('C', Node { y: 3, x: 5 }),
                Amphipod::new('B', Node { y: 3, x: 7 }),
                Amphipod::new('D', Node { y: 3, x: 9 }),
            ],
            side_room_depth: 2,
            cost_estimate: 0,
        }
    }

    fn in_progress_burrow_2() -> Burrow {
        Burrow {
            amphipods: vec![
                Amphipod::new('C', Node { y: 1, x: 1 }),
                Amphipod::new('D', Node { y: 1, x: 6 }),
                Amphipod::new('A', Node { y: 2, x: 5 }),
                Amphipod::new('B', Node { y: 2, x: 7 }),
                Amphipod::new('A', Node { y: 2, x: 9 }),
                Amphipod::new('C', Node { y: 3, x: 5 }),
                Amphipod::new('B', Node { y: 3, x: 7 }),
                Amphipod::new('D', Node { y: 3, x: 9 }),
            ],
            side_room_depth: 2,
            cost_estimate: 0,
        }
    }
}
