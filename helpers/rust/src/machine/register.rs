use std::collections::HashMap;
use std::fmt;

/// The registers of the machine.
pub trait MachineRegister {
    /// Reads a register.
    fn read(&self, key: char) -> i32;

    /// Writes a register.
    fn write(&mut self, key: char, value: i32);

    /// Increments (or decrements if by < 0) a register.
    fn increment(&mut self, key: char, by: i32) {
        self.write(key, self.read(key) + by)
    }
}

/// The default implementation using a HashMap.
#[derive(Debug)]
pub struct HashMapRegister {
    registers: HashMap<char, i32>,
}

impl HashMapRegister {
    /// Constructs a new HashMapRegister.
    pub fn new() -> Self {
        HashMapRegister {
            registers: HashMap::new(),
        }
    }
}

impl MachineRegister for HashMapRegister {
    fn read(&self, key: char) -> i32 {
        *self.registers.get(&key).unwrap_or(&0)
    }

    fn write(&mut self, key: char, value: i32) {
        *self.registers.entry(key).or_insert(0) = value;
    }
}

impl Default for HashMapRegister {
    fn default() -> Self {
        HashMapRegister::new()
    }
}

impl fmt::Display for HashMapRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut keys: Vec<char> = self.registers.keys().copied().collect();
        keys.sort_unstable();

        write!(f, "[")?;
        for (i, key) in keys.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}={}", key, self.read(*key))?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_empty_register() {
        let registers = HashMapRegister::new();
        assert_eq!(registers.read('a'), 0);
    }

    #[test]
    fn test_write_and_read_register() {
        let mut registers = HashMapRegister::new();
        registers.write('a', 10);
        assert_eq!(registers.read('a'), 10);
    }

    #[test]
    fn test_increment_empty_register() {
        let mut registers = HashMapRegister::new();
        registers.increment('a', 5);
        assert_eq!(registers.read('a'), 5);
    }

    #[test]
    fn test_increment_non_empty_register() {
        let mut registers = HashMapRegister::new();
        registers.write('a', 5);
        registers.increment('a', 5);
        assert_eq!(registers.read('a'), 10);
    }

    #[test]
    fn test_format_register() {
        let mut registers = HashMapRegister::new();
        registers.write('a', 10);
        registers.write('d', -5);
        registers.write('c', 12);
        registers.write('b', 0);
        assert_eq!(format!("{}", registers), "[a=10, b=0, c=12, d=-5]");
    }
}
