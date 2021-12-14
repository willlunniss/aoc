use itertools::Itertools;
use lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::{convert::Infallible, str::FromStr};

type CharHash = u32;
pub type Point = (usize, usize);

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
            let code = hash_char(s.lines().enumerate().flat_map(|(y, line)| line.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x, _)| (x, y))));
            assert!(m.insert(code, *letter).is_none());
        }
        m
    };
}

pub struct OcrString {
    points: Vec<Vec<Point>>,
}

impl FromIterator<(isize, isize)> for OcrString {
    fn from_iter<I: IntoIterator<Item = (isize, isize)>>(iter: I) -> Self {
        Self {
            points: group_points(iter.into_iter().map(|(x, y)| (x as usize, y as usize))),
        }
    }
}

impl FromIterator<Point> for OcrString {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        Self {
            points: group_points(iter.into_iter()),
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
                    .map(move |(x, _)| (x, y))
            })
            .collect())
    }
}

impl fmt::Display for OcrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decode().unwrap())
    }
}

impl OcrString {
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
            let code = hash_char(group.iter().map(|&pos| pos));
            println!(
                "{} = {}: {:?}",
                i,
                CHAR_LOOKUP.get(&code).unwrap_or(&'?'),
                group
            );
            let mut grid = vec![vec![' '; 4]; 6];
            for &(x, y) in group {
                grid[y][x] = '#';
            }
            for row in &grid {
                println!("{}", row.iter().collect::<String>());
            }
        }
    }
}

/// Groups points into to per-char groups
fn group_points(points: impl Iterator<Item = Point>) -> Vec<Vec<Point>> {
    let mut max_group = 0;
    let mut grouped_points = vec![Vec::new(); 16];
    for (group, grouped) in &points.group_by(|(x, _y)| (x / 5) as usize) {
        grouped_points[group].extend(grouped.map(|(x, y)| (x % 5, y)));
        if group > max_group {
            max_group = group;
        }
    }
    return grouped_points.drain(0..=max_group).collect();
}

/// Generates a uniq hash value for each char in the alphabet
fn hash_char(points: impl Iterator<Item = Point>) -> CharHash {
    points.fold(0, |hash, (x, y)| hash + (y + 1) * (x + 1) * (x + (y * 4))) as CharHash
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
