use std::collections::HashMap;
use std::fmt::Display;

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

    pub const fn all() -> [Self; 4] {
        use Direction::{Down, Left, Right, Up};
        [Up, Right, Down, Left]
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

/// Grid that uses nested vectors to store data
#[derive(Clone)]
pub struct VecGrid<V> {
    data: Vec<Vec<V>>,
}

/// Iterator over (position, value) tuples
impl<'a, V: Clone + Copy> IntoIterator for &'a VecGrid<V> {
    type Item = (Pos, &'a V);
    type IntoIter = VecGridIterator<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        VecGridIterator {
            grid: self,
            pos: Pos::new(0, 0),
        }
    }
}

pub struct VecGridIterator<'a, V> {
    grid: &'a VecGrid<V>,
    pos: Pos,
}

impl<'a, V: Clone + Copy> Iterator for VecGridIterator<'a, V> {
    type Item = (Pos, &'a V);
    fn next(&mut self) -> Option<(Pos, &'a V)> {
        if self.grid.contains(self.pos) {
            // Get the value
            let result = Some((self.pos, self.grid.get_ref_no_check(self.pos)));
            // Work out where next
            if self.pos.x < self.grid.width() as isize - 1 {
                self.pos.x += 1;
            } else {
                self.pos.x = 0;
                self.pos.y += 1;
            }
            result
        } else {
            None
        }
    }
}

impl<V: Clone + Copy> Default for VecGrid<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Copy> VecGrid<V> {
    /// Gets the element at the supplied position or None if it is outside of bounds
    pub fn get(&self, pos: Pos) -> Option<V> {
        if self.contains(pos) {
            Some(self.data[pos.y as usize][pos.x as usize])
        } else {
            None // Outside of bounds
        }
    }

    pub fn values(&self) -> Vec<V> {
        self.data
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect()
    }

    /// Gets the values of the 4 neighbours to the supplied position
    pub fn neighbours(&self, pos: Pos) -> Vec<Option<V>> {
        use Direction::{Down, Left, Right, Up};
        [Up, Right, Down, Left]
            .iter()
            .map(|d| self.get(pos.next(*d)))
            .collect()
    }

    /// Gets the direction, pos and values of the 4 neighbours to the supplied position
    pub fn neighbours_ex(&self, pos: Pos) -> Vec<(Direction, Pos, Option<V>)> {
        use Direction::{Down, Left, Right, Up};
        [Up, Right, Down, Left]
            .iter()
            .map(|d| {
                let next = pos.next(*d);
                (*d, next, self.get(next))
            })
            .collect()
    }
}

impl<V> VecGrid<V> {
    /// Creates a new empty `VecGrid`
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a new `VecGrid` from an existing nested vector
    pub fn from(data: Vec<Vec<V>>) -> Self {
        Self { data }
    }

    /// Gets the width of the grid
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    /// Gets the height of the grid
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Sets the value at the specified position
    pub fn set(&mut self, pos: Pos, value: V) {
        self.data[pos.y as usize][pos.x as usize] = value;
    }

    /// Checks whether the supplied position exists within the grid
    pub fn contains(&self, pos: Pos) -> bool {
        pos.y >= 0 && pos.y < self.height() as isize && pos.x >= 0 && pos.x < self.width() as isize
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

    /// Gets as a reference to the value at the supplied position without
    /// performing any bounds checking before hand
    fn get_ref_no_check(&self, pos: Pos) -> &V {
        &self.data[pos.y as usize][pos.x as usize]
    }
}

/// Grid that uses a `HashMap` to store data
///
/// Especially useful where you don't know the full size in advance
#[derive(Clone, Debug)]
pub struct MapGrid<V> {
    data: HashMap<Pos, V>,
}

impl<V: Clone> Default for MapGrid<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone> MapGrid<V> {
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
    pub fn get(&self, pos: &Pos) -> Option<&V> {
        self.data.get(pos)
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut V> {
        self.data.get_mut(pos)
    }

    pub fn entry(&mut self, key: Pos) -> std::collections::hash_map::Entry<Pos, V> {
        self.data.entry(key)
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
            grid[(pos.y - min_y) as usize][(pos.x - min_x) as usize] = value.clone();
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
