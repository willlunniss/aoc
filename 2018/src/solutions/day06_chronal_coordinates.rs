use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

/// Calculates the Manhattan distance between two points
const fn manhattan_distance(from: (isize, isize), to: (isize, isize)) -> isize {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| {
            // Split "123, 456" into two and parse as numbers
            let coordinates: (&str, &str) = line.splitn(2, ", ").collect_tuple().unwrap();
            (
                coordinates.0.parse().unwrap(),
                coordinates.1.parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[(isize, isize)]) -> usize {
    // Find boundaries of the area to check
    let min_x = input.iter().min_by_key(|pos| pos.0).unwrap().0;
    let min_y = input.iter().min_by_key(|pos| pos.1).unwrap().1;
    let max_x = input.iter().max_by_key(|pos| pos.0).unwrap().0;
    let max_y = input.iter().max_by_key(|pos| pos.1).unwrap().1;
    let mut areas = HashMap::new();
    let mut infinite = HashSet::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // For each (x, y) inside the area we care about
            let mut min = isize::MAX;
            let mut closest = 0;
            let mut count = 0;
            // Find which input coordinate has the largest area closest to it by assigning them an
            // id and incrementing a counter for how many positions they are closest to based on the
            // manhattan distance
            for (id, dist) in input
                .iter()
                .enumerate()
                .map(|(id, coordinates)| (id, manhattan_distance((y, x), *coordinates)))
            {
                match dist.cmp(&min) {
                    Ordering::Less => {
                        // Found a new closest id
                        min = dist;
                        closest = id;
                        count = 0;
                    }
                    Ordering::Equal => {
                        // Joint tie for closest, if we don't find a new minimum then this
                        // won't be allocated
                        count += 1;
                    }
                    Ordering::Greater => {}
                }
            }
            if x == min_x || x == max_x || y == min_y || y == max_y {
                // Outside the bounds of what we want to check, note it down so we can filter it out
                infinite.insert(closest);
            } else if count == 0 {
                // This position was closest to exactly one id
                *areas.entry(closest).or_insert(0) += 1;
            } // Else there was a joint tie, don't allocate to any
        }
    }
    // Find the area with the most points closest to it that isn't in out
    // set that would spread out infinitely if not contained
    *areas
        .iter()
        .filter_map(|(id, count)| {
            if infinite.contains(id) {
                None
            } else {
                Some(count)
            }
        })
        .max_by_key(|count| **count)
        .unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &[(isize, isize)]) -> usize {
    // Find boundaries of the area to check
    let min_x = input.iter().min_by_key(|pos| pos.0).unwrap().0;
    let min_y = input.iter().min_by_key(|pos| pos.1).unwrap().1;
    let max_x = input.iter().max_by_key(|pos| pos.0).unwrap().0;
    let max_y = input.iter().max_by_key(|pos| pos.1).unwrap().1;
    let mut count = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // For each (x, y) inside the area we care about
            // Increment a counter if this position's total distance to all
            // input coordinates is less than 1000
            if input
                .iter()
                .map(|coordinates| manhattan_distance((y, x), *coordinates))
                .sum::<isize>()
                < 10_000
            {
                count += 1;
            }
        }
    }
    count
}
