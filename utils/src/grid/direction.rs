use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Four heading direction enum to aid moving around a grid
#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Rotates left
    pub const fn rotate_left(self) -> Self {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    /// Rotates right
    pub const fn rotate_right(self) -> Self {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    /// Returns all other directions
    pub const fn others(self) -> [Self; 3] {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => [Right, Down, Left],
            Right => [Up, Down, Left],
            Down => [Up, Right, Left],
            Left => [Up, Right, Down],
        }
    }

    /// Returns the opposite direction
    pub const fn back(self) -> Self {
        use Direction::{Down, Left, Right, Up};
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }

    pub fn all() -> DirectionIter {
        Direction::iter()
    }
}
