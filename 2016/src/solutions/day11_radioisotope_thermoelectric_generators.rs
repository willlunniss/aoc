use itertools::Itertools;
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum Part {
    Generator(char),
    Microchip(char),
}

type Floors = [HashSet<Part>; 4];

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    floors: Floors,
    floor: usize,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.floor.hash(state);
        for floor in &self.floors {
            floor.iter().sorted().collect::<Vec<_>>().hash(state);
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.floor.cmp(&other.floor)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn safe(&self, floor: usize) -> bool {
        let generators = self.floors[floor]
            .iter()
            .filter_map(|p| match *p {
                Part::Generator(isotope) => Some(isotope),
                _ => None,
            })
            .collect::<Vec<_>>();
        // Check if there are any generators
        if generators.is_empty() {
            return true;
        }
        // There is a generator, check that all chips have matching generators
        self.floors[floor]
            .iter()
            .filter_map(|p| match *p {
                Part::Microchip(isotope) => Some(isotope),
                _ => None,
            })
            .all(|chip| generators.contains(&chip))
    }

    /// Generates a rough score of how
    fn score(&self) -> usize {
        self.floors
            .iter()
            .map(|parts| parts.len())
            .fold(0, |acc, x| (acc * 10) + x)
    }

    fn successors(&self) -> Vec<(Self, usize)> {
        let mut valid = Vec::new();
        // Consider all potential states from here which involve moving 1 or 2 items
        // from the current floor up/down by 1
        let target_floors = if self.floor == 0 {
            vec![1]
        } else if self.floor == 3 {
            vec![2]
        } else {
            vec![self.floor + 1, self.floor - 1]
        };

        for parts in self.floors[self.floor]
            .iter()
            .combinations(1)
            .chain(self.floors[self.floor].iter().combinations(2))
        {
            for floor in &target_floors {
                let mut candidate = self.clone();
                // Remove the parts from the current floor
                for part in &parts {
                    candidate.floors[self.floor].remove(*part);
                }
                // Check the current floor's state is valid if removed
                if !candidate.safe(self.floor) {
                    break;
                }

                // Add parts to the target floor
                for part in &parts {
                    candidate.floors[*floor].insert(**part);
                }
                // Check target floor's state is valid if added
                if !candidate.safe(*floor) {
                    continue;
                }
                candidate.floor = *floor;
                valid.push(candidate);
            }
        }
        valid.into_iter().map(|p| (p, 1)).collect()
    }

    /// Returns true if all items are on the fourth floor (i.e. all other floors are empty)
    fn all_on_fourth_floor(&self) -> bool {
        (0..=2).all(|floor| self.floors[floor].is_empty())
    }
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Floors {
    let mut floors: Floors = vec![HashSet::new(); 4].try_into().unwrap();
    for line in input.lines() {
        let sections = line.split(" a ").collect::<Vec<_>>();
        if sections.len() > 1 {
            let floor = match sections[0].splitn(3, ' ').nth(1).unwrap() {
                "first" => 0,
                "second" => 1,
                "third" => 2,
                "fourth" => 3,
                _ => unreachable!(),
            };
            for part in sections.iter().skip(1) {
                let isotope = part.chars().next().unwrap();
                if part.contains("generator") {
                    floors[floor].insert(Part::Generator(isotope));
                } else {
                    floors[floor].insert(Part::Microchip(isotope));
                }
            }
        }
    }
    floors
}

#[aoc(day11, part1)]
fn part1(input: &Floors) -> usize {
    let start = State {
        floors: input.clone(),
        floor: 0,
    };
    astar(
        &start,
        State::successors,
        State::score,
        State::all_on_fourth_floor,
    )
    .unwrap()
    .1
}

#[aoc(day11, part2)]
fn part2(input: &Floors) -> usize {
    let mut floors = input.clone();
    for isotope in ['e', 'd'] {
        floors[0].insert(Part::Generator(isotope));
        floors[0].insert(Part::Microchip(isotope));
    }
    let start = State { floors, floor: 0 };
    astar(
        &start,
        State::successors,
        State::score,
        State::all_on_fourth_floor,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
    The second floor contains a hydrogen generator.
    The third floor contains a lithium generator.
    The fourth floor contains nothing relevant.
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 11);
    }
}
