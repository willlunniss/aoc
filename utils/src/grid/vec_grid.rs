use crate::grid::direction::Direction;
use crate::grid::pos::Pos;
use std::fmt::Display;

/// Grid that uses nested vectors to store data of a known and fixed size
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

    pub fn insert(&mut self, pos: Pos, value: V) {
        self.data[pos.y as usize][pos.x as usize] = value;
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