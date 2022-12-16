use itertools::Itertools;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<usize> {
    // Split into per elf lists and then sum up all values in each
    input
        .split("\n\n")
        .map(|list| list.lines().flat_map(str::parse::<usize>).sum())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[usize]) -> usize {
    // Reverse sort to then sum the 3 largest totals
    input.iter().sorted_by(|a, b| b.cmp(a)).take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 24000);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 45000);
    }
}
