use crate::grid::pos::Pos;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

/// Grid that uses a `HashMap` to store data of a unknown and non-fixed size
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

    /// Creates a new `MapGrid` from an iterator
    pub fn from_iter(iter: impl IntoIterator<Item = (Pos, V)>) -> Self {
        Self {
            data: HashMap::from_iter(iter),
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Pos, V> {
        self.data.iter()
    }

    /// An iterator visiting all keys in arbitrary order.
    pub fn keys(&self) -> std::collections::hash_map::Keys<Pos, V> {
        self.data.keys()
    }

    /// An iterator visiting all values in arbitrary order.
    pub fn values(&self) -> std::collections::hash_map::Values<Pos, V> {
        self.data.values()
    }

    /// Inserts the element into the supplied position
    pub fn insert(&mut self, pos: Pos, value: V) {
        self.data.insert(pos, value);
    }

    /// Returns a reference to the value at the position
    pub fn get(&self, pos: &Pos) -> Option<&V> {
        self.data.get(pos)
    }

    /// Returns a mutable reference to the value at the position
    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut V> {
        self.data.get_mut(pos)
    }

    /// Gets the given position's corresponding entry in the map for in-place manipulation
    pub fn entry(&mut self, key: Pos) -> std::collections::hash_map::Entry<Pos, V> {
        self.data.entry(key)
    }

    /// Converts a `HashMap` based grid to a nested vector
    ///
    /// Any cells within the bounds of those specified in map that don't have values
    /// by be set to default
    pub fn to_vec(&self, default: V) -> Vec<Vec<V>> {
        // 1st pass: Find the size of the grid needed
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
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
