use std::cmp::Ordering;

use crate::{Amphipod, Node, SearchAmphipodLocations};

const HALLWAY_X: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Candidate {
    /// The amphipods in this burrow and their locations.
    pub amphipods: Vec<(Node, Amphipod)>,

    /// Keeps track of all ampipods that have moved twice already.
    pub exhausted: Vec<bool>,

    /// The depth of the side rooms.
    pub side_room_depth: usize,
}

impl Candidate {
    /// Normalize the candidate, by sorting the amphipods.
    pub fn normalize(&mut self) {
        self.amphipods
            .sort_unstable_by(|(one, _), (other, _)| one.y.cmp(&other.y).then(one.x.cmp(&other.x)));

        let mut new_exhausted = vec![false; self.exhausted.len()];
        for (i, (_, amphipod)) in self.amphipods.iter_mut().enumerate() {
            new_exhausted[i] = self.exhausted[amphipod.index()];
            *amphipod = amphipod.with_index(i);
        }
        self.exhausted = new_exhausted;
    }

    /// Checks whether all amphipods are home.
    pub fn is_done(&self) -> bool {
        self.amphipods
            .iter()
            // no need to check the y coordinate, if the x coordinate matches, they must be in a side_room
            .all(|(node, amphipod)| node.x == amphipod.get_target_side_room())
    }

    /// Checks whether there are any amphipods above the current one
    pub fn exit_is_blocked(&self, node: &Node) -> bool {
        for y in 2..node.y {
            if self.amphipods.contains_node(&Node { x: node.x, y }) {
                return true;
            }
        }
        false
    }

    /// Checks if this amphipod can move directly to their side room.
    pub fn find_move_to_side_room(&self, node: &Node, amphipod: &Amphipod) -> Option<Node> {
        if node.is_side_room() && self.exit_is_blocked(node) {
            None
        } else {
            let side_room = amphipod.get_target_side_room();
            let mut room = Node { x: side_room, y: 2 };
            let bottom_room = Node {
                x: side_room,
                y: self.side_room_depth + 1,
            };

            let path_is_blocked = !self.path_exists(node, &room);
            let side_room_is_full = self.amphipods.get_amphipod(&room).is_some();

            if path_is_blocked || side_room_is_full {
                None
            } else {
                let mut target_room = bottom_room;
                while room.y <= self.side_room_depth {
                    room.y += 1;
                    if let Some(other) = self.amphipods.get_amphipod(&room) {
                        if other.has_same_type(amphipod) {
                            if target_room.y >= room.y {
                                target_room.y = room.y - 1;
                            }
                        } else {
                            // there is an amphipod with the wrong color in this side room
                            return None;
                        }
                    }
                }

                Some(target_room)
            }
        }
    }

    /// Returns a list of nodes that this amphipod can move to.
    pub fn find_moves_to_hallway(&self, node: &Node, amphipod: &Amphipod) -> Vec<Node> {
        if node.is_side_room() {
            HALLWAY_X
                .iter()
                .copied()
                .filter(|&x| {
                    // check if there is another amphipod already in this location
                    !self.amphipods.contains_node(&Node { x, y: 1 })
                })
                .map(|x| Node { x, y: 1 })
                .filter(|next| self.path_exists(node, next))
                .filter(|next| self.wont_block(next, amphipod))
                .collect()
        } else {
            // already in the hallway
            vec![]
        }
    }

    fn path_exists(&self, from: &Node, to: &Node) -> bool {
        let range = if from.x < to.x {
            from.x..=to.x
        } else {
            to.x..=from.x
        };

        range
            .map(|x| Node { x, y: 1 })
            .filter(|node| node != from)
            .all(|node| !self.amphipods.contains_node(&node))
    }

    /// Assume two amphipods (A & B) that are both in the hallway. If A blocks B from reaching their
    /// side room and B blocks A from reaching their side room, then it's impossible for either of
    /// them to ever reach their side room.
    fn wont_block(&self, node: &Node, amphipod: &Amphipod) -> bool {
        for (other_node, other_amphipod) in &self.amphipods {
            if other_node.y == 1 {
                let a_x = node.x;
                let a_side_room = amphipod.get_target_side_room();

                let b_x = other_node.x;
                let b_side_room = other_amphipod.get_target_side_room();

                let a_blocks_b =
                    (b_x < a_x && a_x < b_side_room) || (b_x > a_x && a_x > b_side_room);

                let b_blocks_a =
                    (a_x < b_x && b_x < a_side_room) || (a_x > b_x && b_x > a_side_room);

                if a_blocks_b && b_blocks_a {
                    return false;
                }
            }
        }

        true
    }

    fn nr_exhausted(&self) -> usize {
        self.exhausted.iter().filter(|v| **v).count()
    }
}

impl PartialOrd<Self> for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.nr_exhausted().cmp(&other.nr_exhausted())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_done() {
        assert!(done_candidate().is_done());
    }

    #[test]
    fn test_not_is_done() {
        assert!(!candidate().is_done());
    }

    #[test]
    fn test_exit_is_blocked() {
        let candidate = candidate();

        assert!(candidate.exit_is_blocked(&Node { x: 3, y: 3 }));
        assert!(!candidate.exit_is_blocked(&Node { x: 3, y: 2 }));
    }

    fn done_candidate() -> Candidate {
        Candidate {
            amphipods: vec![
                (Node { x: 3, y: 2 }, Amphipod::new('A', 0)),
                (Node { x: 3, y: 3 }, Amphipod::new('A', 1)),
                (Node { x: 5, y: 2 }, Amphipod::new('B', 2)),
                (Node { x: 5, y: 3 }, Amphipod::new('B', 3)),
                (Node { x: 7, y: 2 }, Amphipod::new('C', 4)),
                (Node { x: 7, y: 3 }, Amphipod::new('C', 5)),
                (Node { x: 9, y: 2 }, Amphipod::new('D', 6)),
                (Node { x: 9, y: 3 }, Amphipod::new('D', 7)),
            ],
            exhausted: vec![],
            side_room_depth: 2,
        }
    }

    fn candidate() -> Candidate {
        Candidate {
            amphipods: vec![
                (Node { x: 3, y: 2 }, Amphipod::new('D', 0)),
                (Node { x: 3, y: 3 }, Amphipod::new('C', 1)),
                (Node { x: 5, y: 2 }, Amphipod::new('A', 2)),
                (Node { x: 5, y: 3 }, Amphipod::new('B', 3)),
                (Node { x: 7, y: 2 }, Amphipod::new('B', 4)),
                (Node { x: 7, y: 3 }, Amphipod::new('C', 5)),
                (Node { x: 9, y: 2 }, Amphipod::new('A', 6)),
                (Node { x: 9, y: 3 }, Amphipod::new('D', 7)),
            ],
            exhausted: vec![],
            side_room_depth: 2,
        }
    }
}
