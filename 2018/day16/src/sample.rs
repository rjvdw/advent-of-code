use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Sample {
    pub before: [usize; 4],
    pub after: [usize; 4],
    pub instruction: [usize; 4],
}

impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Before: {:?}", self.before)?;
        writeln!(
            f,
            "{} {} {} {}",
            self.instruction[0], self.instruction[1], self.instruction[2], self.instruction[3]
        )?;
        writeln!(f, "After:  {:?}", self.after)?;

        Ok(())
    }
}
