#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect()
}

// Calculate the sum of values which equal the value
// at offset positions from themselves
fn solve(input: &[usize], offset: usize) -> usize {
    let mut sum = 0;
    let size = input.len();
    for (pos, value) in input.iter().enumerate() {
        if *value == input[(size + pos - offset) % size] {
            sum += value;
        }
    }
    sum
}

#[aoc(day1, part1)]
fn part1(input: &[usize]) -> usize {
    solve(input, 1)
}

#[aoc(day1, part2)]
fn part2(input: &[usize]) -> usize {
    solve(input, input.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[1, 1, 2, 2]), 3);
        assert_eq!(part1(&[1, 1, 1, 1]), 4);
        assert_eq!(part1(&[1, 2, 3, 4]), 0);
        assert_eq!(part1(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[1, 2, 1, 2]), 6);
        assert_eq!(part2(&[1, 2, 2, 1]), 0);
        assert_eq!(part2(&[1, 2, 3, 4, 2, 5]), 4);
    }
}
