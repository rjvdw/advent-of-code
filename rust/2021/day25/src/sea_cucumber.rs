pub enum SeaCucumber {
    East,
    South,
    Empty,
}

impl SeaCucumber {
    pub fn is_east(&self) -> bool {
        matches!(self, SeaCucumber::East)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, SeaCucumber::Empty)
    }
}

impl Default for SeaCucumber {
    fn default() -> Self {
        SeaCucumber::Empty
    }
}
