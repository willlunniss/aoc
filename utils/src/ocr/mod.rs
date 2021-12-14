use itertools::Itertools;
use lazy_static;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::iter::FromIterator;
use std::{convert::Infallible, str::FromStr};

type CharHash = u32;
pub type Point = (usize, usize);

#[derive(Clone)]
enum Font {
    Mode6x4,
    Mode10x6,
}

const ALPHABETS: [(Font, &str, &str); 2] = [
    (Font::Mode6x4, "ABCEFGHIJKLOPRSUYZ", include_str!("6x4.txt")),
    (Font::Mode10x6, "ABCEFGHJKLNPRXZ", include_str!("10x6.txt")),
];

lazy_static! {
    static ref CHAR_LOOKUP: HashMap<CharHash, char> = {
        // Pre-compute the hashes for all known letters
        let mut m = HashMap::new();
        for (_font, chars, dictionary) in ALPHABETS {
            // Parse the dictionary which will group points
            let ocr = dictionary.parse::<OcrString>().unwrap();
            for (points, letter) in ocr.grouped.iter().zip(chars.chars()) {
                let code = hash_char(points.iter());
                assert!(m.insert(code, letter).is_none());
            }
        }
        m
    };
}

pub struct OcrString {
    grouped: Vec<HashSet<Point>>,
}

impl FromIterator<(isize, isize)> for OcrString {
    fn from_iter<I: IntoIterator<Item = (isize, isize)>>(iter: I) -> Self {
        Self {
            grouped: group_points(iter.into_iter().map(|(x, y)| (x as usize, y as usize))),
        }
    }
}

impl FromIterator<Point> for OcrString {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        Self {
            grouped: group_points(iter.into_iter()),
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
        for group in self.grouped.iter() {
            let code = hash_char(group.iter());
            result.push(*CHAR_LOOKUP.get(&code).unwrap_or(&'?'));
        }
        Some(result.iter().collect())
    }

    /// Gets the number of chars that make up this `OcrString`
    pub fn len(&self) -> usize {
        self.grouped.len()
    }

    pub fn debug_print(&self) {
        for (i, group) in self.grouped.iter().enumerate() {
            let code = hash_char(group.iter());
            println!(
                "{}: {} = {}: {} {:?}",
                i,
                code,
                CHAR_LOOKUP.get(&code).unwrap_or(&'?'),
                group.len(),
                group
            );
            let mut grid = vec![vec![' '; 6]; 10];
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
fn group_points(points: impl Iterator<Item = Point>) -> Vec<HashSet<Point>> {
    // FIXME: Do this in a lot fewer passes over the points
    // May just get rid of automatic detection and require 6x4 vs 10x6 to be specified
    // First phase - work out the font type
    let points = points.collect::<Vec<_>>();
    let (_, min_y) = points.iter().min_by_key(|(_w, h)| h).unwrap();
    let (min_x, _) = points.iter().min_by_key(|(w, _h)| w).unwrap();
    let (_, max_y) = points.iter().max_by_key(|(_w, h)| h).unwrap();
    let split = match max_y - min_y {
        5 => 5,
        9 => 8,
        height => panic!(
            "Unexpected font size - detected height as {} with min {:?}",
            height,
            (min_x, min_y),
        ),
    };
    // Second phase, group the points
    let mut max_group = 0;
    let mut grouped_points = vec![HashSet::new(); 26];
    for (group, grouped) in &points
        .iter()
        .group_by(|(x, _y)| ((x - min_x) / split) as usize)
    {
        grouped_points[group].extend(grouped.map(|(x, y)| ((x - min_x) % split, *y - min_y)));
        if group > max_group {
            max_group = group;
        }
    }
    return grouped_points.drain(0..=max_group).collect();
}

/// Generates a uniq hash value for each char in the alphabet
fn hash_char<'a>(points: impl Iterator<Item = &'a Point>) -> CharHash {
    points.fold(0, |hash, (x, y)| hash + ((y + 1) * (x + 1) * (x + (y * 4)))) as CharHash
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_6x4() {
        static SAMPLE: &str = indoc! {"
        .##..###..#..#...##.####.###...##...##.
        #..#.#..#.#.#.....#.#....#..#.#..#.#..#
        #..#.###..##......#.###..###..#....#...
        ####.#..#.#.#.....#.#....#..#.#.##.#...
        #..#.#..#.#.#..#..#.#....#..#.#..#.#..#
        #..#.###..#..#..##..#....###...###..##.
        "};
        let ocr: OcrString = SAMPLE.parse().unwrap();
        ocr.debug_print();
        assert_eq!(ocr.len(), 8);
        assert_eq!(ocr.decode(), Some("ABKJFBGC".to_owned()));
    }

    #[test]
    fn test_10x6() {
        static SAMPLE: &str = indoc! {"
        #....#..######...####...#....#..#####...#####...######..#####.
        #....#..#.......#....#..#....#..#....#..#....#.......#..#....#
        .#..#...#.......#........#..#...#....#..#....#.......#..#....#
        .#..#...#.......#........#..#...#....#..#....#......#...#....#
        ..##....#####...#.........##....#####...#####......#....#####
        ..##....#.......#.........##....#....#..#.........#.....#....#
        .#..#...#.......#........#..#...#....#..#........#......#....#
        .#..#...#.......#........#..#...#....#..#.......#.......#....#
        #....#..#.......#....#..#....#..#....#..#.......#.......#....#
        #....#..######...####...#....#..#####...#.......######..#####.
        "};
        let ocr: OcrString = SAMPLE.parse().unwrap();
        assert_eq!(ocr.len(), 8);
        assert_eq!(ocr.decode(), Some("XECXBPZB".to_owned()));
    }
}
