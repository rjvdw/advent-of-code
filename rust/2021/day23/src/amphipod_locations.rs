use crate::{Amphipod, Node};

pub trait SearchAmphipodLocations {
    fn contains_node(&self, node: &Node) -> bool;
    fn get_amphipod(&self, node: &Node) -> Option<Amphipod>;
}

impl SearchAmphipodLocations for &[(Node, Amphipod)] {
    fn contains_node(&self, node: &Node) -> bool {
        self.iter().any(|(n, _)| n == node)
    }

    fn get_amphipod(&self, node: &Node) -> Option<Amphipod> {
        self.iter().find(|(n, _)| n == node).map(|(_, a)| *a)
    }
}

impl SearchAmphipodLocations for Vec<(Node, Amphipod)> {
    fn contains_node(&self, node: &Node) -> bool {
        self.iter().any(|(n, _)| n == node)
    }

    fn get_amphipod(&self, node: &Node) -> Option<Amphipod> {
        self.iter().find(|(n, _)| n == node).map(|(_, a)| *a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_node_slice() {
        let slice: &[(Node, Amphipod)] = &[(Node { x: 1, y: 2 }, Amphipod::new('A', 0))];

        assert!(slice.contains_node(&Node { x: 1, y: 2 }));
        assert!(!slice.contains_node(&Node { x: 2, y: 1 }));
    }

    #[test]
    fn test_contains_node_vec() {
        let vec = vec![(Node { x: 1, y: 2 }, Amphipod::new('A', 0))];

        assert!(vec.contains_node(&Node { x: 1, y: 2 }));
        assert!(!vec.contains_node(&Node { x: 2, y: 1 }));
    }

    #[test]
    fn test_get_amphipod_slice() {
        let amphipod = Amphipod::new('A', 0);
        let slice: &[(Node, Amphipod)] = &[(Node { x: 1, y: 2 }, amphipod)];

        assert_eq!(slice.get_amphipod(&Node { x: 1, y: 2 }), Some(amphipod));
        assert_eq!(slice.get_amphipod(&Node { x: 2, y: 1 }), None);
    }

    #[test]
    fn test_get_amphipod_vec() {
        let amphipod = Amphipod::new('A', 0);
        let vec = vec![(Node { x: 1, y: 2 }, amphipod)];

        assert_eq!(vec.get_amphipod(&Node { x: 1, y: 2 }), Some(amphipod));
        assert_eq!(vec.get_amphipod(&Node { x: 2, y: 1 }), None);
    }
}
