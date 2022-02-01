use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse in 'aaaaa-bbb-z-y-x-123[abxyz]' into aaaaa-bbb-z-y-x   123   abxyz
        let (encrypted_name, details) = s.rsplit_once('-').unwrap();
        let (sector_id, checksum, _) = details.split(&['[', ']'][..]).collect_tuple().unwrap();
        Ok(Self {
            encrypted_name: encrypted_name.to_owned(),
            sector_id: sector_id.parse().unwrap(),
            checksum: checksum.to_owned(),
        })
    }
}

impl Room {
    /// Determines if the room is real by validating it's checksum
    fn is_real(&self) -> bool {
        // Calculate the frequency of all letters
        // Sort from most common to least, ties broken by alphabetization
        // Then compare against the checksum
        self.encrypted_name
            .chars()
            .filter(char::is_ascii_alphabetic)
            .counts()
            .iter()
            .sorted_by(|a, b| {
                let count = b.1.cmp(a.1);
                if count == Ordering::Equal {
                    a.0.cmp(b.0)
                } else {
                    count
                }
            })
            .map(|(e, _)| e)
            .zip(self.checksum.chars())
            .all(|(e, c)| *e == c)
    }

    /// Returns the decrypted room name
    fn decrypt(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                _ => rotate(c, self.sector_id),
            })
            .collect()
    }
}

/// Rotates a letter through the alphabet cycles number of times
const fn rotate(c: char, cycles: u32) -> char {
    ((((c as u32 - 'a' as u32) + cycles) % 26) + 'a' as u32) as u8 as char
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<Room> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Room]) -> u32 {
    // Sum the ids of all real rooms
    input
        .iter()
        .filter(|r| r.is_real())
        .map(|r| r.sector_id)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Room]) -> u32 {
    // For all real rooms, decrypt the names and then return the id of the one
    // with the north pole objects
    input
        .iter()
        .filter(|r| r.is_real())
        .map(|r| (r.decrypt(), r.sector_id))
        .find_map(|(name, id)| name.contains("northpole").then(|| id))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    aaaaa-bbb-z-y-x-123[abxyz]
    a-b-c-d-e-f-g-h-987[abcde]
    not-a-real-room-404[oarel]
    totally-real-room-200[decoy]
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 1514);
    }
}
