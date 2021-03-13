use std::collections::HashMap;
use std::fmt::Display;
use std::slice::Iter;

/// Four heading direction enum to aid moving around a grid
#[derive(Debug, Copy, Clone)]
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
}

/// (x, y) position for used to reference values on a grid
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
        use Direction::*;
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

/// Grid that uses nested vectors to store data
#[derive(Clone)]
pub struct VecGrid<V> {
    pub data: Vec<Vec<V>>,
}

impl<V: Clone + Copy> Default for VecGrid<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Copy> VecGrid<V> {
    /// Creates a new empty `VecGrid`
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn iter(&self) -> Iter<'_, Vec<V>> {
        self.data.iter()
    }

    /// Gets the values of the 4 neighbours to the supplied position
    pub fn neighbours(&self, pos: Pos) -> Vec<Option<V>> {
        use Direction::{Down, Left, Right, Up};
        return [Up, Right, Down, Left]
            .iter()
            .map(|d| self.get(pos.next(*d)))
            .collect();
    }

    /// Gets the element at the supplied position or None if it is outside of bounds
    pub fn get(&self, pos: Pos) -> Option<V> {
        if self.contains(pos) {
            Some(self.data[pos.y as usize][pos.x as usize])
        } else {
            None // Outside of bounds
        }
    }

    /// Sets the value at the specified position
    pub fn set(&mut self, pos: Pos, value: V) {
        self.data[pos.y as usize][pos.x as usize] = value;
    }

    /// Checks whether the supplied position exists within the grid
    pub fn contains(&self, pos: Pos) -> bool {
        pos.y >= 0
            && pos.y < self.data.len() as isize - 1
            && pos.x >= 0
            && pos.x < self.data[0].len() as isize
    }

    /// Prints the grid to the console
    pub fn print(&self)
    where
        V: Display,
    {
        for row in &self.data {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

/// Grid that uses a HashMap to store data
///
/// Especially useful where you don't know the full size in advance
pub struct MapGrid<V> {
    data: HashMap<Pos, V>,
}

impl<V: Clone + Copy> Default for MapGrid<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Copy> MapGrid<V> {
    /// Creates a new empty `MapGrid`
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Inserts the element into the supplied position
    pub fn insert(&mut self, pos: Pos, value: V) {
        self.data.insert(pos, value);
    }

    /// Gets the element at the supplied position
    pub fn get(&self, pos: Pos) -> Option<&V> {
        self.data.get(&pos)
    }
    /// Converts a `HashMap` based grid to a nested vector
    ///
    /// Any cells within the bounds of those specified in map that don't have values
    /// by be set to default
    pub fn to_vec(&self, default: V) -> Vec<Vec<V>> {
        // 1st pass: Find the size of the grid needed
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        for pos in self.data.keys() {
            if pos.x < min_x {
                min_x = pos.x;
            } else if pos.x > max_x {
                max_x = pos.x;
            }
            if pos.y < min_y {
                min_y = pos.y;
            } else if pos.y > max_y {
                max_y = pos.y;
            }
        }
        let mut grid =
            vec![vec![default; 1 + (max_x - min_x) as usize]; 1 + (max_y - min_y) as usize];
        // 2nd pass: Build the grid
        for (pos, value) in &self.data {
            grid[(max_y - pos.y) as usize][(pos.x - min_x) as usize] = *value;
        }
        grid
    }

    /// Prints the grid to the console
    pub fn print(&self, default: V)
    where
        V: Display,
    {
        for row in self.to_vec(default) {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}
