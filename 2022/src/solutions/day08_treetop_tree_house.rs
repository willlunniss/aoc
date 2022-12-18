use utils::grid::{Direction, VecGrid};

#[aoc_generator(day8)]
fn gen(input: &str) -> VecGrid<u8> {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
fn part1(input: &VecGrid<u8>) -> usize {
    input
        .into_iter()
        .filter(|(pos, &height)| {
            // Keep trees which are visible in any direction
            Direction::all().any(|d| {
                let mut pos = pos.next(d);
                while let Some(tree) = input.get(pos) {
                    if tree >= height {
                        return false; // Taller tree - not visible in this direction
                    }
                    pos = pos.next(d);
                }
                true
            })
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(input: &VecGrid<u8>) -> usize {
    input
        .into_iter()
        .map(|(pos, &height)| {
            // Calculate the scenic score based on viewing distances
            Direction::all()
                .map(|d| {
                    let mut viewing_distance = 0;
                    let mut pos = pos.next(d);
                    while let Some(tree) = input.get(pos) {
                        viewing_distance += 1;
                        if tree >= height {
                            break; // Taller tree - cannot see past it
                        }
                        pos = pos.next(d);
                    }
                    viewing_distance
                })
                .product()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    30373
    25512
    65332
    33549
    35390
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 21);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 8);
    }
}
