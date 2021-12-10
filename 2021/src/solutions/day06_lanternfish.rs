use cached::proc_macro::cached;

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

#[aoc(day6, part1, simulate)]
fn part1_simulate(input: &Vec<usize>) -> usize {
    // Naive approach - simulate what happens one day at a time
    // Only feasible for small numbers of days
    let mut fishes = input.clone();
    for _day in 1..=80 {
        let mut next = Vec::new();
        for fish in fishes {
            // For each fish, if their timer is at 0 create a new one at 8 and add them at 6
            if fish == 0 {
                next.push(6);
                next.push(8);
            } else {
                // For all other values add -1 current timer
                next.push(fish - 1);
            }
        }
        fishes = next;
    }
    fishes.len()
}

/// Recursively calculates how many fish there will be after the supplied days given current timer
#[cached]
fn calculate(timer: usize, days: usize) -> usize {
    if days <= timer {
        // Cannot reproduce in time available, will just be this fish
        return 1;
    }

    // How many days will be left after the first (current) cycle completes
    let days_after_first_cycle = days - timer - 1;

    // How many cycles will this fish complete in total
    let completed_cycles = 1 + (days_after_first_cycle / 7);

    // Total is 1 + for each completed cycle, how many fish given an initial timer of 8 based taking into account when it will be spawned
    1 + (0..completed_cycles)
        .map(|cycle| calculate(8, days_after_first_cycle - (cycle * 7)))
        .sum::<usize>()
}

/// Optimised count of how many fish there will be after a certain number of days
fn quick_count(fishes: &[usize], days: usize) -> usize {
    // Init a circular buffer with the number of fish with each timer value
    let mut fish_with_timer = vec![0; 9];
    for &initial_timer in fishes {
        fish_with_timer[initial_timer] += 1;
    }

    let mut pos = 0;
    for _day in 1..=days {
        // For each day, get the number of fish that have a timer of 0 and remove them
        let completed_cycle = fish_with_timer[pos];
        fish_with_timer[pos] = 0;

        // Then add the new fish in what will be the +6 and +8 slots
        fish_with_timer[(pos + 1 + 6) % 9] += completed_cycle;
        fish_with_timer[(pos + 1 + 8) % 9] += completed_cycle;

        // Advance the position forward by 1
        pos = (pos + 1) % 9;
    }
    // Result is the number of fish at the end
    fish_with_timer.iter().sum()
}

#[aoc(day6, part1, compute)]
fn part1_compute(input: &[usize]) -> usize {
    input.iter().map(|&timer| calculate(timer, 80)).sum()
}

#[aoc(day6, part2, compute)]
fn part2_compute(input: &[usize]) -> usize {
    input.iter().map(|&timer| calculate(timer, 256)).sum()
}

#[aoc(day6, part2, quick_count)]
fn part2_quick_count(input: &[usize]) -> usize {
    quick_count(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1_example_simulate() {
        assert_eq!(part1_simulate(&gen(EXAMPLE_INPUT)), 5934);
    }

    #[test]
    fn test_part1_example_compute() {
        assert_eq!(part1_compute(&gen(EXAMPLE_INPUT)), 5934);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2_compute(&gen(EXAMPLE_INPUT)), 26_984_457_539);
    }

    #[test]
    fn test_part2_example_quick_count() {
        assert_eq!(part2_quick_count(&gen(EXAMPLE_INPUT)), 26_984_457_539);
    }

    #[test]
    fn test_quick_count() {
        assert_eq!(quick_count(&gen(EXAMPLE_INPUT), 18), 26);
    }

    #[test]
    fn test_calculate() {
        // Checks result by stepping through fish starting with a timer of 4 through to 28 days
        assert_eq!(calculate(4, 0), 1);
        assert_eq!(calculate(4, 1), 1);
        assert_eq!(calculate(4, 2), 1);
        assert_eq!(calculate(4, 3), 1);
        assert_eq!(calculate(4, 4), 1);
        assert_eq!(calculate(4, 5), 2);
        assert_eq!(calculate(4, 6), 2);
        assert_eq!(calculate(4, 7), 2);
        assert_eq!(calculate(4, 8), 2);
        assert_eq!(calculate(4, 9), 2);
        assert_eq!(calculate(4, 10), 2);
        assert_eq!(calculate(4, 11), 2);
        assert_eq!(calculate(4, 12), 3);
        assert_eq!(calculate(4, 13), 3);
        assert_eq!(calculate(4, 14), 4);
        assert_eq!(calculate(4, 15), 4);
        assert_eq!(calculate(4, 16), 4);
        assert_eq!(calculate(4, 17), 4);
        assert_eq!(calculate(4, 18), 4);
        assert_eq!(calculate(4, 19), 5);
        assert_eq!(calculate(4, 20), 5);
        assert_eq!(calculate(4, 21), 7);
        assert_eq!(calculate(4, 22), 7);
        assert_eq!(calculate(4, 23), 8);
    }
}
