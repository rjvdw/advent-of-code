/// The current state of the combat.
#[derive(Debug, Copy, Clone)]
pub(crate) enum State {
    /// The combat is over. Either some faction has won, or it's a tie.
    Over,

    /// The combat is still ongoing.
    InProgress,
}
