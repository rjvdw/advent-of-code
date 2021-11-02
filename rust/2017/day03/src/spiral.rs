pub struct Spiral {
    position: (i32, i32),
    direction: u8,
    size: usize,
    d: usize,
}

impl Spiral {
    pub fn new() -> Self {
        Spiral::default()
    }
}

impl Default for Spiral {
    fn default() -> Self {
        Spiral {
            position: (0, 0),
            direction: 0,
            size: 1,
            d: 0,
        }
    }
}

impl Iterator for Spiral {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.position;
        self.position = match self.direction {
            0 => (current.0 + 1, current.1),
            1 => (current.0, current.1 - 1),
            2 => (current.0 - 1, current.1),
            3 => (current.0, current.1 + 1),
            _ => unreachable!(),
        };
        self.d += 1;
        if self.d == self.size {
            self.direction = (self.direction + 1) % 4;
            self.d = 0;
            if self.direction % 2 == 0 {
                self.size += 1;
            }
        }
        Some(current)
    }
}
