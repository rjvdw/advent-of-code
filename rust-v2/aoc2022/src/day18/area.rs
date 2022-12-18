use std::fmt;

use crate::point::Point;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Area {
    pub plane: Plane,
    pub position: Point,
}

impl Area {
    pub fn new(plane: Plane, position: Point) -> Area {
        Area { plane, position }
    }
}

impl fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Area in {:?} @{:?}", self.plane, self.position)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Plane {
    X,
    Y,
    Z,
}

impl fmt::Debug for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Plane::X => write!(f, "X-plane"),
            Plane::Y => write!(f, "Y-plane"),
            Plane::Z => write!(f, "Z-plane"),
        }
    }
}
