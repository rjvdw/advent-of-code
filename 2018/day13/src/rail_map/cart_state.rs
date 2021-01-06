use std::fmt;

#[derive(Debug, Copy, Clone)]
pub(crate) enum CartState {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl CartState {
    pub(crate) fn faces_up(&self) -> bool {
        matches!(self, CartState::Up(_))
    }

    pub(crate) fn faces_down(&self) -> bool {
        matches!(self, CartState::Down(_))
    }

    pub(crate) fn faces_left(&self) -> bool {
        matches!(self, CartState::Left(_))
    }

    pub(crate) fn faces_right(&self) -> bool {
        matches!(self, CartState::Right(_))
    }

    pub(crate) fn pick_direction(&self) -> Self {
        match self {
            CartState::Up(0) => CartState::Left(1),
            CartState::Up(1) => CartState::Up(2),
            CartState::Up(2) => CartState::Right(0),

            CartState::Down(0) => CartState::Right(1),
            CartState::Down(1) => CartState::Down(2),
            CartState::Down(2) => CartState::Left(0),

            CartState::Left(0) => CartState::Down(1),
            CartState::Left(1) => CartState::Left(2),
            CartState::Left(2) => CartState::Up(0),

            CartState::Right(0) => CartState::Up(1),
            CartState::Right(1) => CartState::Right(2),
            CartState::Right(2) => CartState::Down(0),

            _ => unreachable!(),
        }
    }

    pub(crate) fn turn_left(&self) -> Self {
        match self {
            CartState::Up(d) => CartState::Left(*d),
            CartState::Down(d) => CartState::Right(*d),
            CartState::Left(d) => CartState::Down(*d),
            CartState::Right(d) => CartState::Up(*d),
        }
    }

    pub(crate) fn turn_right(&self) -> Self {
        match self {
            CartState::Up(d) => CartState::Right(*d),
            CartState::Down(d) => CartState::Left(*d),
            CartState::Left(d) => CartState::Up(*d),
            CartState::Right(d) => CartState::Down(*d),
        }
    }

    pub(crate) fn next(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            CartState::Up(_) if y > 0 => (x, y - 1),
            CartState::Down(_) => (x, y + 1),
            CartState::Left(_) if x > 0 => (x - 1, y),
            CartState::Right(_) => (x + 1, y),
            _ => panic!("A cart has left the map."),
        }
    }
}

impl fmt::Display for CartState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CartState::Up(_) => write!(f, "^"),
            CartState::Down(_) => write!(f, "v"),
            CartState::Left(_) => write!(f, "<"),
            CartState::Right(_) => write!(f, ">"),
        }
    }
}
