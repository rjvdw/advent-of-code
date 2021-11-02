use crate::direction::Direction;
use crate::value::Value;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Instruction {
    pub(crate) write: Value,
    pub(crate) move_cursor: Direction,
    pub(crate) next_state: char,
}
