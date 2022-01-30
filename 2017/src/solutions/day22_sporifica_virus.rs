use utils::grid::{Direction, MapGrid, Pos};

#[aoc_generator(day22)]
fn gen(input: &str) -> MapGrid<char> {
    input.parse().unwrap()
}

fn infect(grid: &MapGrid<char>, bursts: usize, evolved: bool) -> usize {
    let mut grid = grid.clone();
    let middle = ((grid.values().count() as f64).sqrt() / 2f64) as usize;
    let mut position = Pos::new(middle, middle);
    let mut direction = Direction::Up;
    let mut infections = 0;
    for _ in 0..bursts {
        let state = grid.entry(position).or_insert('.');
        // Update direction based on current node state
        direction = match state {
            '#' => direction.rotate_right(),
            'W' => direction,
            'F' => direction.back(),
            '.' => direction.rotate_left(),
            _ => unreachable!(),
        };
        // Update state of node
        *state = match state {
            '#' => {
                if evolved {
                    'F'
                } else {
                    '.'
                }
            }
            'W' => {
                infections += 1;
                '#'
            }
            'F' => '.',
            '.' => {
                if evolved {
                    'W'
                } else {
                    infections += 1;
                    '#'
                }
            }
            _ => unreachable!(),
        };
        // Move forward
        position = position.next(direction);
    }
    infections
}

#[aoc(day22, part1)]
fn part1(input: &MapGrid<char>) -> usize {
    infect(input, 10_000, false)
}

#[aoc(day22, part2)]
fn part2(input: &MapGrid<char>) -> usize {
    infect(input, 10_000_000, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
        ..#
        #..
        ...
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 5587);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 2_511_944);
    }
}
