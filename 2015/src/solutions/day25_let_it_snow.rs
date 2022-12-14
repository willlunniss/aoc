use itertools::Itertools;

#[aoc_generator(day25)]
fn gen(input: &str) -> (u64, u64) {
    input
        .split(&[' ', ',', '.'][..])
        .flat_map(str::parse)
        .collect_tuple()
        .unwrap()
}

#[aoc(day25, part1)]
fn part1(input: &(u64, u64)) -> u64 {
    let (target_row, target_column) = *input;
    // Find what sequence number will be at the target row/column
    // Calculate the diagonal that will contain it
    let containing_diagonal = target_row + target_column - 1;
    // Use triangle numbers to get the sequence number at the end of that diagonal (top right)
    let diagonal_end = containing_diagonal * (containing_diagonal + 1) / 2;
    // Then step backwards to the actual row/column we want based on how close to the end it is
    let sequence_number = diagonal_end - (containing_diagonal - target_column);
    // Keep computing the next `sequence_number` codes to find the one to use
    (1..sequence_number).fold(20_151_125, |code, _number| (code * 252_533) % 33_554_393)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&(4, 1)), 24592653);
        assert_eq!(part1(&(2, 5)), 15514188);
        assert_eq!(part1(&(4, 4)), 9380097);
        assert_eq!(part1(&(6, 6)), 27995004);
        assert_eq!(part1(&(1, 6)), 33511524);
    }
}
