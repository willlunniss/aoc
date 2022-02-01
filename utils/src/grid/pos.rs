use crate::grid::Direction;
use itertools::Itertools;
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsePosError {
    #[error("invalid format: '{0}' is not a comma separated pair")]
    InvalidPair(String),
    #[error("invalid number: '{0}' is not a valid number")]
    InvalidNumber(String),
}

/// (x, y) position for referencing values in a `MapGrid` or `VecGrid`
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl FromStr for Pos {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split 'x,y' string, parse as a number and build a Pos from it
        let values = s
            .split(',')
            .map(|n| {
                n.parse()
                    .map_err(|_| ParsePosError::InvalidNumber(n.to_owned()))
            })
            .collect::<Result<Vec<isize>, _>>()?;
        if values.len() == 2 {
            Ok(Self {
                x: values[0],
                y: values[1],
            })
        } else {
            Err(ParsePosError::InvalidPair(s.to_owned()))
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(item: (i32, i32)) -> Self {
        Self {
            x: item.0 as isize,
            y: item.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Pos {
    fn from(item: (isize, isize)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(item: (usize, usize)) -> Self {
        Self {
            x: item.0 as isize,
            y: item.1 as isize,
        }
    }
}

impl TryFrom<Pos> for (usize, usize) {
    type Error = std::num::TryFromIntError;
    fn try_from(item: Pos) -> Result<(usize, usize), Self::Error> {
        Ok((isize::try_into(item.x)?, isize::try_into(item.y)?))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        let row = self.y.cmp(&other.y);
        if row == Ordering::Equal {
            self.x.cmp(&other.x)
        } else {
            row
        }
    }
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
        self.next_by(direction, 1)
    }

    /// Gets the next position if we headed distance places in the supplied direction
    pub const fn next_by(&self, direction: Direction, distance: isize) -> Self {
        use Direction::{Down, Left, Right, Up};
        match direction {
            Up => Self {
                x: self.x,
                y: self.y - distance,
            },
            Down => Self {
                x: self.x,
                y: self.y + distance,
            },
            Left => Self {
                x: self.x - distance,
                y: self.y,
            },
            Right => Self {
                x: self.x + distance,
                y: self.y,
            },
        }
    }

    /// Gets position of the 4 neighbours
    pub fn neighbours(&self) -> impl Iterator<Item = Pos> + '_ {
        Direction::all().map(move |d| self.next(d))
    }

    /// Gets position of the 8 neighbours
    pub fn neighbours8(self) -> impl Iterator<Item = Pos> {
        [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ]
        .iter()
        .map(move |shift| self + *shift)
    }

    /// Calculates the manhattan distance from another point
    pub fn manhattan_distance(&self, from: &Pos) -> usize {
        (isize::abs(from.x - self.x) + isize::abs(from.y - self.y)) as usize
    }

    /// Calculates the manhattan distance from the origin
    pub fn manhattan_distance_origin(&self) -> usize {
        (isize::abs(self.x) + isize::abs(self.y)) as usize
    }

    // Gets all positions up to and including the target position
    // Diagonals must by 45 degrees
    pub fn positions_inclusive(&self, to: &Pos) -> Vec<Pos> {
        let x_values = if to.x > self.x {
            (self.x..=to.x).collect_vec()
        } else if self.x > to.x {
            (to.x..=self.x).rev().collect_vec()
        } else {
            std::iter::repeat(self.x)
                .take((isize::abs(self.y - to.y) as usize) + 1)
                .collect_vec()
        };

        let y_values = if to.y > self.y {
            (self.y..=to.y).collect_vec()
        } else if self.y > to.y {
            (to.y..=self.y).rev().collect_vec()
        } else {
            std::iter::repeat(self.y)
                .take((isize::abs(self.x - to.x) as usize) + 1)
                .collect_vec()
        };

        // TODO: Change this to return an iterator
        x_values
            .iter()
            .zip(y_values)
            .map(|(x, y)| Pos { x: *x, y })
            .collect_vec()
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

impl Add<(usize, usize)> for Pos {
    type Output = Self;

    fn add(self, other: (usize, usize)) -> Self {
        Self {
            x: self.x + other.0 as isize,
            y: self.y + other.1 as isize,
        }
    }
}

impl Add<(u32, u32)> for Pos {
    type Output = Self;

    fn add(self, other: (u32, u32)) -> Self {
        Self {
            x: self.x + other.0 as isize,
            y: self.y + other.1 as isize,
        }
    }
}

impl Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, other: (i32, i32)) -> Self {
        Self {
            x: self.x + other.0 as isize,
            y: self.y + other.1 as isize,
        }
    }
}

impl Sub<(isize, isize)> for Pos {
    type Output = Self;

    fn sub(self, other: (isize, isize)) -> Self {
        Self {
            x: self.x - other.0,
            y: self.y - other.1,
        }
    }
}

impl Sub<(usize, usize)> for Pos {
    type Output = Self;

    fn sub(self, other: (usize, usize)) -> Self {
        Self {
            x: self.x - other.0 as isize,
            y: self.y - other.1 as isize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("1,1".parse::<Pos>(), Ok(Pos::new(1, 1)));
        assert_eq!("1,-1".parse::<Pos>(), Ok(Pos::from((1, -1i32))));
        assert_eq!(
            "1".parse::<Pos>(),
            Err(ParsePosError::InvalidPair("1".to_owned()))
        );
        assert_eq!(
            "1,n".parse::<Pos>(),
            Err(ParsePosError::InvalidNumber("n".to_owned()))
        );
        assert_eq!(
            "1,2,3".parse::<Pos>(),
            Err(ParsePosError::InvalidPair("1,2,3".to_owned()))
        );
    }
}
