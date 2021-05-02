use std::collections::HashMap;
use std::convert::TryInto;

type Point = [isize; 4];

fn manhattan_distance(a: &Point, b: &Point) -> usize {
    (0..4)
        .map(|index| (a[index] - b[index]).abs() as usize)
        .sum()
}

#[aoc_generator(day25)]
fn gen(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect::<Vec<isize>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[aoc(day25, part1)]
fn part1(input: &[Point]) -> usize {
    let mut constellations: HashMap<usize, Vec<Point>> = HashMap::new();
    let mut next_id = 0;
    for point in input {
        // For each point, see which constellations it connects to (may be more than 1!)
        let connects_to_ids = constellations
            .iter()
            .filter(|(_, constellation)| {
                constellation
                    .iter()
                    .any(|existing| manhattan_distance(point, existing) <= 3)
            })
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();
        match connects_to_ids.len() {
            0 => {
                // No matches, create a new constellation
                constellations.insert(next_id, vec![*point; 1]);
                next_id += 1;
            }
            1 => {
                // Single match, add to it
                let id = connects_to_ids[0];
                constellations.get_mut(&id).unwrap().push(*point);
            }
            _ => {
                // More than one match (adding this point joins up multiple constellations)
                // Add to the first match
                let id = connects_to_ids[0];
                constellations.get_mut(&id).unwrap().push(*point);
                for merge in connects_to_ids.iter().skip(1) {
                    // Then merge in the points from the others
                    let merged_points = constellations.remove(merge).unwrap();
                    constellations.get_mut(&id).unwrap().extend(merged_points);
                }
            }
        }
    }
    // Return the number of constellations
    constellations.len()
}
