fn traverse_slope(input: &str, right: usize, down: usize) -> i64 {
    // Data should look like:
    /*
    0: ..##.......
    1: #...#...#..
    2: .#....#..#.
    3: ..#.#...#.#
    ...
    n: .#..#...#.#
    */
    // Work out how far over we move on each row
    let step: f64 = right as f64 / down as f64;
    let mut x: f64 = 0f64;
    let mut trees = 0;
    for row in input.lines() {
        let width = row.len();
        // Only check if we have moved over onto an exact integer position (for when we move down more than 1 per move)
        if x.fract() == 0.0 && row.chars().nth(x as usize % width).unwrap() == '#' {
            trees += 1;
        }
        x += step;
    }
    return trees as i64;
}

#[aoc(day3, part1)]
fn part1(input: &str) -> i64 {
    // Just traverse the one slope and count trees
    traverse_slope(&input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
    // Traverse these 5 slopes and calculate the multiple of the number of
    // trees encountered on each slope
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for slope in slopes.iter() {
        let (right, down) = slope;
        result = result * traverse_slope(&input, *right, *down);
    }
    return result;
}
