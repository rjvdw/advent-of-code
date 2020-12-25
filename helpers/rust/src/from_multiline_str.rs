/// Mirrors `std::str::FromStr`, but slightly modified so it can be used to parse record that span
/// multiple lines.
pub trait FromMultilineStr {
    /// If true, all lines before the first line for which `self.indicates_new_record(line)` returns
    /// true are discarded.
    const DISCARD_FIRST_RECORD: bool;

    /// The associated error which can be returned from parsing.
    type Err;

    /// Create a new initial record.
    fn new() -> Self;

    /// Test for whether a line indicates that a new record starts.
    fn indicates_new_record(line: &str) -> bool;

    /// Parses a line.
    fn parse(&mut self, line: &str) -> Result<(), Self::Err>;
}
