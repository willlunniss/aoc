/// Computes the hash for the given secret key and number
fn hash(secret_key: &str, number: usize) -> [u8; 16] {
    md5::compute(format!("{secret_key}{number}")).into()
}

/// Returns an iterator with all possible hash values
fn hashes(secret_key: &str) -> impl Iterator<Item = [u8; 16]> + '_ {
    (0..).map(|n| hash(secret_key, n))
}

/// Checks to see if a hash starts with five leading zeros
const fn starts_with_five_zeros(values: &[u8; 16]) -> bool {
    // Each value contains two chars, test that the first 5 are 0
    values[0] == 0 && values[1] == 0 && (values[2] & 0xF0) == 0
}

/// Checks to see if a hash starts with size leading zeros
const fn starts_with_six_zeros(values: &[u8; 16]) -> bool {
    // Each value contains two chars, test that the first 6 are 0
    values[0] == 0 && values[1] == 0 && values[2] == 0
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    hashes(input)
        .enumerate()
        .find(|(_, hash)| starts_with_five_zeros(hash))
        .unwrap()
        .0
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    hashes(input)
        .enumerate()
        .find(|(_, hash)| starts_with_six_zeros(hash))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("abcdef"), 609_043);
    }
}
