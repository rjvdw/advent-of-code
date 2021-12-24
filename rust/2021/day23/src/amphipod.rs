#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Amphipod(Color, usize);

impl Amphipod {
    /// Constructs a new amphipod
    pub fn new(color: char, discriminator: usize) -> Amphipod {
        match color {
            'A' => Amphipod(Color::Amber, discriminator),
            'B' => Amphipod(Color::Bronze, discriminator),
            'C' => Amphipod(Color::Copper, discriminator),
            _ => Amphipod(Color::Desert, discriminator),
        }
    }

    /// Checks whether two amphipods have the same type.
    pub fn has_same_type(&self, other: &Amphipod) -> bool {
        self.0 == other.0
    }

    /// The amount of energy it takes for this amphipod to move a number of steps.
    pub fn compute_energy(&self, steps: usize) -> usize {
        steps
            * match self.0 {
                Color::Amber => 1,
                Color::Bronze => 10,
                Color::Copper => 100,
                Color::Desert => 1000,
            }
    }

    pub fn get_target_side_room(&self) -> usize {
        match self.0 {
            Color::Amber => 3,
            Color::Bronze => 5,
            Color::Copper => 7,
            Color::Desert => 9,
        }
    }

    /// The unique index of this amphipod.
    pub fn index(&self) -> usize {
        self.1
    }

    /// Returns a new amphipod with an updated index.
    pub fn with_index(&self, idx: usize) -> Amphipod {
        Amphipod(self.0, idx)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Color {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Amphipod::new('A', 10), Amphipod(Color::Amber, 10));
        assert_eq!(Amphipod::new('B', 3), Amphipod(Color::Bronze, 3));
        assert_eq!(Amphipod::new('C', 12), Amphipod(Color::Copper, 12));
        assert_eq!(Amphipod::new('D', 1), Amphipod(Color::Desert, 1));
    }

    #[test]
    fn test_same_type() {
        assert!(Amphipod::new('A', 0).has_same_type(&Amphipod::new('A', 1)));
        assert!(!Amphipod::new('A', 0).has_same_type(&Amphipod::new('B', 1)));
    }

    #[test]
    fn test_compute_energy() {
        let a = Amphipod::new('A', 0);
        let b = Amphipod::new('B', 0);
        let c = Amphipod::new('C', 0);
        let d = Amphipod::new('D', 0);

        assert_eq!(a.compute_energy(10), 10);
        assert_eq!(b.compute_energy(10), 100);
        assert_eq!(c.compute_energy(10), 1000);
        assert_eq!(d.compute_energy(10), 10000);
    }

    #[test]
    fn test_get_target_side_room() {
        let a = Amphipod::new('A', 0);
        let b = Amphipod::new('B', 0);
        let c = Amphipod::new('C', 0);
        let d = Amphipod::new('D', 0);

        assert_eq!(a.get_target_side_room(), 3);
        assert_eq!(b.get_target_side_room(), 5);
        assert_eq!(c.get_target_side_room(), 7);
        assert_eq!(d.get_target_side_room(), 9);
    }

    #[test]
    fn test_index() {
        assert_eq!(Amphipod::new('A', 10).index(), 10);
    }
}
