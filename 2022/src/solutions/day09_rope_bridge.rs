use utils::grid::{Direction, MapGrid, Pos};

/// Simulates a rope with a number of knots moving through a series of motions and
/// returns the number of unique locations the tail knot visits
fn simulate<const KNOTS: usize>(motions: &[(Direction, usize)]) -> usize {
    let mut map = MapGrid::new();
    let mut rope = [Pos::new(0, 0); KNOTS];
    for (direction, steps) in motions {
        for _ in 0..*steps {
            // Move the head knot
            rope[0] = rope[0].next(*direction);
            // Update following knots
            for knot in 1..KNOTS {
                // Check to see if the knot that this one follows has moved too far away
                let follow = rope[knot - 1];
                if rope[knot] != follow && rope[knot].neighbours8().all(|n| n != follow) {
                    // Move tail to follow the rope
                    rope[knot] = rope[knot].step_towards(follow);
                } else {
                    break; // This knot doesn't need to move so no others will
                }
            }
            // Record the position of the last tail knot
            map.insert(rope[KNOTS - 1], '#');
        }
    }
    map.values().count()
}

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|line| {
            let (direction, dist) = line.split_once(' ').unwrap();
            (direction.parse().unwrap(), dist.parse().unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[(Direction, usize)]) -> usize {
    simulate::<2>(input)
}

#[aoc(day9, part2)]
fn part2(input: &[(Direction, usize)]) -> usize {
    simulate::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_1: &str = indoc! {"
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2
"};

    static EXAMPLE_INPUT_2: &str = indoc! {"
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT_1)), 13);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT_2)), 36);
    }
}
