use rdcl_aoc_helpers::math::taxi_cab_2d;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub y: usize,
    pub x: usize,
}

impl Node {
    /// Checks whether this node represents a side room.
    pub fn is_side_room(&self) -> bool {
        self.y > 1
    }

    /// Computes the distance between two nodes.
    pub fn distance_to(&self, other: &Node) -> usize {
        if self.y == 1 || other.y == 1 || self.x == other.x {
            // already in the hallway, or just moving within the same side room
            taxi_cab_2d((self.x, self.y), (other.x, other.y))
        } else {
            // need to move to the hallway first
            let hw = Node { y: 1, x: self.x };
            taxi_cab_2d((self.x, self.y), (hw.x, hw.y))
                + taxi_cab_2d((hw.x, hw.y), (other.x, other.y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_side_room() {
        assert!(Node { y: 3, x: 1 }.is_side_room());
        assert!(!Node { y: 1, x: 1 }.is_side_room());
    }

    #[test]
    fn test_distance_to() {
        let node_a = Node { y: 2, x: 3 };
        let node_b = Node { y: 2, x: 5 };
        let node_c = Node { y: 1, x: 4 };

        assert_eq!(node_a.distance_to(&node_a), 0);
        assert_eq!(node_a.distance_to(&node_b), 4);
        assert_eq!(node_a.distance_to(&node_c), 2);

        assert_eq!(node_b.distance_to(&node_a), 4);
        assert_eq!(node_b.distance_to(&node_b), 0);
        assert_eq!(node_b.distance_to(&node_c), 2);

        assert_eq!(node_c.distance_to(&node_a), 2);
        assert_eq!(node_c.distance_to(&node_b), 2);
        assert_eq!(node_c.distance_to(&node_c), 0);
    }
}
