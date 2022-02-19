#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let elves = input.parse().unwrap();
    let mut circle = (0..elves).collect::<Vec<_>>();
    circle.rotate_left(1);
    let mut pos = 0;
    while circle[pos] != pos {
        circle[pos] = circle[circle[pos]];
        pos = circle[pos];
    }
    circle[pos] + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("5"), 3);
    }
}
