use crate::grid::Pos;
use itertools::Itertools;
use lazy_static;
use std::collections::HashMap;

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

/// Decodes an ASCII art string where '#' is used to draw the letters
pub fn decode(ascii_art: &str) -> Option<String> {
    decode_points(ascii_art.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(move |(x, _)| Pos::new(x, y))
    }))
}

/// Decodes a Iterator of points
pub fn decode_points(points: impl Iterator<Item = Pos>) -> Option<String> {
    // Group points into separate per-char groups
    let mut grouped_points = vec![Vec::new(); 16];
    for (group, grouped) in &points.group_by(|point| point.x / 5) {
        grouped_points[group as usize].extend(grouped);
    }

    // For each char group, workout what char it is
    let mut result: Vec<char> = Vec::new();
    for (char_index, grouped) in grouped_points
        .iter()
        .enumerate()
        .filter(|(_, x)| !x.is_empty())
    {
        let code = hash_char(grouped.iter().map(|&pos| pos - (5 * char_index, 0)));
        result.push(*CHAR_LOOKUP.get(&code)?);
    }
    if result.is_empty() {
        return None;
    }
    Some(result.iter().collect())
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
        assert_eq!(decode(SAMPLE), Some("ABKJFBGC".to_owned()));
    }
}
