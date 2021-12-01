use itertools::Itertools;

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split('\t')
                .map(|value| value.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Vec<usize>]) -> usize {
    // Calculate the checksum by summing the difference between each rows min/max value
    input
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum::<usize>()
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<usize>]) -> usize {
    // Calculate the result by summing the result of dividing the only two evenly divisible values in each row
    input
        .iter()
        .flat_map(|row| {
            row.iter().permutations(2).filter_map(|values| {
                if values[0] % values[1] == 0 {
                    Some(values[0] / values[1])
                } else {
                    None
                }
            })
        })
        .sum::<usize>()
}
