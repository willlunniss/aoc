use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    // Split into groups based on empty lines
    // Count number of chars in each line for each group by putting them in a HashSet and getting the length
    // Sum across all groups
    return input.split("\r\n\r\n")
        .map(|group| group.lines().flat_map(|l| l.chars()).collect::<HashSet<char>>().len())
        .sum();
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    // Split into groups based on empty lines
    // For each line (person's answers) turn into HashSet of chars
    // Find the intersection of each persons answers for that group and get length
    // Sum across all groups    
    return input.split("\r\n\r\n")
        .map(|group| {
            group.lines().map(|line| line.chars().collect())
                .fold(None, | acc: Option<HashSet<char>>, chars| {
                    if acc == None {
                        Some(chars) // Initialise with first persons answers
                    } else {
                        // For each subsequent person reduce by calculating the intersection
                        Some(acc.unwrap().intersection(&chars).copied().collect())
                    }
                }).unwrap().len()
        })
        .sum();
}
