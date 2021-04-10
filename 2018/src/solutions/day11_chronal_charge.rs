use digits_iterator::DigitsExtension;
use std::collections::HashMap;

// Calculates the power level for a given fuel cell
fn power_level(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;
    let hundreds = (((rack_id * y) + serial_number) * rack_id)
        .digits()
        .rev()
        .nth(2)
        .unwrap_or(0) as isize;
    hundreds - 5
}

/// Finds the grid of the specified size which has the maximum total power
///
/// Returns a tuple of power and x, y positions of that grid
fn maximise_power(
    levels: &mut HashMap<(usize, usize), isize>,
    serial_number: usize,
    size: usize,
) -> (isize, (usize, usize)) {
    let mut max = isize::MIN;
    let mut pos = (0, 0);
    for y_base in 1..=300 - size {
        for x_base in 1..=300 - size {
            // Calculate the power for a grid of size size at x_base, y_base
            let power = (y_base..y_base + size)
                .map(|y| {
                    (x_base..x_base + size)
                        .map(|x| {
                            // Get the level for this specific cell, or calculate it if we haven't done so yet
                            *levels
                                .entry((x, y))
                                .or_insert_with(|| power_level(x, y, serial_number))
                        })
                        .sum::<isize>()
                })
                .sum::<isize>();
            if power > max {
                // Found a grid with a higher power
                max = power;
                pos = (x_base, y_base);
            }
        }
    }
    (max, pos)
}

#[aoc_generator(day11)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day11, part1)]
fn part1(input: &usize) -> String {
    let mut levels: HashMap<(usize, usize), isize> = HashMap::new();
    // Simply find the grid of maximum power for size of 3
    let (_, pos) = maximise_power(&mut levels, *input, 3);
    format!("{},{}", pos.0, pos.1)
}

#[aoc(day11, part2)]
fn part2(input: &usize) -> String {
    let mut levels: HashMap<(usize, usize), isize> = HashMap::new();
    let mut best = (0, 1, (1, 1));
    // Find the grid of size between 1 and 300 that has maximum power
    // FIXME: This is too slow, must be a better way
    for size in 1..=300 {
        let (power, pos) = maximise_power(&mut levels, *input, size);
        if power > best.0 {
            // New best
            best = (power, size, pos);
        } else if power < (best.0 / 2) {
            // Now less that half our best, including any more cells will just make it worse
            // so no point to continue searching
            break;
        }
    }
    format!("{},{},{}", best.2 .0, best.2 .1, best.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
