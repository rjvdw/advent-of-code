pub struct DeterministicDie {
    value: u64,
    sides: u64,
    rolls: usize,
}

impl DeterministicDie {
    pub fn new(sides: u64) -> DeterministicDie {
        DeterministicDie {
            value: 1,
            sides,
            rolls: 0,
        }
    }

    pub fn get_rolls(&self) -> usize {
        self.rolls
    }
}

impl Iterator for DeterministicDie {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.value;
        self.rolls += 1;
        self.value = if self.value == self.sides {
            1
        } else {
            self.value + 1
        };
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_die() {
        let mut die = DeterministicDie::new(5);
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.next(), Some(2));
        assert_eq!(die.next(), Some(3));
        assert_eq!(die.next(), Some(4));
        assert_eq!(die.next(), Some(5));
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.next(), Some(2));
        assert_eq!(die.next(), Some(3));
        assert_eq!(die.next(), Some(4));
        assert_eq!(die.next(), Some(5));
    }
}
