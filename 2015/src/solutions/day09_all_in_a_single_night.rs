use itertools::Itertools;
use pathfinding::prelude::*;
use std::collections::HashMap;

// Assumption: no more than 32 locations
type Location = u32;
type Distance = usize;

#[derive(Debug)]
struct Route {
    start: Location,
    end: Location,
    distance: Distance,
}

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<Route> {
    let mut locations = HashMap::new();
    let mut next_id = 0;
    input
        .lines()
        .map(move |line| {
            let (start, _, end, _, distance) =
                line.split_ascii_whitespace().collect_tuple().unwrap();
            Route {
                start: *locations.entry(start).or_insert_with(|| {
                    next_id += 1;
                    1 << (next_id - 1)
                }),
                end: *locations.entry(end).or_insert_with(|| {
                    next_id += 1;
                    1 << (next_id - 1)
                }),
                distance: distance.parse().unwrap(),
            }
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    location: Location,
    visited: Location,
}

impl State {
    const fn move_to(&self, location: Location) -> Self {
        Self {
            location,
            visited: self.visited | location,
        }
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Route]) -> usize {
    let mut distances: HashMap<Location, HashMap<Location, Distance>> = HashMap::new();

    // For all routes (A-B), add distances for A->B and B->A
    for route in input {
        distances
            .entry(route.start)
            .or_default()
            .insert(route.end, route.distance);
        distances
            .entry(route.end)
            .or_default()
            .insert(route.start, route.distance);
    }

    // Completed when we have visited all locations
    let target = distances.keys().sum();
    let places = distances.len();
    for i in 0..places {
        // Add distance of 0 from the virtual start location to each real location
        distances.entry(0).or_default().insert(1 << i, 0);
    }

    // Compute shortest path to visit all locations
    dijkstra(
        &State {
            location: 0,
            visited: 0,
        },
        |state| {
            distances
                .get(&state.location)
                .unwrap()
                .iter()
                .filter(|(id, _dist)| state.visited & **id == 0)
                .map(|(id, dist)| (state.move_to(*id), *dist))
                .collect::<Vec<_>>()
        },
        |state| state.visited == target,
    )
    .unwrap()
    .1
}

#[aoc(day9, part2)]
fn part2(input: &[Route]) -> usize {
    let mut distances: HashMap<Location, HashMap<Location, Distance>> = HashMap::new();

    // For all routes (A-B), add distances for A->B and B->A
    for route in input {
        distances
            .entry(route.start)
            .or_default()
            .insert(route.end, route.distance);
        distances
            .entry(route.end)
            .or_default()
            .insert(route.start, route.distance);
    }

    // Consider all possible permutations of paths through all locations
    // calculating what the total distance would be
    // and then selecting the maximum
    distances
        .keys()
        .permutations(distances.len())
        .map(|path| {
            path.iter()
                .tuple_windows()
                .map(|(from, to)| distances[from][to])
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    London to Dublin = 464
    London to Belfast = 518
    Dublin to Belfast = 141
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 605);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 982);
    }
}
