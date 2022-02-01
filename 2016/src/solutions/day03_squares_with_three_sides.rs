use itertools::Itertools;

/// Checks whether a triangle is possible i.e.
/// the sum of any two sides is greater than the remaining side
fn is_possible(triangle: &[usize; 3]) -> bool {
    triangle
        .iter()
        .permutations(3)
        .all(|sides| *sides[0] < sides[1] + sides[2])
}

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<[usize; 3]> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &[[usize; 3]]) -> usize {
    // Count how many triangles are possible
    input
        .iter()
        .filter(|triangle| is_possible(triangle))
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &[[usize; 3]]) -> usize {
    // Each group of three rows has 3 triangles specified vertically
    // Transpose rows/columns to get the actual 3 triangles and then
    // as before count how many are possible
    input
        .iter()
        .tuples()
        .flat_map(|(row1, row2, row3)| {
            vec![
                [row1[0], row2[0], row3[0]],
                [row1[1], row2[1], row3[1]],
                [row1[2], row2[2], row3[2]],
            ]
        })
        .filter(is_possible)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT_2: &str = indoc! {"
    101 301 501
    102 302 502
    103 303 503
    201 401 601
    202 402 602
    203 403 603
    "};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen("5 10 25")), 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT_2)), 6);
    }
}
