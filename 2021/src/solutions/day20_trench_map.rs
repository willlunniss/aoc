use utils::grid::{Pos, VecGrid};

#[aoc_generator(day20)]
fn gen(input: &str) -> (Vec<u8>, VecGrid<u8>) {
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
        .map(|line| {
            line.chars()
                .map(move |c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .into();

    (algorithm, grid)
}

/// Returns the default value for a pixel beyond what we have so far considered
/// for the previous and next cycles
const fn get_default(algorithm: &[u8], step: usize) -> (u8, u8) {
    // Some algorithms cause the infinite pixels to flash on/off
    if algorithm[0] == 1 {
        // If the 0 indexed pixel is 1, this means that a 3x3 grid of 0
        // will transition to a 1 on the next step
        if step % 2 == 0 {
            (algorithm[0], algorithm[algorithm.len() - 1])
        } else {
            (algorithm[algorithm.len() - 1], algorithm[0])
        }
    } else {
        (0, 0)
    }
}

/// Returns the binary value that is an index into the algorithm for the 3x3
/// grid around the center position
fn get_3x3_bin_value(grid: &VecGrid<u8>, center: &Pos, default: u8) -> usize {
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
        let pixel = grid.get(pos).unwrap_or(default);
        (pixel as usize) << idx
    })
    .sum()
}

/// Enhances an image the specified number of times
fn enhance(start_grid: &VecGrid<u8>, algorithm: &Vec<u8>, steps: usize) -> VecGrid<u8> {
    // Assume square grid that grows out in each direction by +1 each step
    // We can take the current size then calculate the final width/height
    let size = start_grid.values().count();
    let width = (size as f64).sqrt() as isize;
    let target_width = width as usize + ((steps + 1) * 2);
    // Build a new grid for it
    let mut grid = VecGrid::new_sized(0, target_width, target_width);
    // And then calculate the offsets that we will use
    let mut min = steps as isize;
    let mut max = min + width;
    // Then fill in the tarting grid
    for (pos, &value) in start_grid {
        grid.insert(pos + (steps + 1, steps + 1), value);
    }
    // Now enhance the grid
    for step in 1..=steps {
        // Expand the grid that we consider
        min -= 1;
        max += 1;

        // Some algorithms cause the infinite pixels to flash on/off
        // work out what the default value should be for pixels beyond the grid
        let (prev_default, next_default) = get_default(algorithm, step);
        // Setup the next grid
        let mut next = VecGrid::new_sized(next_default, target_width, target_width);
        // Calculate the next values
        for y in min..=max {
            for x in min..=max {
                let pos = Pos::from((x, y));
                next.insert(pos, algorithm[get_3x3_bin_value(&grid, &pos, prev_default)]);
            }
        }
        std::mem::swap(&mut next, &mut grid);
    }
    grid
}

#[aoc(day20, part1)]
fn part1(input: &(Vec<u8>, VecGrid<u8>)) -> usize {
    let (algorithm, grid) = input;
    enhance(grid, algorithm, 2)
        .values()
        .filter(|&&p| p == 1)
        .count()
}

#[aoc(day20, part2)]
fn part2(input: &(Vec<u8>, VecGrid<u8>)) -> usize {
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
        let (default, _) = get_default(&algorithm, 1);
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
