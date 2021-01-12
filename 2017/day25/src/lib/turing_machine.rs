use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::instruction::Instruction;
use crate::value::Value;

/// You find the Turing machine blueprints (your puzzle input) on a tablet in a nearby pile of
/// debris. Looking back up at the broken Turing machine above, you can start to identify its parts:
#[derive(Debug, Clone)]
pub struct TuringMachine {
    /// A tape which contains 0 repeated infinitely to the left and right.
    pub(crate) tape: HashSet<i64>,

    /// A cursor, which can move left or right along the tape and read or write values at its
    /// current position.
    pub(crate) cursor: i64,

    /// A set of states, each containing rules about what to do based on the current value under the
    /// cursor.
    pub(crate) states: HashMap<(char, Value), Instruction>,

    pub(crate) diagnostic_checksum_after: usize,
    pub(crate) current_state: char,
}

impl TuringMachine {
    pub fn get_diagnostic_checksum_after(&self) -> usize {
        self.diagnostic_checksum_after
    }

    pub fn step(&mut self) {
        let state = self.current_state;
        let value = Value::from_tape(&self.tape, self.cursor);
        let instruction = self.states.get(&(state, value)).unwrap();

        instruction.write.to_tape(&mut self.tape, self.cursor);
        self.cursor = instruction.move_cursor.run(self.cursor);
        self.current_state = instruction.next_state;
    }

    pub fn checksum(&self) -> usize {
        self.tape.len()
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Begin in state {}.", self.current_state)?;
        write!(
            f,
            "Perform a diagnostic checksum after {} steps.",
            self.diagnostic_checksum_after
        )?;

        let mut states: Vec<&(char, Value)> = self.states.keys().collect();
        states.sort_unstable();

        for key in states {
            writeln!(f)?;
            if matches!(key.1, Value::Zero) {
                writeln!(f)?;
                writeln!(f, "In state {}:", key.0)?;
            }
            writeln!(f, "  If the current value is {}:", key.1)?;
            let instruction = self.states.get(key).unwrap();
            writeln!(f, "    - Write the value {}.", instruction.write)?;
            writeln!(f, "    - Move one slot to the {}.", instruction.move_cursor)?;
            write!(f, "    - Continue with state {}.", instruction.next_state)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn test_turing_machine() {
        let mut turing_machine = TuringMachine {
            tape: HashSet::new(),
            cursor: 0,
            states: HashMap::new(),
            diagnostic_checksum_after: 6,
            current_state: 'A',
        };

        turing_machine.states.insert(
            ('A', Value::Zero),
            Instruction {
                write: Value::One,
                move_cursor: Direction::Right,
                next_state: 'B',
            },
        );

        turing_machine.states.insert(
            ('A', Value::One),
            Instruction {
                write: Value::Zero,
                move_cursor: Direction::Left,
                next_state: 'B',
            },
        );

        turing_machine.states.insert(
            ('B', Value::Zero),
            Instruction {
                write: Value::One,
                move_cursor: Direction::Left,
                next_state: 'A',
            },
        );

        turing_machine.states.insert(
            ('B', Value::One),
            Instruction {
                write: Value::One,
                move_cursor: Direction::Right,
                next_state: 'A',
            },
        );

        assert_eq!(turing_machine.checksum(), 0);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 1);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 2);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 1);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 2);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 3);
        turing_machine.step();
        assert_eq!(turing_machine.checksum(), 3);
    }
}
