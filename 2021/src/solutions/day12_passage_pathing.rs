use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Cave<'a> {
    connections: Vec<u32>,
    name: &'a str,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str, connects_to: u32) -> Self {
        Cave {
            name,
            connections: vec![connects_to; 1],
        }
    }
}

/// Maximum id used for small caves
/// If a cave's id is greater than this then it is a big cave
static MAX_SMALL_CAVE_ID: u32 = 1 << 15;

/// Generates the id for a cave
fn gen_id(name: &str, last_small_index: &mut u32, last_big_index: &mut u32) -> u32 {
    let index = if is_small_cave(name) {
        *last_small_index += 1;
        last_small_index
    } else {
        *last_big_index -= 1;
        last_big_index
    };
    1 << *index
}

/// Generates a `HashMap` of ids -> Caves
///
/// Ids are allocated using bit fields
/// shifting up from 1 for small caves up to 1<< 15
/// shifting down from 32 for big caves down to 1 << 16
/// This works because there is a small enough number of caves
/// that they can each take up 1 bit
fn gen(input: &str) -> HashMap<u32, Cave> {
    let mut caves: HashMap<u32, Cave> = HashMap::new();
    let mut cave_ids: HashMap<&str, u32> = HashMap::new();
    let mut small_index = 0;
    let mut big_index = 32;
    for (from, to) in input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
    {
        // Loop up id for each cave
        let from_id = *cave_ids
            .entry(from)
            .or_insert_with(|| gen_id(from, &mut small_index, &mut big_index));
        let to_id = *cave_ids
            .entry(to)
            .or_insert_with(|| gen_id(to, &mut small_index, &mut big_index));

        // For each each connected cave record that we can go between them
        caves
            .entry(from_id)
            .and_modify(|x| x.connections.push(to_id))
            .or_insert_with(|| Cave::new(from, to_id));
        caves
            .entry(to_id)
            .and_modify(|x| x.connections.push(from_id))
            .or_insert_with(|| Cave::new(to, from_id));
    }
    // Sanity check that we haven't tried to allocate to many ids
    assert!(1 << small_index <= MAX_SMALL_CAVE_ID);
    assert!(1 << big_index > MAX_SMALL_CAVE_ID);
    caves
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(char::is_lowercase)
}

/// Counts the number of distinct paths from `start` --> `end`
///
/// For each path, big caves can be visited multiple times and small caves can be visited once
/// Exception being is that if `extra_visit` is true, then a single small cave can
/// be visited twice
fn count_paths(caves: &HashMap<u32, Cave>, extra_visit: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut paths = 0;
    let (start_id, _) = caves.iter().find(|(_, cave)| cave.name == "start").unwrap();
    let (end_id, _) = caves.iter().find(|(_, cave)| cave.name == "end").unwrap();
    queue.push_back((start_id, 0, !extra_visit));
    while let Some((id, visited_small, used_extra_visit)) = queue.pop_front() {
        // Consider all the places we could move to from this cave
        let cave = caves.get(id).unwrap();
        for next_id in &cave.connections {
            if next_id == start_id {
                // don't go back to the start
                continue;
            }
            if next_id == end_id {
                // Found a complete path
                paths += 1;
                continue;
            }
            // Haven't been here yet, for this path
            if *next_id > MAX_SMALL_CAVE_ID {
                // Can visit big caves multiple times, move into it
                queue.push_back((next_id, visited_small, used_extra_visit));
            } else if visited_small & next_id == 0 {
                // Haven't visited this small cave before, move into it recording that
                // we have visited it
                queue.push_back((next_id, visited_small | next_id, used_extra_visit));
            } else if !used_extra_visit {
                // Have visited this small cave before, but haven't used our extra visit yet
                // Use it up and move into it
                queue.push_back((next_id, visited_small, true));
            }
        }
    }
    paths
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    // Count all paths, only visiting small caves once per path
    count_paths(&gen(input), false)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    // Count all paths, allowing a single extra visit to a small cave per path
    count_paths(&gen(input), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 10);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 36);
    }
}
