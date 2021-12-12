use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Cave<'a> {
    connections: Vec<CaveId>,
    name: &'a str,
}

impl<'a> Cave<'a> {
    fn new(name: &'a str, connects_to: CaveId) -> Self {
        Cave {
            name,
            connections: vec![connects_to; 1],
        }
    }
}

type CaveId = u32;

/// Maximum id used for small caves, if a cave's id is greater than this then it is a big cave
static MAX_SMALL_CAVE_ID: CaveId = 32;

/// Generates the id for a cave
///
/// Ids are allocated from 1 to 32 for small caves and 33+ for big caves
fn gen_id(name: &str, last_small_id: &mut CaveId, last_big_id: &mut CaveId) -> CaveId {
    if is_small_cave(name) {
        *last_small_id += 1;
        *last_small_id
    } else {
        *last_big_id += 1;
        *last_big_id
    }
}

/// Checks if a cave name is a small cave
fn is_small_cave(name: &str) -> bool {
    name.chars().all(char::is_lowercase)
}

/// Generates a `HashMap` of ids -> Caves
fn gen(input: &str) -> HashMap<CaveId, Cave> {
    let mut caves: HashMap<CaveId, Cave> = HashMap::new();
    let mut cave_ids: HashMap<&str, CaveId> = HashMap::new();
    let mut last_small_id = 0;
    let mut last_big_id = MAX_SMALL_CAVE_ID;
    for (from, to) in input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
    {
        // Loop up id for each cave
        let from_id = *cave_ids
            .entry(from)
            .or_insert_with(|| gen_id(from, &mut last_small_id, &mut last_big_id));
        let to_id = *cave_ids
            .entry(to)
            .or_insert_with(|| gen_id(to, &mut last_small_id, &mut last_big_id));

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
    assert!(last_small_id <= MAX_SMALL_CAVE_ID);
    assert!(last_big_id > MAX_SMALL_CAVE_ID);
    caves
}

/// Counts the number of distinct paths from `start` --> `end`
///
/// For each path, big caves can be visited multiple times and small caves can be visited once
/// Exception being is that if `extra_visit` is true, then a single small cave can be visited twice
fn count_paths(input: &HashMap<CaveId, Cave>, extra_visit: bool) -> usize {
    // First stage - eliminate all big caves and remove connections back to the start
    let (&start_id, _) = input.iter().find(|(_, cave)| cave.name == "start").unwrap();
    let (&end_id, _) = input.iter().find(|(_, cave)| cave.name == "end").unwrap();
    let mut simplified = input.clone();
    for id in input.keys().filter(|&&id| id > MAX_SMALL_CAVE_ID) {
        // For all big caves
        let remove = simplified.get(id).unwrap();
        let connections = &remove.connections.clone();
        for connection in connections {
            if let Some(cave) = simplified.get_mut(connection) {
                // For every cave that it's connected to replace it with connections to all other connected caves
                // NOTE: This will include adding a loop back connection which is needed for part two so that we can
                // explore a small cave twice
                cave.connections.extend(connections);
                // Remove connection to the cave we are removing and the start
                cave.connections.retain(|x| x != id && x != &start_id);
            }
        }
        // Can now remove it
        simplified.remove(id);
    }

    // Second stage - transform into an array of cave id -> connections
    let mut caves = vec![Vec::new(); MAX_SMALL_CAVE_ID as usize];
    for (id, cave) in simplified {
        caves[id as usize] = cave.connections;
    }

    // Third stage - now we can finally count the paths
    let mut queue = VecDeque::new();
    let mut paths = 0;
    queue.push_back((start_id, 0, !extra_visit));
    while let Some((id, visited, used_extra_visit)) = queue.pop_front() {
        // Consider all the places we could move to from this cave
        for &next_id in &caves[id as usize] {
            if next_id == end_id {
                // Found a complete path
                paths += 1;
                continue;
            }
            // See if we have been here yet
            // Track which caves we have visited by setting a bit based on the cave id
            if visited & (1 << next_id) == 0 {
                // Haven't visited this small cave before, move into it recording that
                // we have visited it
                queue.push_back((next_id, visited | 1 << next_id, used_extra_visit));
            } else if !used_extra_visit {
                // Have visited this small cave before, but haven't used our extra visit yet
                // Use it up and move into it
                queue.push_back((next_id, visited, true));
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
