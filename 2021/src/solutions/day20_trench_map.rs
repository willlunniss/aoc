use utils::grid::{MapGrid, Pos};

#[aoc_generator(day20)]
fn gen(input: &str) -> (Vec<u8>, MapGrid<u8>) {
    let mut iter = input.lines();

    // First line is the algorithm
    let algorithm = iter
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<Vec<_>>();

    // After a blank line the rest is the grid
    let grid = iter
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                if c == '#' {
                    (Pos::new(x, y), 1)
                } else {
                    (Pos::new(x, y), 0)
                }
            })
        })
        .collect();

    (algorithm, grid)
}

/// Returns the default value for a pixel beyond what we have so far considered
const fn get_default(algorithm: &[u8], step: usize) -> u8 {
    // Some algorithms cause the infinite pixels to flash on/off
    if algorithm[0] == 1 {
        // If the 0 indexed pixel is 1, this means that a 3x3 grid of 0
        // will transition to a 1 on the next step
        if step % 2 == 0 {
            algorithm[0]
        } else {
            algorithm[algorithm.len() - 1]
        }
    } else {
        0
    }
}

/// Returns the binary value that is an index into the algorithm for the 3x3
/// grid around the center position
fn get_3x3_bin_value(grid: &MapGrid<u8>, center: &Pos, default: u8) -> usize {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .rev()
    .enumerate()
    .map(|(idx, &shift)| {
        let pos = *center + shift;
        let pixel = *grid.get(&pos).unwrap_or(&default);
        (pixel as usize) << idx
    })
    .sum()
}

/// Enhances an image the specified number of times
fn enhance(grid: &MapGrid<u8>, algorithm: &Vec<u8>, steps: usize) -> MapGrid<u8> {
    let mut grid = grid.clone();
    // Assume sqaure grid that grows out in each direction by +1 each step
    let size = grid.values().count();
    let mut min_x = 0_isize;
    let mut min_y = 0_isize;
    let mut max_x = (size as f64).sqrt() as isize;
    let mut max_y = max_x;
    for step in 1..=steps {
        // Expand the grid that we consider
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;

        // Some algorithms cause the infinite pixels to flash on/off
        // work out what the default value should be for pixels beyond the grid
        let default = get_default(algorithm, step);

        let mut next = MapGrid::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pos = Pos::from((x, y));
                next.insert(pos, algorithm[get_3x3_bin_value(&grid, &pos, default)]);
            }
        }
        std::mem::swap(&mut next, &mut grid);
    }
    grid
}

#[aoc(day20, part1)]
fn part1(input: &(Vec<u8>, MapGrid<u8>)) -> usize {
    let (algorithm, grid) = input;
    enhance(grid, algorithm, 2)
        .values()
        .filter(|&&p| p == 1)
        .count()
}

#[aoc(day20, part2)]
fn part2(input: &(Vec<u8>, MapGrid<u8>)) -> usize {
    let (algorithm, grid) = input;
    enhance(grid, algorithm, 50)
        .values()
        .filter(|&&p| p == 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
"};

    #[test]
    fn test_part1_example() {
        let (algorithm, grid) = gen(EXAMPLE_INPUT);
        let default = get_default(&algorithm, 1);
        let index = get_3x3_bin_value(&grid, &Pos::new(2, 2), default);
        assert_eq!(index, 34);
        assert_eq!(algorithm[index], 1);
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 35);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 3351);
    }
}
