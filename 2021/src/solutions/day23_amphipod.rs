use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt;
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

fn is_target(amphipod: char, pos: &Pos, spaces: &HashSet<Pos>) -> bool {
    let (t1, t2, t3, t4) = targets(amphipod);
    *pos == t1
        || { *pos == t2 && !spaces.contains(&t1) }
        || { *pos == t3 && !spaces.contains(&t2) }
        || { *pos == t4 && !spaces.contains(&t3) }
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
        for candidate in pos
            .neighbours()
            .filter(|pos| spaces.contains(pos) && !amphipods.contains_key(pos))
        {
            if let Entry::Vacant(e) = candidates.entry(candidate) {
                e.insert(moves + 1);
                queue.push_back((candidate, moves + 1));
            }
        }
    }
    if let Some((pos, cost)) = candidates
        .iter()
        .find(|(pos, _)| is_target(amphipod, pos, spaces))
    {
        // If we can move directly to out target then always do that
        return vec![(*pos, *cost); 1];
    }
    return candidates
        .iter()
        .filter(|(pos, _)| can_stop(pos) && (in_hallway(pos) && !in_hallway(start)))
        .map(|(k, v)| (*k, *v))
        .collect();
}

fn debug(spaces: &HashSet<Pos>, amphipods: &HashMap<Pos, char>, energy: usize) {
    let mut grid = VecGrid::new_sized('#', 14, 5);
    for pos in spaces {
        grid[*pos] = '.';
    }
    for (pos, amphipod) in amphipods {
        grid[*pos] = *amphipod;
    }
    println!("Cost {}", energy);
    grid.print();
}

struct State {
    energy: usize,
    spaces: HashSet<Pos>,
    amphipods: HashMap<Pos, char>,
}

impl State {
    const fn new(spaces: HashSet<Pos>, amphipods: HashMap<Pos, char>, energy: usize) -> Self {
        Self {
            energy,
            spaces,
            amphipods,
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.energy == other.energy
            && self.spaces == other.spaces
            && self.amphipods == other.amphipods
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
        let remaining = other.amphipods.len().cmp(&self.amphipods.len());
        let energy = other.energy.cmp(&self.energy);
        if energy == Ordering::Equal {
            remaining
        } else {
            energy
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("energy", &self.energy)
            .field("remaining", &self.amphipods.len())
            .finish()
    }
}

fn organise(input: &VecGrid<char>) -> usize {
    let spaces = input
        .into_iter()
        .filter(|(_, &x)| x == '.' || is_amphipod(x))
        .map(|(pos, _)| pos)
        .collect::<HashSet<Pos>>();
    let amphipods = input
        .into_iter()
        .filter(|(_, &x)| is_amphipod(x))
        .map(|(pos, &a)| (pos, a))
        .collect::<HashMap<Pos, char>>();

    // Search for the moves that require the lowest energy to organise
    let mut results = Vec::new();
    let mut queue = BinaryHeap::new();
    queue.push(State::new(spaces.clone(), amphipods.clone(), 0));
    // Track which states we have exploded and at what energy cost
    let mut explored: HashMap<Vec<(Pos, char)>, usize> = HashMap::new();

    while !queue.is_empty() {
        // Get the next state to consider with the lowest energy cost so far (ties broken by least amphipods to move)
        let state = queue.pop().unwrap();

        // Consider all amphipods
        for (pos, &amphipod) in &state.amphipods {
            let candidates = can_move_to(amphipod, pos, &state.spaces, &state.amphipods);
            if candidates.is_empty() {
                continue;
            }
            // Check where they could go to
            for (candidate, moves) in candidates {
                let mut amphipods = state.amphipods.clone();
                let mut spaces = state.spaces.clone();
                // Move to the new state
                let energy = state.energy + (moves * requires_energy(amphipod));
                amphipods.remove(pos);
                if is_target(amphipod, &candidate, &spaces) {
                    // At the target so can stop having to consider this amphipod
                    spaces.remove(&candidate);
                } else {
                    amphipods.insert(candidate, amphipod);
                }
                if amphipods.is_empty() {
                    // Found a valid solution
                    results.push(energy);
                } else {
                    // Check to see if we have considered this state before
                    let best = explored
                        .entry(
                            amphipods
                                .iter()
                                .map(|(k, v)| (*k, *v))
                                .sorted()
                                .collect::<Vec<_>>(),
                        )
                        .or_insert(energy);
                    if *best >= energy {
                        queue.push(State::new(spaces, amphipods, energy));
                    } // else - have seen this state before at a lower energy so no point considering again
                }
            }
        }
    }

    *results.iter().min().unwrap()
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    organise(&input.parse().unwrap())
}

static P2_INSERT: [&str; 2] = ["  #D#C#B#A#", "  #D#B#A#C#"];

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.splice(3..3, P2_INSERT);
    organise(&lines.into())
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
