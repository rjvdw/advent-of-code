use crate::{Amphipod, Node};

pub trait SearchAmphipodLocations {
    fn contains_node(&self, node: &Node) -> bool;
    fn get_node(&self, node: &Node) -> Option<Amphipod>;
}

impl SearchAmphipodLocations for &[(Node, Amphipod)] {
    fn contains_node(&self, node: &Node) -> bool {
        self.iter().any(|(n, _)| n == node)
    }

    fn get_node(&self, node: &Node) -> Option<Amphipod> {
        self.iter().find(|(n, _)| n == node).map(|(_, a)| *a)
    }
}

impl SearchAmphipodLocations for Vec<(Node, Amphipod)> {
    fn contains_node(&self, node: &Node) -> bool {
        self.iter().any(|(n, _)| n == node)
    }

    fn get_node(&self, node: &Node) -> Option<Amphipod> {
        self.iter().find(|(n, _)| n == node).map(|(_, a)| *a)
    }
}

pub trait MutateAmphipodLocations {
    fn move_amphipod(&mut self, amphipod: &Amphipod, to: &Node);
}

impl MutateAmphipodLocations for Vec<(Node, Amphipod)> {
    fn move_amphipod(&mut self, amphipod: &Amphipod, to: &Node) {
        let idx = self.iter().position(|(_, a)| a == amphipod).unwrap();
        self[idx] = (*to, *amphipod);
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
    fn test_get_node_slice() {
        let amphipod = Amphipod::new('A', 0);
        let slice: &[(Node, Amphipod)] = &[(Node { x: 1, y: 2 }, amphipod)];

        assert_eq!(slice.get_node(&Node { x: 1, y: 2 }), Some(amphipod));
        assert_eq!(slice.get_node(&Node { x: 2, y: 1 }), None);
    }

    #[test]
    fn test_get_node_vec() {
        let amphipod = Amphipod::new('A', 0);
        let vec = vec![(Node { x: 1, y: 2 }, amphipod)];

        assert_eq!(vec.get_node(&Node { x: 1, y: 2 }), Some(amphipod));
        assert_eq!(vec.get_node(&Node { x: 2, y: 1 }), None);
    }

    #[test]
    fn test_move_amphipod() {
        let amphipod = Amphipod::new('A', 0);
        let mut vec = vec![(Node { x: 1, y: 2 }, amphipod)];

        vec.move_amphipod(&amphipod, &Node { x: 4, y: 5 });

        assert_eq!(vec, vec![(Node { x: 4, y: 5 }, amphipod)]);
    }
}
