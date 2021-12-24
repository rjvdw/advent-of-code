use std::cmp::Ordering;

use crate::node::Node;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Amphipod {
    pub color: AmphipodColor,
    pub exhausted: bool,
    pub location: Node,
}

impl Amphipod {
    /// Constructs a new Amphipod.
    pub fn new(ch: char, location: Node) -> Amphipod {
        Amphipod {
            color: AmphipodColor::from_char(ch),
            exhausted: false,
            location,
        }
    }

    /// Computes the cost for an amphipod to walk to a new location.
    pub fn compute_cost(&self, to: &Node) -> usize {
        self.color.cost() * self.location.distance_to(to)
    }

    /// Returns the x coordinate of the side room this amphipod needs to move to.
    pub fn get_home(&self) -> usize {
        self.color.home()
    }

    /// Checks whether an amphipod is in their desired side room.
    pub fn is_home(&self) -> bool {
        self.color.home() == self.location.x
    }
}

impl PartialOrd<Self> for Amphipod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Amphipod {
    fn cmp(&self, other: &Self) -> Ordering {
        self.location
            .y
            .cmp(&other.location.y)
            .then(self.location.x.cmp(&other.location.x))
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AmphipodColor {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl AmphipodColor {
    fn from_char(ch: char) -> AmphipodColor {
        match ch {
            'A' => AmphipodColor::Amber,
            'B' => AmphipodColor::Bronze,
            'C' => AmphipodColor::Copper,
            _ => AmphipodColor::Desert,
        }
    }

    pub fn cost(&self) -> usize {
        match self {
            AmphipodColor::Amber => 1,
            AmphipodColor::Bronze => 10,
            AmphipodColor::Copper => 100,
            AmphipodColor::Desert => 1000,
        }
    }

    pub fn home(&self) -> usize {
        match self {
            AmphipodColor::Amber => 3,
            AmphipodColor::Bronze => 5,
            AmphipodColor::Copper => 7,
            AmphipodColor::Desert => 9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost() {
        let a = Amphipod::new('A', Node { y: 3, x: 3 });
        let b = Amphipod::new('B', Node { y: 3, x: 3 });
        let c = Amphipod::new('C', Node { y: 3, x: 3 });
        let d = Amphipod::new('D', Node { y: 3, x: 3 });

        assert_eq!(a.compute_cost(&Node { y: 1, x: 4 }), 3);
        assert_eq!(b.compute_cost(&Node { y: 1, x: 4 }), 30);
        assert_eq!(c.compute_cost(&Node { y: 1, x: 4 }), 300);
        assert_eq!(d.compute_cost(&Node { y: 1, x: 4 }), 3000);

        assert_eq!(a.compute_cost(&Node { y: 2, x: 5 }), 5);
        assert_eq!(b.compute_cost(&Node { y: 2, x: 5 }), 50);
        assert_eq!(c.compute_cost(&Node { y: 2, x: 5 }), 500);
        assert_eq!(d.compute_cost(&Node { y: 2, x: 5 }), 5000);
    }
}
