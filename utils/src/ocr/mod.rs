use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OcrStringError {
    #[error("invalid font height (expected 6 or 10, found {height})")]
    InvalidHeight { height: usize },
}

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

#[derive(Debug)]
pub struct OcrString {
    grouped: Vec<HashSet<Point>>,
    height: usize,
}

impl FromIterator<(isize, isize)> for OcrString {
    fn from_iter<I: IntoIterator<Item = (isize, isize)>>(iter: I) -> Self {
        OcrString::new_without_bounds(iter.into_iter().map(|(x, y)| (x as usize, y as usize)))
            .unwrap()
    }
}

impl FromIterator<Point> for OcrString {
    fn from_iter<I: IntoIterator<Item = Point>>(iter: I) -> Self {
        OcrString::new_without_bounds(iter.into_iter()).unwrap()
    }
}

impl TryFrom<Vec<Vec<char>>> for OcrString {
    type Error = OcrStringError;

    /// Builds an `OcrString` from a `Vec` of row `Vec` of `char` where '#' is used to draw the letters
    fn try_from(vec: Vec<Vec<char>>) -> Result<Self, Self::Error> {
        let height = vec.len();
        let points = vec.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'#')
                .map(move |(x, _)| (x, y))
        });
        OcrString::new(points, (0, 0), height)
    }
}

impl FromStr for OcrString {
    type Err = OcrStringError;

    /// Builds an `OcrString` from ASCII art string where '#' is used to draw the letters
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let points = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| (x, y))
        });
        OcrString::new(points, (0, 0), height)
    }
}

impl OcrString {
    /// Creates a new `OcrString` from Points with a known top left corner `origin` and font `height`
    fn new(
        points: impl Iterator<Item = Point>,
        origin: Point,
        height: usize,
    ) -> Result<Self, OcrStringError> {
        let (min_x, min_y) = origin;
        let split = match height {
            6 => 5,
            10 => 8,
            _ => {
                return Err(OcrStringError::InvalidHeight { height });
            }
        };
        // Group the points - shifting their coordinates as needed
        let mut grouped = Vec::new();
        for (x_orig, y_orig) in points {
            // First shift to give an effective origin of (0, 0)
            let (x, y) = (x_orig - min_x, y_orig - min_y);
            // Then group by the char they are part of
            let group = x / split;
            while group >= grouped.len() {
                grouped.push(HashSet::new());
            }
            grouped[group].insert((x % split, y));
        }

        Ok(Self { grouped, height })
    }

    /// Creates a new `OcrString` from Points without any prior knowledge about where they are and what font is used
    fn new_without_bounds(points: impl Iterator<Item = Point>) -> Result<Self, OcrStringError> {
        // First work out where the chars start (in terms of the top left of the first char) and how heigh they are
        let points = points.collect::<Vec<_>>();
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_y = 0;
        for &(x, y) in points.iter() {
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            } else if y > max_y {
                max_y = y;
            }
        }
        let height = max_y - min_y + 1;
        // Now create a new `OcrString`
        OcrString::new(points.into_iter(), (min_x, min_y), height)
    }

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
        println!("OcrString: Font height: {}", self.height);
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
            let mut grid = vec![vec![' '; 6]; self.height];
            for &(x, y) in group {
                grid[y][x] = '#';
            }
            for row in &grid {
                println!("{}", row.iter().collect::<String>());
            }
        }
    }
}

impl fmt::Display for OcrString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decode().unwrap())
    }
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
    fn test_6x4_str1() {
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
        assert_eq!(ocr.height, 6);
        assert_eq!(ocr.decode(), Some("ABKJFBGC".to_owned()));
    }
    #[test]
    fn test_6x4_str2() {
        static SAMPLE: &str = indoc! {"
        ####.####.####.#...##..#.####.###..####..###...##.
        #....#....#....#...##.#..#....#..#.#......#.....#.
        ###..###..###...#.#.##...###..#..#.###....#.....#.
        #....#....#......#..#.#..#....###..#......#.....#.
        #....#....#......#..#.#..#....#.#..#......#..#..#.
        ####.#....####...#..#..#.#....#..#.#.....###..##..
        "};
        let ocr: OcrString = SAMPLE.parse().unwrap();
        assert_eq!(ocr.len(), 10);
        assert_eq!(ocr.height, 6);
        assert_eq!(ocr.decode(), Some("EFEYKFRFIJ".to_owned()));
    }

    #[test]
    fn test_invalid_str() {
        static SAMPLE: &str = indoc! {"
        .##..###..#..#...##.####.###...##...##.
        #..#.#..#.#.#.....#.#....#..#.#..#.#..#
        "};
        assert_eq!(
            SAMPLE.parse::<OcrString>().err(),
            Some(OcrStringError::InvalidHeight { height: 2 })
        );
    }

    #[test]
    fn test_6x4_points() {
        static SAMPLE: &str = indoc! {"
        .............................................
        .............................................
        .............................................
        ....##..###..#..#...##.####.###...##...##....
        ...#..#.#..#.#.#.....#.#....#..#.#..#.#..#...
        ...#..#.###..##......#.###..###..#....#......
        ...####.#..#.#.#.....#.#....#..#.#.##.#......
        ...#..#.#..#.#.#..#..#.#....#..#.#..#.#..#...
        ...#..#.###..#..#..##..#....###...###..##....
        .............................................
        .............................................
        .............................................
        "};
        let ocr: OcrString = SAMPLE
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| (x, y))
            })
            .collect();
        assert_eq!(ocr.len(), 8);
        assert_eq!(ocr.height, 6);
        assert_eq!(ocr.decode(), Some("ABKJFBGC".to_owned()));
    }

    #[test]
    fn test_10x6_str() {
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
        assert_eq!(ocr.height, 10);
        assert_eq!(ocr.decode(), Some("XECXBPZB".to_owned()));
    }
}
