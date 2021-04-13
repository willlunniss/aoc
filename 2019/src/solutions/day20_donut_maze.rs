use std::collections::{HashMap, HashSet, VecDeque};
use utils::grid::{Direction, Pos, VecGrid};

fn discover_portal(map: &VecGrid<char>, pos: Pos, value: char) -> Option<(Pos, String, bool)> {
    for (d1, d2) in &[
        (Direction::Right, Direction::Left),
        (Direction::Down, Direction::Up),
    ] {
        // Get the next position (either right or down)
        let next = pos.next(*d1);
        if let Some(value1) = map.get(next) {
            if matches!(value1, 'A'..='Z') {
                // Found a new portal
                let portal = format!("{}{}", value, value1);
                // Work out if it's an inner or outer one
                let outer = (pos.x < 3 || pos.x >= (map.width() as isize - 3))
                    || (pos.y < 3 || pos.y >= (map.height() as isize - 3));
                if let Some('.') = map.get(pos.next(*d2)) {
                    // With entrance at the first position
                    return Some((pos, portal, outer));
                }
                // With entrance at the second position
                return Some((next, portal, outer));
            }
        }
    }
    // Already found from a different point of reference
    None
}

fn solve_maze(map: &VecGrid<char>, recursive: bool) -> usize {
    // Find all the portals and create links between positions
    let mut portals: HashMap<Pos, String> = HashMap::new();
    let mut links: HashMap<String, Vec<(Pos, bool)>> = HashMap::new();
    for (pos, value) in map {
        if matches!(value, 'A'..='Z') {
            // Looks like a portal
            if let Some((pos, portal, outer)) = discover_portal(map, pos, *value) {
                // Yes it is (and not one we have already)
                // Record info
                let link = links.entry(portal.clone()).or_default();
                link.push((pos, outer));
                portals.insert(pos, portal);
            }
        }
    }

    // Start at AA
    let (start, _) = links.get("AA").unwrap().first().unwrap();
    // Where to search next
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0, 0));
    // Guard set to stop us searching the same place twice
    let mut explored = HashSet::new();
    // Perform a BFS to find the shortest path from AA to ZZ
    while !queue.is_empty() {
        // Get the next position to explore
        let (pos, distance, level) = queue.pop_front().unwrap();
        explored.insert((level, pos));
        for (_, next, value) in map.neighbours_ex(pos) {
            // Get all neighbours
            if explored.contains(&(level, next)) {
                continue; // Already explored (so can't be a shorter route)
            }
            let value = value.unwrap();
            if value == '.' {
                // Normal passage, queue up
                queue.push_back((next, distance + 1, level));
                continue;
            }
            if let Some(portal) = portals.get(&next) {
                // Walking through a portal, teleport to other end
                if portal == "ZZ" {
                    // Exit is only valid on level 0, otherwise treat as a wall
                    if level == 0 {
                        // At the exit on the right level
                        // Return the distance (subtract 1 to account for starting in AA rather than next to it)
                        return distance - 1;
                    }
                    continue;
                } else if portal == "AA" {
                    // Can't go back through AA, treat as a wall
                    continue;
                }
                // Find where the portal goes to
                let target = links
                    .get(portal)
                    .unwrap()
                    .iter()
                    .find(|p| p.0 != next)
                    .unwrap();
                // Work out what level we will go to after going through the portal
                let new_level = if !recursive {
                    // Not recursing, stay on a single level
                    level
                } else if target.1 {
                    // Coming out at an outer portal, so going further in via an inner
                    level + 1
                } else {
                    // Coming out at an inner portal, so coming back out via an outer
                    level - 1
                };
                // Check that we aren't trying to go past the outer most level
                if new_level >= 0 {
                    // Teleport without increasing distance and push to front of the queue for processing next
                    queue.push_front((target.0, distance, new_level));
                }
            }
        }
    }
    0
}

#[aoc_generator(day20)]
fn gen(input: &str) -> VecGrid<char> {
    VecGrid::from(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day20, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    solve_maze(input, false)
}

#[aoc(day20, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    solve_maze(input, true)
}
