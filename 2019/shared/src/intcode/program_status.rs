/// Represents the current status of a program.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProgramStatus {
    Paused,
    Running,
    Halted,
}
