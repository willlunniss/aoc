use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::collections::VecDeque;
use utils::grid::{Pos, VecGrid};

#[aoc_generator(day9)]
fn gen(input: &str) -> VecGrid<u8> {
    // Read in the grid, treating each char as a u8
    VecGrid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| char::to_digit(c, 10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

/// Finds all of the low points defined by having a value that is less than each
/// of their neighbours
fn find_low_points(grid: &VecGrid<u8>) -> impl Iterator<Item = (Pos, &u8)> + '_ {
    grid.into_iter()
        .filter(|(pos, &value)| grid.neighbours(*pos).flatten().all(|x| value < x))
}

#[aoc(day9, part1)]
fn part1(input: &VecGrid<u8>) -> u32 {
    // Risk value is low point value + 1 summed for all low points
    find_low_points(input)
        .map(|(_, &value)| value as u32 + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &VecGrid<u8>) -> u32 {
    let low_points = find_low_points(input).map(|(pos, _)| pos);

    // Track basin sizes in a binary heap to make it easy to get the 3 biggest at the end
    let mut basin_sizes = BinaryHeap::new();
    // Track where we have explored so we don't count somewhere twice
    let mut explored = HashSet::new();
    for start in low_points {
        // For each low point, see how big the basin is
        let mut size = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(next) = queue.pop_front() {
            if !explored.insert(next) {
                continue; // already explored - skip
            }
            // New place - increment basin size
            size += 1;
            for (_, candidate_pos, candidate_value) in input.neighbours_ex(next) {
                if let Some(value) = candidate_value {
                    if value == 9 {
                        continue; // reached a peak
                    }
                    // New place in the basin, add to queue
                    queue.push_back(candidate_pos);
                }
            }
        }
        // Finished exploring the basin, store it's size
        basin_sizes.push(size);
    }

    // Result is the 3 largest basin sizes multiplied together (BinaryHeap means this is the first 3 values)
    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    2199943210
    3987894921
    9856789892
    8767896789
    9899965678
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 15);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 1134);
    }
}
