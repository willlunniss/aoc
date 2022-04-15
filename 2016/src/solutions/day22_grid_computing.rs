use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::{num::ParseIntError, str::FromStr};
use utils::grid::{MapGrid, Pos};

#[derive(Debug, Clone, Copy)]
struct Node {
    pos: Pos,
    size: usize,
    used: usize,
    avail: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum NodeType {
    Empty,
    Interchangeable,
    Goal,
    Full,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    empty: Pos,
    goal: Pos,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_number(s: &str) -> Result<usize, ParseIntError> {
            s.chars()
                .filter(|x| x.is_digit(10))
                .collect::<String>()
                .parse()
        }

        if s.starts_with("/dev/") {
            let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
            // Extract the position of the node
            let name = parts[0].rsplit('/').next().unwrap().to_owned();
            let pos = name
                .split('-')
                .flat_map(extract_number)
                .collect_tuple::<(usize, usize)>()
                .unwrap()
                .into();
            // and the stats
            Ok(Self {
                pos,
                size: extract_number(parts[1]).unwrap(),
                used: extract_number(parts[2]).unwrap(),
                avail: extract_number(parts[3]).unwrap(),
            })
        } else {
            Err("Not a valid node".to_owned())
        }
    }
}

/// Represents the current state of moving data around the grid
impl State {
    /// Moves the goal node by swapping with the empty node
    const fn move_goal(self) -> Self {
        Self {
            empty: self.goal,
            goal: self.empty,
        }
    }

    /// Moves the empty node by swapping with an interchangeable node
    const fn move_empty(self, interchangeable: Pos) -> Self {
        Self {
            empty: interchangeable,
            goal: self.goal,
        }
    }
}

#[aoc_generator(day22)]
fn gen(input: &str) -> Vec<Node> {
    input.lines().flat_map(str::parse).collect()
}

#[aoc(day22, part1)]
fn part1(input: &[Node]) -> usize {
    // Count the number of pairs of nodes where [0] is not empty and can fit in [1]
    input
        .iter()
        .permutations(2)
        .filter(|x| x[0].used > 0 && x[0].used <= x[1].avail)
        .count()
}

#[aoc(day22, part2)]
fn part2(input: &[Node]) -> usize {
    // Find the size of the smallest node
    let min_size = input.iter().map(|n| n.size).min().unwrap();
    // Create a grid containing all nodes that could fit their data in the smallest node
    // Other nodes are considered full/not movable
    let grid: MapGrid<NodeType> = input
        .iter()
        .filter(|n| n.used <= min_size)
        .map(|n| (n.pos, NodeType::Interchangeable))
        .collect();

    // Work out the start state based on where the empty and goal nodes are
    let empty = input.iter().find(|node| node.used == 0).unwrap().pos;
    let goal = input
        .iter()
        .filter(|n| n.pos.y == 0)
        .max_by_key(|n| n.pos.x)
        .unwrap()
        .pos;
    let start_state = State { empty, goal };
    // Want to get the goal to the target by moving data around using the empty node
    let target = Pos::new(0, 0);
    dijkstra(
        &start_state,
        |state| {
            let mut candidates = Vec::new();
            // Can move the goal if it is next to the empty node
            if state.goal.neighbours().any(|n| n == state.empty) {
                candidates.push((state.move_goal(), 1));
            }
            // Can move the empty node into any interchangeable node
            for (_, pos, _) in grid
                .neighbours_ex(state.empty)
                .filter(|(_, _, n)| *n == Some(&NodeType::Interchangeable))
                .filter(|(_, pos, _)| *pos != state.goal)
            {
                candidates.push((state.move_empty(pos), 1));
            }
            candidates
        },
        |s| s.goal == target,
    )
    .expect("Failed to find a way to move the goal data to the target")
    .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    Filesystem            Size  Used  Avail  Use%
    /dev/grid/node-x0-y0   10T    8T     2T   80%
    /dev/grid/node-x0-y1   11T    6T     5T   54%
    /dev/grid/node-x0-y2   32T   28T     4T   87%
    /dev/grid/node-x1-y0    9T    7T     2T   77%
    /dev/grid/node-x1-y1    8T    0T     8T    0%
    /dev/grid/node-x1-y2   11T    7T     4T   63%
    /dev/grid/node-x2-y0   10T    6T     4T   60%
    /dev/grid/node-x2-y1    9T    8T     1T   88%
    /dev/grid/node-x2-y2    9T    6T     3T   66%
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 7);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 7);
    }
}
