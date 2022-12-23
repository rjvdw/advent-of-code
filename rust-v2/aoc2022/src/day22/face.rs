#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Face {
    /// This face is used to represent a flat map (i.e. not a cube).
    F0,

    /// The 1-side of a die.
    F1,

    /// The 2-side of a die.
    F2,

    /// The 3-side of a die.
    F3,

    /// The 4-side of a die.
    F4,

    /// The 5-side of a die.
    F5,

    /// The 6-side of a die.
    F6,
}

impl Face {
    pub fn is_cube(&self) -> bool {
        !matches!(self, Face::F0)
    }

    pub fn determine_face(cube_size: usize, position: (usize, usize)) -> Option<Face> {
        let row = (position.1 - 1) / cube_size;
        let col = (position.0 - 1) / cube_size;

        match (col, row) {
            (1, 0) => Some(Face::F1),
            (2, 0) => Some(Face::F3),
            (1, 1) => Some(Face::F2),
            (0, 2) => Some(Face::F4),
            (1, 2) => Some(Face::F6),
            (0, 3) => Some(Face::F5),
            _ => None,
        }
    }

    pub fn xy(&self, cube_size: usize) -> (usize, usize) {
        match self {
            Face::F0 => (1, 1),
            Face::F1 => (1 + cube_size, 1),
            Face::F2 => (1 + cube_size, 1 + cube_size),
            Face::F3 => (1 + 2 * cube_size, 1),
            Face::F4 => (1, 1 + 2 * cube_size),
            Face::F5 => (1, 1 + 3 * cube_size),
            Face::F6 => (1 + cube_size, 1 + 2 * cube_size),
        }
    }

    pub fn x(&self, cube_size: usize) -> usize {
        self.xy(cube_size).0
    }

    pub fn y(&self, cube_size: usize) -> usize {
        self.xy(cube_size).1
    }
}
