use std::collections::HashSet;
use utils::grid::{Direction, Pos};

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<(Direction, isize)> {
    input
        .split(", ")
        .map(|x| {
            // Convert e.g. 'L52' into a direction and number
            let (turn, blocks) = x.split_at(1);
            (turn.parse().unwrap(), blocks.parse().unwrap())
        })
        .collect()
}

/// Follows the `directions` to the end or if `stop_on_visited` is True,
/// to the first place that is visited twice
fn follow(directions: &[(Direction, isize)], stop_on_visited: bool) -> Pos {
    let mut visited = HashSet::new();
    let mut pos = Pos::new(0, 0);
    let mut direction = Direction::Up;
    for (turn, blocks) in directions {
        direction += *turn;
        for _block in 0..*blocks {
            pos = pos.next(direction);
            if stop_on_visited && !visited.insert(pos) {
                return pos;
            }
        }
    }
    pos
}

#[aoc(day1, part1)]
fn part1(input: &[(Direction, isize)]) -> usize {
    follow(input, false).manhattan_distance_origin()
}

#[aoc(day1, part2)]
fn part2(input: &[(Direction, isize)]) -> usize {
    follow(input, true).manhattan_distance_origin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen("R8, R4, R4, R8")), 4);
    }
}
