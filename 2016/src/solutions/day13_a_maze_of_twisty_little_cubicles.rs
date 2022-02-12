use bit_iter::BitIter;
use pathfinding::prelude::{dijkstra, dijkstra_all};
use utils::grid::Pos;

/// Returns true if the position is an open space
fn open_space(pos: &Pos, favourite_number: usize) -> bool {
    pos.x >= 0
        && pos.y >= 0
        && BitIter::from(
            (pos.x * pos.x)
                + (3 * pos.x)
                + (2 * pos.x * pos.y)
                + pos.y
                + (pos.y * pos.y)
                + favourite_number as isize,
        )
        .count()
            % 2
            == 0
}

#[aoc_generator(day13)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day13, part1)]
fn part1(input: &usize) -> usize {
    let target = if *input == 10 {
        Pos::new(7, 4) // Test
    } else {
        Pos::new(31, 39) // Real
    };
    // Find the shortest path to the target
    dijkstra(
        &Pos::new(1, 1),
        |p| {
            p.neighbours()
                .filter(|pos| open_space(pos, *input))
                .map(|p| (p, 1))
                .collect::<Vec<_>>()
        },
        |p| p == &target,
    )
    .unwrap()
    .1
}

#[aoc(day13, part2)]
fn part2(input: &usize) -> usize {
    let start = &Pos::new(1, 1);
    // Find all positions that are reachable in at most 50 steps
    // + 1 to include the starting position
    dijkstra_all(start, |p| {
        p.neighbours()
            .filter(|pos| open_space(pos, *input))
            .filter(|pos| pos.manhattan_distance(start) <= 50)
            .map(|p| (p, 1))
            .collect::<Vec<_>>()
    })
    .values()
    .filter(|(_, steps)| *steps <= 50)
    .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen("10")), 11);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen("10")), 0);
    }
}
