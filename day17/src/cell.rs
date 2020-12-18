use std::hash::Hash;

pub trait Cell: Eq + Hash + Sized + Copy + Clone {
    fn from(x: i32, y: i32) -> Self;

    fn neighbours(&self) -> Vec<Self>;
}

impl Cell for (i32, i32, i32) {
    fn from(x: i32, y: i32) -> Self {
        (x, y, 0)
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut result = Vec::new();

        for d0 in -1..=1 {
            for d1 in -1..=1 {
                for d2 in -1..=1 {
                    if d0 != 0 || d1 != 0 || d2 != 0 {
                        result.push((self.0 + d0, self.1 + d1, self.2 + d2))
                    }
                }
            }
        }

        result
    }
}

impl Cell for (i32, i32, i32, i32) {
    fn from(x: i32, y: i32) -> Self {
        (x, y, 0, 0)
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut result = Vec::new();

        for d0 in -1..=1 {
            for d1 in -1..=1 {
                for d2 in -1..=1 {
                    for d3 in -1..=1 {
                        if d0 != 0 || d1 != 0 || d2 != 0 || d3 != 0 {
                            result.push((self.0 + d0, self.1 + d1, self.2 + d2, self.3 + d3))
                        }
                    }
                }
            }
        }

        result
    }
}
