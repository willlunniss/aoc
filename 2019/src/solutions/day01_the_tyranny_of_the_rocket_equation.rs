/// Calculates the fuel required for a given mass
fn required_fuel(mass: usize) -> usize {
    return usize::max(mass /3, 2) - 2;
}

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse::<usize>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<usize>) -> usize {
    // Calculate the sum of fuel required for each module
    return input.iter().map(|module| required_fuel(*module)).sum();
}

#[aoc(day1, part2)]
fn part2(input: &Vec<usize>) -> usize {
    // Calculate the sum of fuel required for each module
    // accounting for the fuel required by the fuel
    return input.iter().map(|module| {
        let mut fuel = required_fuel(*module);
        let mut total = fuel;
        while fuel > 0 {
            fuel = required_fuel(fuel);
            total += fuel;
        }
        return total;
    }).sum();
}