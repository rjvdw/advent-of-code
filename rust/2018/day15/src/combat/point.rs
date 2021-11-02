use std::cmp::Ordering;

/// Defines the order on points.
pub(crate) fn cmp_points((xa, ya): &(usize, usize), (xb, yb): &(usize, usize)) -> Ordering {
    ya.cmp(yb).then_with(|| xa.cmp(xb))
}
