/// Reacts a polymer by removing adjacent units of the same type (letter) but different polarity (case)
/// and returns the length of the resulting polymer
fn react(iter: impl Iterator<Item = char>) -> usize {
    // Store the resulting polymer as we scan through the input
    let mut polymer = Vec::new();
    for unit in iter {
        // Compare the current unit with the last in the polymer
        // Treat chars as ASCII numbers, look for a absolute difference of 32 between them to mean they are
        // the same type but with opposite polarity
        if (*polymer.last().unwrap_or(&' ') as i8 - unit as i8).abs() == 32 {
            // Units react, remove the last from the polymer
            polymer.pop();
        } else {
            // Units don't react, add to the polymer
            polymer.push(unit);
        }
    }
    polymer.len()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    // React the polymer as is
    react(input.chars())
}

#[aoc(day5, part2)]
fn part2(input: &str) -> usize {
    // Calculate what the length of the reacted polymer would be if we removed each type of unit
    // and then return the minimum length
    ('a'..='z')
        .map(|unit| {
            // React the polymer with this type of unit removed (of both polarities)
            react(
                input
                    .chars()
                    .filter(|&u| u != unit && u != unit.to_ascii_uppercase()),
            )
        })
        .min()
        .unwrap()
}
