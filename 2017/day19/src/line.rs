#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Line {
    Horizontal,
    Vertical,
    Junction,
    Empty,
    Letter(char),
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        match self {
            Line::Horizontal => true,
            Line::Vertical => false,
            Line::Junction => true,
            Line::Empty => false,
            Line::Letter(_) => true,
        }
    }

    pub fn is_vertical(&self) -> bool {
        match self {
            Line::Horizontal => false,
            Line::Vertical => true,
            Line::Junction => true,
            Line::Empty => false,
            Line::Letter(_) => true,
        }
    }
}
