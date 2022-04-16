use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<isize> {
    // Represent ( as going up a floor and ) as down a floor using +1/-1
    input
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[isize]) -> isize {
    // Result is simply the sum of +1 up/-1 down floor moves
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[isize]) -> isize {
    input
        .iter()
        .fold_while((0, 0), |(mut pos, mut floor), x| {
            // Keep moving up and down until floor is negative (in the basement)
            pos += 1;
            floor += x;
            if floor < 0 {
                Done((pos, floor))
            } else {
                Continue((pos, floor))
            }
        })
        .into_inner()
        .0 // Return the position through the directions when we entered the basement
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen("))(((((")), 3);
        assert_eq!(part1(&gen(")())())")), -3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(")")), 1);
        assert_eq!(part2(&gen("()())")), 5);
    }
}
