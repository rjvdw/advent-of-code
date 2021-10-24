use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Value {
    Zero,
    One,
}

impl Value {
    pub(crate) fn from_tape(tape: &HashSet<i64>, cursor: i64) -> Value {
        if tape.contains(&cursor) {
            Value::One
        } else {
            Value::Zero
        }
    }

    pub(crate) fn to_tape(self, tape: &mut HashSet<i64>, cursor: i64) {
        match self {
            Value::Zero => {
                tape.remove(&cursor);
            }
            Value::One => {
                tape.insert(cursor);
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Zero, Value::One) => Ordering::Less,
            (Value::One, Value::Zero) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Zero => write!(f, "0"),
            Value::One => write!(f, "1"),
        }
    }
}
