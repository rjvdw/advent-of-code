#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::cave) struct Reachable {
    pub(in crate::cave) position: (usize, usize),
    pub(in crate::cave) distance: usize,
    pub(in crate::cave) key: char,
}
