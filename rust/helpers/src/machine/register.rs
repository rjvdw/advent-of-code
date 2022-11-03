use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};

/// The registers of the machine.
pub trait MachineRegister: Debug + Display {
    /// Reads a register.
    fn read(&self, key: char) -> i64;

    /// Writes a register.
    fn write(&mut self, key: char, value: i64);

    /// Clears the register.
    fn clear(&mut self);

    /// Increments (or decrements if by < 0) a register.
    fn increment(&mut self, key: char, by: i64) {
        self.write(key, self.read(key) + by)
    }

    /// Writes a boolean value (1 = true, 0 = false) to a register.
    fn write_bool(&mut self, key: char, condition: bool) {
        self.write(key, i64::from(condition));
    }
}

/// The default implementation using a HashMap.
#[derive(Debug)]
pub struct HashMapRegister {
    registers: HashMap<char, i64>,
}

impl HashMapRegister {
    /// Constructs a new HashMapRegister.
    pub fn new() -> Self {
        HashMapRegister {
            registers: HashMap::new(),
        }
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
            write!(f, "{}=", key)?;
            fmt::Display::fmt(&self.read(*key), f)?;
        }
        write!(f, "]")
    }
}

impl MachineRegister for HashMapRegister {
    fn read(&self, key: char) -> i64 {
        *self.registers.get(&key).unwrap_or(&0)
    }

    fn write(&mut self, key: char, value: i64) {
        *self.registers.entry(key).or_insert(0) = value;
    }

    fn clear(&mut self) {
        self.registers.clear();
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
