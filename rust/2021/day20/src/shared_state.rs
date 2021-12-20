use std::collections::HashSet;

use crate::bounds::Bounds;
use crate::point::Point;

pub struct SharedState {
    pub lit: HashSet<Point>,
    pub bounds: Bounds,
}
