use itertools::Itertools;
use std::convert::TryFrom;
use utils::grid::{Direction, MapGrid, Pos};

#[derive(PartialEq)]
enum State {
    Wall,
    Edge,
    AlreadyOverflowed,
}

/// Simulates water flowing downwards through the ground until `max_y`
///
/// Returns the total number of squares the water can reach
fn flow(map: &mut MapGrid<char>, from: &Pos, max_y: isize) -> usize {
    // Keep heading downwards until we hit clay (or max_y)
    let mut water = 0;
    let mut pos = *from;
    while map.get(&pos).is_none() {
        if pos.y > max_y {
            // Reached the bottom of the map
            return water;
        }
        map.insert(pos, '|');
        pos = pos.next(Direction::Down);
        water += 1;
    }
    if map.get(&pos).unwrap() == &'|' {
        // Hit existing overflowing bucket, can't fill up any higher
        water
    } else {
        // Hit either clay or some non-overflowed water, fill up
        fill(map, &pos.next(Direction::Up), max_y) + water
    }
}

/// Simulates filling up a clay bucket
///
/// Returns the total number of squares the water can reach
fn fill(map: &mut MapGrid<char>, from: &Pos, max_y: isize) -> usize {
    // Head outwards until we go over the edge and flow down
    // If there are clay edges on both side, move up a level and repeat
    let mut water = 0;
    let mut pos = *from;
    loop {
        let mut boundaries = Vec::new();
        'explore_out: for &direction in &[Direction::Left, Direction::Right] {
            let mut next = pos;
            while map.get(&next.next(Direction::Down)).is_some() {
                let current = next;
                if map.insert(current, '|').is_none() {
                    // If there wasn't already water here, increment counter
                    water += 1;
                }
                next = current.next(direction);
                match map.get(&next) {
                    Some(&'#') => {
                        // Hit an wall, now try in the other direction
                        boundaries.push((current, State::Wall));
                        continue 'explore_out;
                    }
                    Some(&'|') => {
                        if let Some(&'|') = map.get(&next.next(Direction::Down)) {
                            // Already overflowed here below
                            boundaries.push((current, State::AlreadyOverflowed));
                            continue 'explore_out;
                        }
                    }
                    None | Some(_) => {}
                }
            }
            // Didn't find a wall, overflowed over the edge
            boundaries.push((next, State::Edge));
        }
        if boundaries.iter().all(|(_, state)| *state == State::Wall) {
            // Found a wall on both sides so filled up this level
            // Mark the water as at rest '~'
            let range = boundaries.first().unwrap().0.x..=boundaries.last().unwrap().0.x;
            range.for_each(|x| {
                map.insert(Pos::from((x, pos.y)), '~');
            });
            // Now move up
            pos = pos.next(Direction::Up);
        } else {
            // Overflowed on one or more sides
            // Return water that we have filled this bucket with + what we get from overflowing over each edge
            return water
                + boundaries
                    .iter()
                    .map(|(edge, state)| {
                        if *state == State::Edge {
                            flow(map, edge, max_y)
                        } else {
                            0
                        }
                    })
                    .sum::<usize>();
        }
    }
}

#[aoc_generator(day17)]
fn gen(input: &str) -> MapGrid<char> {
    input
        .lines()
        .flat_map(|line| {
            // Turn x=495, y=2..7 or y=7, x=495..501 into a Vec of points where there is clay
            // Get the parts e.g. for x=495, y=2..7 we get (x, 495, y, 2, 7)
            let (fixed_axis, fixed_value, _range_axis, range_start, range_end) = line
                .split(&['=', ',', ' ', '.'][..])
                .filter(|p| !p.is_empty())
                .collect_tuple()
                .unwrap();
            let range = range_start.parse().unwrap()..=range_end.parse().unwrap();
            if fixed_axis == "x" {
                let x: usize = fixed_value.parse().unwrap();
                range.map(|y| Pos::new(x, y)).collect::<Vec<Pos>>()
            } else {
                let y: usize = fixed_value.parse().unwrap();
                range.map(|x| Pos::new(x, y)).collect::<Vec<Pos>>()
            }
        })
        .map(move |pos| (pos, '#')) // Represent each point as # for clay
        .collect()
}

#[aoc(day17, part1)]
fn part1(input: &MapGrid<char>) -> usize {
    let min_y = input.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_y = input.keys().max_by_key(|pos| pos.y).unwrap().y;
    let mut map = input.clone();
    // Simulate the water flowing from the spring and return the total number of
    // tiles the water can reach between min and max y
    flow(&mut map, &Pos::new(500, 0), max_y) - usize::try_from(min_y).unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &MapGrid<char>) -> usize {
    let max_y = input.keys().max_by_key(|pos| pos.y).unwrap().y;
    let mut map = input.clone();
    // Simulate the water flowing from the spring then return the total number of
    // tiles where the water sits at rest
    flow(&mut map, &Pos::new(500, 0), max_y);
    map.values().filter(|value| **value == '~').count()
}
