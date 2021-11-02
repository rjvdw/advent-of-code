use std::fmt;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Track {
    Vertical,
    Horizontal,
    Intersection,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Track::Vertical => write!(f, "|"),
            Track::Horizontal => write!(f, "-"),
            Track::Intersection => write!(f, "+"),
            Track::TopLeft => write!(f, "/"),
            Track::TopRight => write!(f, "\\"),
            Track::BottomLeft => write!(f, "\\"),
            Track::BottomRight => write!(f, "/"),
            Track::None => write!(f, " "),
        }
    }
}
