use std::ops::RangeInclusive;

pub struct MinMax {
    min: i64,
    max: i64,
}

impl MinMax {
    pub fn update(&mut self, value: i64) {
        if value < self.min {
            self.min = value;
        }
        if value > self.max {
            self.max = value;
        }
    }

    pub fn range_inclusive(&self) -> RangeInclusive<i64> {
        self.min..=self.max
    }

    pub fn len_inclusive(&self) -> i64 {
        self.max - self.min + 1
    }
}

impl Default for MinMax {
    fn default() -> Self {
        MinMax {
            min: i64::MAX,
            max: i64::MIN,
        }
    }
}
