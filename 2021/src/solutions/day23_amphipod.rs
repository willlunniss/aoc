use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
use utils::grid::{Pos, VecGrid};

fn requires_energy(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn targets(amphipod: char) -> (Pos, Pos, Pos, Pos) {
    let x = match amphipod {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => unreachable!(),
    };
    (
        Pos::new(x, 5),
        Pos::new(x, 4),
        Pos::new(x, 3),
        Pos::new(x, 2),
    )
}

fn is_target(
    amphipod: char,
    pos: &Pos,
    spaces: &HashSet<Pos>,
    amphipods: &HashMap<Pos, char>,
) -> bool {
    let (t1, t2, t3, t4) = targets(amphipod);
    if *pos == t1 {
        true
    } else if *pos == t2 {
        !spaces.contains(&t1) && !amphipods.contains_key(&t1)
    } else if *pos == t3 {
        !spaces.contains(&t2) && !amphipods.contains_key(&t2)
    } else if *pos == t4 {
        !spaces.contains(&t3) && !amphipods.contains_key(&t3)
    } else {
        false
    }
}

const fn in_hallway(pos: &Pos) -> bool {
    pos.y == 1
}

const fn can_stop(pos: &Pos) -> bool {
    pos.y != 1 || !matches!(pos.x, 3 | 5 | 7 | 9)
}

const fn is_amphipod(c: char) -> bool {
    matches!(c, 'A' | 'B' | 'C' | 'D')
}

fn can_move_to(
    amphipod: char,
    start: &Pos,
    spaces: &HashSet<Pos>,
    amphipods: &HashMap<Pos, char>,
) -> Vec<(Pos, usize)> {
    let mut candidates = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((*start, 0));
    while let Some((pos, moves)) = queue.pop_front() {
        for candidate in pos.neighbours().filter(|pos| spaces.contains(pos)) {
            if let Entry::Vacant(e) = candidates.entry(candidate) {
                e.insert(moves + 1);
                queue.push_back((candidate, moves + 1));
            }
        }
    }
    return candidates
        .iter()
        .filter(|(pos, _)| {
            can_stop(pos)
                && ((in_hallway(pos) && !in_hallway(start))
                    || is_target(amphipod, pos, spaces, amphipods))
        })
        .map(|(k, v)| (*k, *v))
        .collect();
}

fn debug(spaces: &HashSet<Pos>, amphipods: &HashMap<Pos, char>, energy: usize) {
    let mut grid = VecGrid::new_sized('#', 14, 7);
    for pos in spaces {
        grid[*pos] = '.';
    }
    for (pos, amphipod) in amphipods {
        grid[*pos] = *amphipod;
    }
    println!("Cost {}", energy);
    grid.print();
}

#[derive(Clone)]
struct State {
    spaces: HashSet<Pos>,
    amphipods: HashMap<Pos, char>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.amphipods
            .iter()
            .sorted()
            .collect::<Vec<_>>()
            .hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.amphipods
            .iter()
            .sorted()
            .eq(other.amphipods.iter().sorted())
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.amphipods.len().cmp(&other.amphipods.len())
    }
}

impl State {
    const fn new(spaces: HashSet<Pos>, amphipods: HashMap<Pos, char>) -> Self {
        Self { spaces, amphipods }
    }

    fn organised(&self) -> bool {
        self.amphipods.is_empty()
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("remaining", &self.amphipods.len())
            .finish()
    }
}

fn organise_amphipods(input: &VecGrid<char>) -> usize {
    let spaces = input
        .into_iter()
        .filter(|(_, &x)| x == '.')
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>();
    let mut amphipods = input
        .into_iter()
        .filter(|(_, &x)| is_amphipod(x))
        .map(|(pos, &a)| (pos, a))
        .collect::<HashMap<Pos, char>>();
    let in_target = amphipods
        .iter()
        .filter(|(pos, &amphipod)| is_target(amphipod, pos, &spaces, &amphipods))
        .map(|(pos, _)| *pos)
        .collect::<Vec<_>>();
    for pos in in_target {
        amphipods.remove(&pos);
    }
    let initial = State::new(spaces.clone(), amphipods.clone());
    dijkstra(
        &initial,
        |state| {
            let mut next = Vec::new();
            // Consider all amphipods
            for (pos, amphipod) in &state.amphipods {
                let candidates = can_move_to(*amphipod, pos, &state.spaces, &state.amphipods);
                if candidates.is_empty() {
                    continue;
                }
                // Check where they could go to
                for (candidate, moves) in candidates {
                    let mut amphipods = state.amphipods.clone();
                    let mut spaces = state.spaces.clone();
                    // Move to the new state
                    amphipods.remove(pos);
                    if !is_target(*amphipod, &candidate, &spaces, &state.amphipods) {
                        amphipods.insert(candidate, *amphipod);
                    }
                    // Free up the space where we were and remove where we have gone to
                    spaces.insert(*pos);
                    spaces.remove(&candidate);
                    let energy = moves * requires_energy(*amphipod);
                    next.push((State::new(spaces, amphipods), energy));
                }
            }
            next
        },
        State::organised,
    )
    .unwrap()
    .1
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    organise_amphipods(&input.parse().unwrap())
}

static P2_INSERT: [&str; 2] = ["  #D#C#B#A#", "  #D#B#A#C#"];

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.splice(3..3, P2_INSERT);
    organise_amphipods(&lines.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    #############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 12521);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 44169);
    }
}
