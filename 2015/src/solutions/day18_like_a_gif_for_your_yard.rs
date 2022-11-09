use utils::grid::{Pos, VecGrid};

#[aoc_generator(day18)]
fn gen(input: &str) -> VecGrid<char> {
    input.parse().unwrap()
}

// Simulates a set number of steps on the grid of lights
fn simulate(input: &VecGrid<char>, steps: usize, force_corners_on: bool) -> usize {
    let mut current = input.clone();
    for _ in 0..steps {
        let mut next = current.clone();
        for (pos, value) in &current {
            // For all positions, count neighbours that are currently on
            let on_neighbours = current
                .neighbours8_ex(pos)
                .filter(|(_, x)| x == &Some('#'))
                .count();
            // Then update state as needed
            if *value == '#' {
                if !(2..=3).contains(&on_neighbours) {
                    // Turn off
                    next.insert(pos, '.');
                }
            } else {
                if 3 == on_neighbours {
                    // Turn on
                    next.insert(pos, '#');
                }
            }
        }
        if force_corners_on {
            // Force corners to stay on
            next.insert(Pos::new(0, 0), '#');
            next.insert(Pos::new(0, current.width() - 1), '#');
            next.insert(Pos::new(current.height() - 1, 0), '#');
            next.insert(Pos::new(current.height() - 1, current.width() - 1), '#');
        }
        std::mem::swap(&mut next, &mut current);
    }
    current.values().filter(|&&x| x == '#').count()
}

#[aoc(day18, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    simulate(input, 100, false)
}

#[aoc(day18, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    simulate(input, 100, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_1: &str = indoc! {"
    .#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..
"};

    static EXAMPLE_INPUT_2: &str = indoc! {"
    ##.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####.#
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(simulate(&gen(EXAMPLE_INPUT_1), 4, false), 4);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(simulate(&gen(EXAMPLE_INPUT_2), 5, true), 17);
    }
}
