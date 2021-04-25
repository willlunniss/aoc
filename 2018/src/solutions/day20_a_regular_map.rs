use std::collections::{HashMap, HashSet, VecDeque};
use utils::grid::{Direction, MapGrid, Pos};

/// Builds a map of the building from the regex directions
fn build_map(input: &str) -> MapGrid<char> {
    let mut map = MapGrid::new();
    let mut branch_stack = Vec::new();
    // Start at the center in a room
    let mut pos = Pos::new(0, 0);
    map.insert(pos, 'X');
    for c in input.chars() {
        match c {
            'N' | 'S' | 'E' | 'W' => {
                // Move forward into the door
                let direction = Direction::from(c);
                let door = pos.next(direction);
                let door_type = match direction {
                    Direction::Up | Direction::Down => '-',
                    Direction::Left | Direction::Right => '|',
                };
                // Add the door and the walls either side to the map
                map.insert(door, door_type);
                map.insert(door.next(direction.rotate_left()), '#');
                map.insert(door.next(direction.rotate_right()), '#');
                // Now move through the door into the next room
                pos = door.next(direction);
                map.insert(pos, '.');
            }
            '(' => {
                // Start of a new branch
                branch_stack.push(pos);
            }
            '|' => {
                // Completed this option, go back to the start of the branch
                pos = *branch_stack.last().unwrap();
            }
            ')' => {
                // Completed this branch
                branch_stack.pop();
            }
            _ => {}
        }
    }
    map
}

/// Returns the number of doors that you need to go through to reach the further room
fn furthest_room(input: &mut impl Iterator<Item = char>) -> usize {
    let mut doors = 0;
    let mut max = 0;
    while let Some(c) = input.next() {
        match c {
            'N' | 'S' | 'E' | 'W' => {
                // Move forward through a door into a new room
                doors += 1;
            }
            '(' => {
                // Start of a new branch
                doors += furthest_room(input);
            }
            '|' => {
                // Completed this option, go back to the start of the branch
                // Update max doors for this branch
                if doors > max {
                    max = doors;
                }
                doors = 0;
            }
            ')' => {
                // End of a branch
                // Update max doors for this branch
                if doors > max {
                    max = doors;
                }
                return if doors == 0 {
                    // If the last option in this branch was 0, treat as a going off and then back again to where we were
                    0
                } else {
                    // Otherwise return max doors across all options in this branch
                    max
                };
            }
            _ => {}
        }
    }
    doors
}

/// Returns the distance to every room in the building
fn rooms_distances(map: &MapGrid<char>) -> HashMap<Pos, usize> {
    // Perform a BFS from the center outwards recording the shortest distances to each room
    let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
    let mut rooms = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    queue.push_back((Pos::new(0, 0), 0));
    while !queue.is_empty() {
        let (pos, distance) = queue.pop_front().unwrap();
        for (dir, next, value) in map.neighbours_ex(pos) {
            // Check to see if a we are next to a door we haven't gone through yet
            if (value == Some(&'-') || value == Some(&'|')) && visited.insert(next) {
                // Go through the door into the new room
                let room = next.next(dir);
                rooms.insert(room, distance + 1);
                queue.push_back((room, distance + 1));
            }
        }
    }
    rooms
}

#[aoc(day20, part1)]
fn part1(input: &str) -> usize {
    // Quickly read the input to find the furthest room
    furthest_room(&mut input.chars())
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    // Build a complete map from the input
    let map = build_map(input);
    // Use a BFS to find all the rooms and the distances to them
    let rooms = rooms_distances(&map);
    // Count the rooms that are at least 1000 from the start
    rooms.values().filter(|&&dist| dist >= 1_000).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_furthest_room() {
        assert_eq!(furthest_room(&mut "^WNE$".chars()), 3);
        assert_eq!(furthest_room(&mut "^ENWWW(NEEE|SSE(EE|N))$".chars()), 10);
        assert_eq!(
            furthest_room(&mut "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".chars()),
            18
        );
        assert_eq!(
            furthest_room(&mut "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$".chars()),
            23
        );
        assert_eq!(
            furthest_room(
                &mut "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$".chars()
            ),
            31
        );
    }
}
