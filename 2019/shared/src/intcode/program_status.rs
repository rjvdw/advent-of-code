/// Represents the current status of a program.
#[derive(Debug, Copy, Clone)]
pub enum ProgramStatus {
    Paused,
    Running,
    Halted,
}
