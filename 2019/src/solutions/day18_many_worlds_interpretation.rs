use crate::utils::grid::{Direction, MapGrid, Pos, VecGrid};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Kind {
    Key(char),
    Door(char),
    Start,
    Wall,
    Path,
    Empty,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(key) => write!(f, "{}", key),
            Self::Door(door) => write!(f, "{}", door),
            Self::Path => write!(f, "."),
            Self::Wall => write!(f, "#"),
            Self::Start => write!(f, "@"),
            _ => write!(f, " "),
        }
    }
}

impl Kind {
    fn from(c: char) -> Self {
        match c {
            '@' => Self::Start,
            'A'..='Z' => Self::Door(c),
            'a'..='z' => Self::Key(c),
            '#' => Self::Wall,
            '.' => Self::Path,
            _ => {
                panic!()
            }
        }
    }
}

/// Represent each key as a single bit
const fn key_bit_value(key: char) -> usize {
    1 << (key as u8 as usize - 97)
}

/// Tests if the key exists in the supplied keys
const fn is_new_key(keys: usize, key: char) -> Option<usize> {
    let value = key_bit_value(key);
    if value & keys == 0 {
        Some(value)
    } else {
        None
    }
}

/// Tests if we have the key for this door in the supplied keys
fn is_unlocked(keys: usize, door: char) -> bool {
    key_bit_value(door.to_ascii_lowercase()) & keys != 0
}

#[derive(PartialEq, Debug, Clone)]
struct Node {
    pos: Pos,
    kind: Kind,
    edges: HashMap<Pos, usize>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl Node {
    fn new(pos: Pos, kind: Kind) -> Self {
        Self {
            kind,
            pos,
            edges: HashMap::new(),
        }
    }

    fn none() -> Self {
        Self {
            kind: Kind::Empty,
            pos: Pos::new(0, 0),
            edges: HashMap::new(),
        }
    }
}

/// Performs a BFS from the start point outwards returning a graph of all nodes stored in a map
fn explore(input: &VecGrid<char>, start: Pos) -> MapGrid<Node> {
    // Resulting map
    let mut map: MapGrid<Node> = MapGrid::new();
    // Where to search next
    let mut queue = VecDeque::new();
    // Guard set to stop us searching the same place from the same parent twice
    let mut explored = HashSet::new();

    // Handle start position first
    let kind = Kind::from(input.get(start).unwrap());
    map.insert(start, Node::new(start, kind));
    queue.push_back((start, kind, start, 0));
    explored.insert((start, start));

    // Keep searching until nothing left
    while !queue.is_empty() {
        // Get the next position and it's details
        let (pos, kind, mut parent, mut distance) = queue.pop_front().unwrap();
        if matches!(kind, Kind::Door(_) | Kind::Key(_)) {
            // We have found a door or key, link it to it's parent
            let parent_node = map.get_mut(&parent).unwrap();
            parent_node.edges.insert(pos, distance);

            let mut created_node = false;
            map.entry(pos).or_insert_with(|| {
                // First time we have found it, create the new node to insert
                created_node = true;
                Node::new(pos, kind)
            });
            if created_node {
                // We created a new node, update distance/parent for next search phase starting from here
                parent = pos;
                distance = 0;
            } else {
                // If we haven't create a node, then it means we have seen it before
                // Stop searching
                continue;
            }
        }
        // Search from here
        for next in Direction::all().iter().map(|d| pos.next(*d)) {
            if explored.insert((next, parent)) {
                // Find out what we have here
                let kind = Kind::from(input.get(next).unwrap());
                if kind != Kind::Wall {
                    // Not a wall, so add it
                    queue.push_back((next, kind, parent, distance + 1));
                }
            }
        }
    }

    map
}

/// Finds all reachable keys from the current position given the available keys
/// 
/// A key is reachable if we can get to it without having to pass through either a key
/// that we don't yet have or a door that we haven't yet unlocked
fn reachable(map: &MapGrid<Node>, start: &Pos, keys: usize) -> HashMap<Pos, (usize, usize)> {
    let mut nodes = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((map.get(start).unwrap(), 0));
    visited.insert(start);
    while !queue.is_empty() {
        let (node, distance) = queue.pop_front().unwrap();
        for (next_pos, next_distance) in &node.edges {
            if visited.insert(next_pos) {
                let next = map.get(next_pos).unwrap();
                match next.kind {
                    Kind::Door(door) => {
                        // See if we can go through it
                        if !is_unlocked(keys, door) {
                            // Don't have the key to get through this door yet, stop
                            continue;
                        }
                    }
                    Kind::Key(key) => {
                        if let Some(key) = is_new_key(keys, key) {
                            // println!("Can reach new key {:?}", next);
                            nodes.insert(*next_pos, (distance + next_distance, key));
                            continue;
                        }
                    }
                    _ => {}
                }
                // Else at a key we have or a door that's unlocked, so check to see where we can get to from here
                queue.push_back((next, distance + next_distance));
            }
        }
    }
    nodes
}

/// Calculates the shortest path from the start node to pick up all keys
fn shortest_path(
    cache: &mut HashMap<(Pos, usize), usize>,
    map: &MapGrid<Node>,
    pos: &Pos,
    keys: usize,
) -> usize {
    if let Some(min) = cache.get(&(*pos, keys)) {
        // Already calculated for this position with these keys, return directly
        return *min;
    }
    // Get a list of reachable nodes from this position with these keys
    let nodes = reachable(map, pos, keys);
    // Select the shortest path from here to collect all keys
    // This is done by recursively building up the shortest path from each node that we can get to
    // with the current keys until all are picked up (no where new that we can reach)
    let min = nodes
        .iter()
        .map(|(next, (dist, new_key))| dist + shortest_path(cache, map, next, keys | new_key))
        .min()
        .unwrap_or(0);
    // Store the result in the cache
    cache.insert((*pos, keys), min);
    // and then finally return it
    min
}

#[aoc_generator(day18)]
fn gen(input: &str) -> VecGrid<char> {
    VecGrid::from(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day18, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    // Find the start position
    let start = input
        .into_iter()
        .find_map(|(pos, value)| if *value == '@' { Some(pos) } else { None })
        .unwrap();
    // Explore the map and build a tree of routes
    // Assumption: There are no loops in the map and the multiple route options at the start are of equal length
    let map = explore(input, start);

    // Calculate the shortest path to pick up all keys from our start position
    shortest_path(&mut HashMap::new(), &map, &start, 0)
}

#[aoc(day18, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_bit_value() {
        let calculated = ('a'..='z')
            .collect::<Vec<char>>()
            .iter()
            .map(|key| key_bit_value(*key))
            .collect::<Vec<usize>>();
        let reference = (0..26)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| 2_usize.pow(*x))
            .collect::<Vec<usize>>();
        assert_eq!(calculated, reference);
        assert_eq!(calculated.iter().sum::<usize>(), 2_usize.pow(26) - 1);
    }

    #[test]
    fn test_sample_1() {
        let input = "#########\n#b.A.@.a#\n#########";
        assert_eq!(part1(&gen(input)), 8);
    }

    #[test]
    fn test_sample_2() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(part1(&gen(input)), 86);
    }

    #[test]
    fn test_sample_3() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(part1(&gen(input)), 132);
    }

    #[test]
    fn test_sample_4() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(part1(&gen(input)), 136);
    }

    #[test]
    fn test_sample_5() {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(part1(&gen(input)), 81);
    }
}
