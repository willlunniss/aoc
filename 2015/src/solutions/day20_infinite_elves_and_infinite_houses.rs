/// Returns the lowest house number that receives at least the target
/// number of presents based on specific presents per house and houses
/// per elf
fn first_house_with_presents(
    presents: usize,
    presents_per_house: usize,
    houses_per_elf: Option<usize>,
) -> usize {
    let max_deliveries = houses_per_elf.unwrap_or(usize::MAX);
    // Calculate how many presents would be delivered to all houses
    // To upper bound the houses to consider, go upto the house which could get all `presents` presents
    // from a single elf (plus extras for any with lower numbers still delivering)
    let mut houses: Vec<usize> = vec![0; presents / presents_per_house];
    // For each elf
    (1..presents / presents_per_house).for_each(|elf| {
        // Deliver presents to all houses it would visit
        (elf..presents / presents_per_house)
            .step_by(elf)
            .take(max_deliveries)
            .for_each(|house| houses[house] += elf * presents_per_house);
    });
    // Find the lowest house number with at least the target presents delivered
    houses
        .iter()
        .enumerate()
        .find_map(|(house, delivered)| (delivered >= &presents).then_some(house))
        .unwrap()
}

#[aoc_generator(day20)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
fn part1(input: &usize) -> usize {
    first_house_with_presents(*input, 10, None)
}

#[aoc(day20, part2)]
fn part2(input: &usize) -> usize {
    first_house_with_presents(*input, 11, Some(50))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&70), 4);
    }
}
