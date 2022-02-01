use std::ops::{Add, AddAssign};
use std::str::FromStr;
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

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "up" | "N" | "north" => Ok(Self::Up),
            "D" | "down" | "S" | "south" => Ok(Self::Down),
            "L" | "left" | "W" | "west" => Ok(Self::Left),
            "R" | "right" | "E" | "east" => Ok(Self::Right),
            _ => Err(format!("Cannot parse '{}' to direction", s)),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    /// Returns a Direction from NSEW or UDLR single chars
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' | 'N' => Ok(Self::Up),
            'D' | 'S' => Ok(Self::Down),
            'L' | 'W' => Ok(Self::Left),
            'R' | 'E' => Ok(Self::Right),
            _ => Err(format!("Cannot convert '{}' to direction", c)),
        }
    }
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

impl Add<Direction> for Direction {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        match other {
            Direction::Left => self.rotate_left(),
            Direction::Right => self.rotate_right(),
            Direction::Up => self,
            Direction::Down => self.back(),
        }
    }
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, other: Direction) {
        *self = match other {
            Direction::Left => self.rotate_left(),
            Direction::Right => self.rotate_right(),
            Direction::Up => *self,
            Direction::Down => self.back(),
        }
    }
}
