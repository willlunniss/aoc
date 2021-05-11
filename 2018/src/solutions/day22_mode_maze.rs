use itertools::Itertools;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BTreeSet, HashMap};
use std::convert::TryFrom;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use utils::grid::{MapGrid, Pos};

#[derive(Debug, EnumIter, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    /// Checks if the current `Tool` is usable in the `Region`
    fn usable(self, region: Region) -> bool {
        match self {
            Self::Torch => region != Region::Wet,
            Self::ClimbingGear => region != Region::Narrow,
            Self::Neither => region != Region::Rocky,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl Region {
    /// Returns the risk level for the region
    const fn risk_level(self) -> usize {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Rocky => write!(f, "."),
            Self::Wet => write!(f, "="),
            Self::Narrow => write!(f, "|"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Search {
    time: usize,
    pos: Pos,
    tool: Tool,
    region: Region,
}

impl PartialOrd for Search {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Search {
    /// Custom cmp impl to ensure that we search with lowest elapsed time first
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by time first
        let cmp = self.time.cmp(&other.time);
        if cmp == Ordering::Equal {
            // If time is equal, sort by position and tool
            (self.pos, self.tool).cmp(&(other.pos, other.tool))
        } else {
            cmp
        }
    }
}

/// Calculates the geologic index for the given `pos`, updating `map` if needed to contain it
fn geologic_index(map: &mut MapGrid<usize>, pos: &Pos, depth: usize, target: &Pos) -> usize {
    if let Some(value) = map.get(pos) {
        // Already calculated, return directly
        return *value;
    }
    // Calculate as per rules from <https://adventofcode.com/2018/day/22>
    let value = if (pos.x == 0 && pos.y == 0) || pos == target {
        0
    } else if pos.y == 0 {
        usize::try_from(pos.x).unwrap() * 16807
    } else if pos.x == 0 {
        usize::try_from(pos.y).unwrap() * 48271
    } else {
        erosion_level(map, &(*pos + (-1, 0)), depth, target)
            * erosion_level(map, &(*pos + (0, -1)), depth, target)
    };
    // Store result before returning it
    map.insert(*pos, value);
    value
}

/// Calculates the geologic index for a given `pos`
fn erosion_level(map: &mut MapGrid<usize>, pos: &Pos, depth: usize, target: &Pos) -> usize {
    (geologic_index(map, pos, depth, target) + depth) % 20183
}

/// Calculates the region type for a given `geologic_index`
fn region_type(geologic_index: usize, depth: usize) -> Region {
    match ((geologic_index + depth) % 20183) % 3 {
        0 => Region::Rocky,
        1 => Region::Wet,
        2 => Region::Narrow,
        _ => panic!("Unexpected region type"),
    }
}

#[aoc_generator(day22)]
fn gen(input: &str) -> (usize, Pos) {
    // Get the values of form 'key: value'
    let values = input
        .lines()
        .map(|line| line.splitn(2, ' ').nth(1).unwrap())
        .collect::<Vec<_>>();
    let depth = values[0].parse().unwrap();
    let coordinates: (isize, isize) = values[1]
        .splitn(2, ',')
        .map(|value| value.parse().unwrap())
        .collect_tuple()
        .unwrap();
    (depth, Pos::from(coordinates))
}

#[aoc(day22, part1)]
fn part1(input: &(usize, Pos)) -> usize {
    let (depth, target) = *input;
    // Calculate the geologic index for smallest rectangle including 0,0 and `target`
    let mut geologic_map = MapGrid::new();
    for x in 0..=target.x {
        for y in 0..=target.y {
            geologic_index(&mut geologic_map, &Pos::from((x, y)), depth, &target);
        }
    }
    // Convert into the region's type and then sum up the risk levels for the area
    geologic_map
        .iter()
        .map(|(&_, &index)| region_type(index, depth).risk_level())
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &(usize, Pos)) -> Option<usize> {
    let (depth, target) = *input;
    // Perform a BFS from the start outwards considering each (Pos, Tool) pair uniquely
    // As the cost is not fixed we have a custom cmp impl for `Search` to ensure that we
    // process the next search location with the lowest elapsed time first
    let mut geologic_map = MapGrid::new();
    let mut visited: HashMap<(Pos, Tool), usize> = HashMap::new();
    let mut queue: BTreeSet<Search> = BTreeSet::new();
    // Queue up the start position using the torch
    let start = Pos::new(0, 0);
    queue.insert(Search {
        time: 0,
        pos: start,
        tool: Tool::Torch,
        region: Region::Rocky,
    });
    visited.insert((start, Tool::Torch), 0);
    while !queue.is_empty() {
        // Get the entry from the queue with the lowest time spent so far (BTreeSet and our Search cmp impl take care of that for us)
        // (Not using queue.pop_first().unwrap() as it's experimental)
        let current = *queue.iter().next().unwrap();
        queue.remove(&current);
        if current.pos == target {
            // Reached the target, as we process the queue by time this is guaranteed to be the shortest
            return Some(current.time);
        }
        // Check neighbours to see where we could move to
        for candidate in current.pos.neighbours() {
            if candidate.x < 0 || candidate.y < 0 {
                // The regions with negative X or Y are solid rock and cannot be traversed
                continue;
            }
            if candidate == target {
                // Found the target
                // If we need don't already have the torch we need to equip it
                let cost = if current.tool == Tool::Torch { 0 } else { 7 };
                // Add it to the queue (in case we just got here without the torch and are about
                // to reach from a different route with it already equipped)
                queue.insert(Search {
                    time: current.time + 1 + cost,
                    pos: candidate,
                    tool: Tool::Torch,
                    region: Region::Rocky,
                });
                continue;
            }
            // Calculate the region for this candidate
            let region = region_type(
                geologic_index(&mut geologic_map, &candidate, depth, &target),
                depth,
            );
            // Check which tools we can use in the current and new region
            // Tool must be valid for both as otherwise we won't be able to switch
            for tool in Tool::iter().filter(|t| t.usable(region) && t.usable(current.region)) {
                let time = current.time + 1 + if current.tool == tool { 0 } else { 7 };
                if let Some(&previous_best) = visited.get(&(candidate, tool)) {
                    // Already been here before with this equipment
                    if previous_best <= time {
                        // And was faster so stop this route
                        continue;
                    }
                }
                // Queue up searching the new position
                visited.insert((candidate, tool), time);
                queue.insert(Search {
                    time,
                    pos: candidate,
                    tool,
                    region,
                });
            }
        }
    }
    None
}
