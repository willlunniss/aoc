use crate::solutions::day10_knot_hash::KnotHasher;
use std::collections::HashSet;
use utils::grid::Pos;

fn count_set_bits(value: u128) -> usize {
    let mut set = 0;
    for bit in 0..128 {
        if value & (1 << bit) != 0 {
            set += 1;
        }
    }
    set
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    let mut used = 0;
    for row in 0..128 {
        // Build key based on the input and row number
        let key = format!("{}-{}", input, row);
        // Calculate the knot hash for this row
        let hash = KnotHasher::hash(&key);
        // Parse as hex value
        let value = u128::from_str_radix(&hash, 16).unwrap();
        // Number of used squares is equal to number of set bits
        used += count_set_bits(value);
    }
    used
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    // Build up a set representing the used squares on the disk
    let mut grid = HashSet::new();
    for row in 0..128 {
        // Build key based on the input and row number
        let key = format!("{}-{}", input, row);
        // Calculate the knot hash for this row
        let hash = KnotHasher::hash(&key);
        // Parse as hex value
        let value = u128::from_str_radix(&hash, 16).unwrap();
        // Number of used squares is equal to number of set bits
        for col in 0..128_u128 {
            if value & (1 << (127 - col)) != 0 {
                grid.insert(Pos::new(col as usize, row));
            }
        }
    }

    // Count number of regions
    let mut regions = 0;
    while !grid.is_empty() {
        // Pick a start square
        let start = *grid.iter().next().unwrap();
        let mut adjacent = HashSet::new();
        let mut queue = vec![start];
        adjacent.insert(start);
        // Find all adjacent squares
        while let Some(pos) = queue.pop() {
            for neighbour in pos.neighbours() {
                if grid.contains(&neighbour) && adjacent.insert(neighbour) {
                    queue.push(neighbour);
                }
            }
        }
        // Remove all squares for this region
        grid.retain(|x| !adjacent.contains(x));
        // Increment region counter
        regions += 1;
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "flqrgnkx";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 8108);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 1242);
    }
}
