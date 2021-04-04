use crate::grid::Direction;
use std::ops::Add;

/// (x, y) position for referencing values in a `MapGrid` or `VecGrid`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    /// Gets the next position if we headed in the supplied direction
    pub const fn next(&self, direction: Direction) -> Self {
        use Direction::{Down, Left, Right, Up};
        match direction {
            Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, other: (isize, isize)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}
