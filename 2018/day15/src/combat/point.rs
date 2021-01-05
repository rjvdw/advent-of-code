use std::cmp::Ordering;

/// Defines the order on points.
pub(in crate::combat) fn cmp_points(
    (xa, ya): &(usize, usize),
    (xb, yb): &(usize, usize),
) -> Ordering {
    ya.cmp(yb).then_with(|| xa.cmp(xb))
}

/// The distance between two points.
pub(in crate::combat) fn compute_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (if p1.0 < p2.0 {
        p2.0 - p1.0
    } else {
        p1.0 - p2.0
    }) + (if p1.1 < p2.1 {
        p2.1 - p1.1
    } else {
        p1.1 - p2.1
    })
}
