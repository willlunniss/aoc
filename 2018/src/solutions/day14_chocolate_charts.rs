use digits_iterator::DigitsExtension;
use std::char;

#[aoc_generator(day14)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
fn part1(input: &usize) -> String {
    let elves = 2;
    let mut recipes: Vec<u8> = [3, 7].to_vec();
    let mut current_indexes: Vec<usize> = (0..elves).collect();
    while recipes.len() < input + 10 {
        // Create new recipes based on the current ones
        let current: Vec<u8> = current_indexes
            .iter()
            .map(|index| recipes[*index])
            .collect();
        recipes.extend(current.iter().sum::<u8>().digits());
        // Advance the index by 1 + value of current recipes (wrapping round if needed)
        current_indexes = current_indexes
            .iter()
            .zip(current)
            .map(|(index, recipe)| (index + 1 + recipe as usize) % recipes.len())
            .collect();
    }
    // Return the scores of the 10 recipes after our input
    recipes
        .iter()
        .skip(*input)
        .map(|recipe| char::from_digit(u32::from(*recipe), 10).unwrap())
        .collect()
}

#[aoc(day14, part2)]
fn part2(input: &usize) -> usize {
    let elves = 2;
    let mut recipes: Vec<u8> = [3, 7].to_vec();
    let mut current_indexes: Vec<usize> = (0..elves).collect();
    let search: Vec<u8> = input.digits().rev().collect();
    let search_len = search.len();
    loop {
        // Create new recipes based on the current ones
        let current: Vec<u8> = current_indexes
            .iter()
            .map(|index| recipes[*index])
            .collect();
        for new in current.iter().sum::<u8>().digits() {
            // Add each new recipe individually
            recipes.push(new);
            if new == search[0] {
                // Just added a new recipe that matches the end of what we are search for
                // Check to see if the last search.len() match what we want
                if recipes
                    .iter()
                    .rev()
                    .take(search_len)
                    .zip(search.iter())
                    .all(|(r, s)| r == s)
                {
                    // Matches, return the number of recipes before the search pattern
                    return recipes.len() - search.len();
                }
            }
        }
        // Advance the index by 1 + value of current recipes (wrapping round if needed)
        current_indexes = current_indexes
            .iter()
            .zip(current)
            .map(|(index, recipe)| (index + 1 + recipe as usize) % recipes.len())
            .collect();
    }
}
