#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &[isize]) -> isize {
    let max = *input.iter().max().unwrap();
    // For all positions between 0 and maximum
    // Find the one the results in the minimum number of total moves
    (0..=max)
        .map(|target| input.iter().map(|&start| isize::abs(start - target)).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[isize]) -> isize {
    let max = *input.iter().max().unwrap();
    // For all positions between 0 and maximum
    // Find the one the results in the minimum number of total moves
    (0..=max)
        .map(|target| {
            input
                .iter()
                .map(|&start| 
                    // Cost increased with every move
                    (1..=isize::abs(start - target)).sum::<isize>())
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 37);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 168);
    }
}
