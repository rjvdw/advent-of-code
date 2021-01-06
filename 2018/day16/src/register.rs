use std::collections::HashMap;

pub struct Register(HashMap<usize, usize>);

impl Register {
    pub fn new() -> Register {
        Register(HashMap::new())
    }

    pub fn get(&self, register: &usize) -> usize {
        *self.0.get(register).unwrap_or(&0)
    }

    pub fn set(&mut self, register: &usize, value: usize) {
        self.0.insert(*register, value);
    }

    pub fn set_bool(&mut self, register: &usize, condition: bool) {
        self.0.insert(*register, if condition { 1 } else { 0 });
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new()
    }
}
