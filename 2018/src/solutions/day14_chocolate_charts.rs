use digits_iterator::DigitsExtension;
use std::char;

#[aoc_generator(day14)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
fn part1(input: &usize) -> String {
    let mut recipes: Vec<u8> = [3, 7].to_vec();
    let mut current_indexes = [0, 1];
    while recipes.len() < input + 10 {
        // Create new recipes based on the current ones
        let current = [recipes[current_indexes[0]], recipes[current_indexes[1]]];
        recipes.extend(current.iter().sum::<u8>().digits());
        // Advance the index by 1 + value of current recipes (wrapping around if needed)
        current_indexes[0] = (current_indexes[0] + 1 + current[0] as usize) % recipes.len();
        current_indexes[1] = (current_indexes[1] + 1 + current[1] as usize) % recipes.len();
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
    let mut recipes: Vec<u8> = [3, 7].to_vec();
    let mut current_indexes = [0, 1];
    // Store what we are searching for in reverse
    let search: Vec<u8> = input.digits().rev().collect();
    let search_len = search.len();
    loop {
        // Create new recipes based on the current ones
        let current = [recipes[current_indexes[0]], recipes[current_indexes[1]]];
        for new in current.iter().sum::<u8>().digits() {
            // Add each new recipe individually
            recipes.push(new);
            if new == search[0] {
                // Just added a new recipe that matches the end of what we are search for
                // Check backwards through the recipes to see if the last search.len() match what we want
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
        current_indexes[0] = (current_indexes[0] + 1 + current[0] as usize) % recipes.len();
        current_indexes[1] = (current_indexes[1] + 1 + current[1] as usize) % recipes.len();
    }
}
