#[derive(Debug, Copy, Clone, Default)]
pub struct SubRoutine {
    pub divides: bool,
    pub constant_1: i64,
    pub constant_2: i64,
}

impl SubRoutine {
    pub fn get_x_value(&self, z: i64) -> i64 {
        z % 26 + self.constant_1
    }

    pub fn run(&self, w: i64, mut z: i64) -> i64 {
        let x = self.get_x_value(z);

        if self.divides {
            z /= 26;
        }

        if x != w {
            z = z * 26 + w + self.constant_2;
        }

        z
    }
}
