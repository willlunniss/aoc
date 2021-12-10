#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<isize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn median(input: &Vec<isize>) -> isize {
    let mut numbers = input.clone();
    numbers.sort_unstable();
    if numbers.len() % 2 == 0 {
        (numbers[numbers.len() / 2] + numbers[(numbers.len() / 2) + 1]) / 2
    } else {
        numbers[numbers.len() / 2]
    }
}

fn mean(numbers: &[isize]) -> isize {
    numbers.iter().sum::<isize>() / numbers.len() as isize
}

#[aoc(day7, part1)]
fn part1(input: &Vec<isize>) -> isize {
    // With a linear cost of movement the median will ~minimise overall fuel usage
    let median = median(input);
    // Due to rounding it may be slightly off, so check it and +/- 1 either side
    (-1..=1)
        .map(|offset| median + offset)
        .map(|target| input.iter().map(|&start| isize::abs(start - target)).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(input: &[isize]) -> isize {
    // With a increasing cost of movement the mean will ~minimise overall fuel usage
    let mean = mean(input);
    // Due to rounding it may be slightly off, so check it and +/- 1 either side
    (-1..=1)
        .map(|offset| mean + offset)
        .map(|target| {
            input
                .iter()
                .map(|&start| {
                    // Cost increased with every move aka triangular numbers (1, 2, 3, 4 ...)
                    let dist = isize::abs(start - target);
                    dist * (dist + 1) / 2
                })
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
