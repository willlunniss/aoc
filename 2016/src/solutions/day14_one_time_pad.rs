use itertools::Itertools;
use std::collections::HashMap;

/// Computes a normal hash (MD5)
fn hash(salt: &str, index: usize) -> String {
    format!("{:x}", md5::compute(format!("{salt}{index}")))
}

/// Computes a stretched hash (by re-hashing 2016 times)
fn stretched_hash(salt: &str, index: usize) -> String {
    (0..=2016).fold(format!("{salt}{index}"), |acc, _| {
        format!("{:x}", md5::compute(acc))
    })
}

/// Returns the index that results in the key number
fn find_key_index(
    salt: &str,
    key_number: usize,
    hasher: fn(&str, usize) -> String,
) -> Option<usize> {
    let mut keys = 0;
    let mut candidates: HashMap<char, Vec<usize>> = HashMap::new();
    for index in 0.. {
        // Compute the hash using the supplied hasher function
        let hash = hasher(salt, index);

        // Could make a candidate a key if it has 5 consecutive matching chars
        if let Some(quintuplet) = hash
            .chars()
            .tuple_windows()
            .find(|(a, b, c, d, e)| [a, b, c, d, e].windows(2).all(|w| w[0] == w[1]))
        {
            if let Some(candidate) = candidates.get(&quintuplet.0) {
                // Candidate must have be generated in within the last 1000 indexes
                for original_index in candidate {
                    if index - *original_index <= 1000 {
                        keys += 1;
                        if keys == key_number {
                            // Have found the set number of keys
                            return Some(*original_index);
                        }
                    }
                }
                candidates.remove(&quintuplet.0);
            }
        }
        // Is new key candidate if it has 3 consecutive matching chars
        if let Some(triplet) = hash
            .chars()
            .tuple_windows()
            .find(|(a, b, c)| a == b && b == c)
        {
            candidates.entry(triplet.0).or_default().push(index);
        }
    }
    None
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    find_key_index(input, 64, hash).unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    find_key_index(input, 64, stretched_hash).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("abc"), 22728);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("abc"), 22551);
    }
}
