use std::collections::HashSet;
use utils::grid::VecGrid;

#[aoc_generator(day11)]
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

fn step(grid: &mut VecGrid<u8>) -> usize {
    let mut flash_que = Vec::new();
    let mut flashed = HashSet::new();
    // Increase energy level across the grid
    for pos in grid.indexes() {
        grid[pos] += 1;
        if grid[pos] == 10 {
            // If they have flashed, add to queue to process
            flash_que.push(pos);
        }
    }

    while let Some(pos) = flash_que.pop() {
        if flashed.insert(pos) {
            // For every new flash increment the energy level for all neighbours
            for neighbour in pos.neighbours8() {
                if grid.contains(neighbour) {
                    grid[neighbour] += 1;
                    if grid[neighbour] == 10 {
                        // If they have flashed, add to queue to process
                        flash_que.push(neighbour);
                    }
                }
            }
        }
    }
    for &pos in &flashed {
        grid[pos] = 0;
    }
    // Result is number of flashes this turn
    flashed.len()
}

#[aoc(day11, part1)]
fn part1(input: &VecGrid<u8>) -> usize {
    let mut grid = input.clone();
    (1..=100).fold(0, |acc, _| acc + step(&mut grid))
}

#[aoc(day11, part2)]
fn part2(input: &VecGrid<u8>) -> Option<usize> {
    let mut grid = input.clone();
    let size = grid.width() * grid.height();
    (1..).find(|_| step(&mut grid) == size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 1656);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), Some(195));
    }
}
