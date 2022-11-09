use itertools::Itertools;

#[aoc_generator(day17)]
fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

// Calculates the combinations of containers that add up to exactly the target size
fn valid_combinations(
    containers: &[usize],
    target_size: usize,
) -> impl Iterator<Item = Vec<&usize>> {
    (1..containers.len()).flat_map(move |num_containers| {
        containers // Consider all combinations for different numbers of containers
            .iter()
            .combinations(num_containers)
            .filter(move |x| x.iter().copied().sum::<usize>() == target_size)
    })
}

// Calculates the number of configurations that use the min number of containers
fn valid_minimal_combinations(containers: &[usize], target_size: usize) -> usize {
    // Get all valid combinations
    let valid: Vec<_> = valid_combinations(containers, target_size).collect();
    // Find the min number of containers that works
    let min_containers = valid.iter().map(Vec::len).min().unwrap();
    // Return the number of configurations that use the min number of containers
    valid.iter().filter(|x| x.len() == min_containers).count()
}

#[aoc(day17, part1)]
fn part1(input: &[usize]) -> usize {
    // Return all valid combinations
    valid_combinations(input, 150).count()
}

#[aoc(day17, part2)]
fn part2(input: &[usize]) -> usize {
    // Return the number of configurations that use the min number of containers
    valid_minimal_combinations(input, 150)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    20
    15
    10
    5
    5
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(valid_combinations(&gen(EXAMPLE_INPUT), 25).count(), 4);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(valid_minimal_combinations(&gen(EXAMPLE_INPUT), 25), 3);
    }
}
