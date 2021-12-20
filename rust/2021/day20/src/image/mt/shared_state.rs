use std::collections::HashSet;

use crate::bounds::Bounds;
use crate::point::Point;

pub(crate) struct SharedState {
    pub(crate) lit: HashSet<Point>,
    pub(crate) bounds: Bounds,
}
