use crate::grid::{Pos, VecGrid};
use itertools::Itertools;
use lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::{convert::Infallible, str::FromStr};

type CharHash = u32;

const ALPHABET_6_4: [(char, &str); 18] = [
    ('A', ".##.\n#..#\n#..#\n####\n#..#\n#..#"),
    ('B', "###.\n#..#\n###.\n#..#\n#..#\n###."),
    ('C', ".##.\n#..#\n#...\n#...\n#..#\n.##."),
    ('E', "####\n#...\n###.\n#...\n#...\n####"),
    ('F', "####\n#...\n###.\n#...\n#...\n#..."),
    ('G', ".##.\n#..#\n#...\n#.##\n#..#\n.###"),
    ('H', "#..#\n#..#\n####\n#..#\n#..#\n#..#"),
    ('I', ".###\n..#.\n..#.\n..#.\n..#.\n.###"),
    ('J', "..##\n...#\n...#\n...#\n#..#\n.##."),
    ('K', "#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#"),
    ('L', "#...\n#...\n#...\n#...\n#...\n####"),
    ('O', ".##.\n#..#\n#..#\n#..#\n#..#\n.##."),
    ('P', "###.\n#..#\n#..#\n###.\n#...\n#..."),
    ('R', "###.\n#..#\n#..#\n###.\n#.#.\n#..#"),
    ('S', ".###\n#...\n#...\n.##.\n...#\n###."),
    ('U', "#..#\n#..#\n#..#\n#..#\n#..#\n.##."),
    ('Y', "#...\n#...\n.#.#\n..#.\n..#.\n..#."),
    ('Z', "####\n...#\n..#.\n.#..\n#...\n####"),
];

lazy_static! {
    static ref CHAR_LOOKUP: HashMap<CharHash, char> = {
        // Pre-compute the hashes for all known letters
        let mut m = HashMap::new();
        for (letter, s) in &ALPHABET_6_4 {
            let code = hash_char(s.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x, _)| Pos::new(x, y))));
            assert!(m.insert(code, *letter).is_none());
        }
        m
    };
}

pub struct OcrString {
    points: Vec<Vec<Pos>>,
}

impl FromIterator<Pos> for OcrString {
    fn from_iter<I: IntoIterator<Item = Pos>>(iter: I) -> Self {
        Self {
            points: group_points(iter.into_iter()),
        }
    }
}

impl FromIterator<(isize, isize)> for OcrString {
    fn from_iter<I: IntoIterator<Item = (isize, isize)>>(iter: I) -> Self {
        Self {
            points: group_points(iter.into_iter().map(|pos| Pos::from(pos))),
        }
    }
}

impl FromIterator<(usize, usize)> for OcrString {
    fn from_iter<I: IntoIterator<Item = (usize, usize)>>(iter: I) -> Self {
        Self {
            points: group_points(iter.into_iter().map(|pos| Pos::from(pos))),
        }
    }
}

impl FromStr for OcrString {
    type Err = Infallible;

    /// Builds an `OcrString` from ASCII art string where '#' is used to draw the letters
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Pos::new(x, y))
            })
            .collect())
    }
}

impl OcrString {
    #[must_use]
    /// Decodes the `OcrString` into a `String`
    pub fn decode(&self) -> Option<String> {
        // For each char group, workout what char it is
        let mut result: Vec<char> = Vec::new();
        for group in self.points.iter() {
            let code = hash_char(group.iter().map(|&pos| pos));
            result.push(*CHAR_LOOKUP.get(&code)?);
        }
        if result.len() != self.len() {
            return None;
        }
        Some(result.iter().collect())
    }

    /// Gets the number of chars that make up this `OcrString`
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn debug_print(&self) {
        for (i, group) in self.points.iter().enumerate() {
            println!("Char {}: {:?}", i, group);
            let mut grid = VecGrid::new_sized(' ', 4, 6);
            for &point in group {
                grid[point] = '#';
            }
            grid.print();
        }
    }
}

/// Groups points into to per-char groups
fn group_points(points: impl Iterator<Item = Pos>) -> Vec<Vec<Pos>> {
    let mut max_group = 0;
    let mut grouped_points = vec![Vec::new(); 16];
    for (group, grouped) in &points.group_by(|point| (point.x / 5) as usize) {
        grouped_points[group].extend(grouped.map(|pos| Pos::from((pos.x % 5, pos.y))));
        if group > max_group {
            max_group = group;
        }
    }
    return grouped_points.drain(0..=max_group).collect();
}

/// Generates a uniq hash value for each char in the alphabet
fn hash_char(points: impl Iterator<Item = Pos>) -> CharHash {
    points.fold(0, |hash, pos| {
        hash + (pos.y + 1) * (pos.x + 1) * (pos.x + (pos.y * 4))
    }) as CharHash
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_generated_hashes() {
        static SAMPLE: &str = indoc! {"
        .##..###..#..#...##.####.###...##...##.
        #..#.#..#.#.#.....#.#....#..#.#..#.#..#
        #..#.###..##......#.###..###..#....#...
        ####.#..#.#.#.....#.#....#..#.#.##.#...
        #..#.#..#.#.#..#..#.#....#..#.#..#.#..#
        #..#.###..#..#..##..#....###...###..##.
        "};
        let ocr: OcrString = SAMPLE.parse().unwrap();
        assert_eq!(ocr.len(), 8);
        assert_eq!(ocr.decode(), Some("ABKJFBGC".to_owned()));
    }
}
