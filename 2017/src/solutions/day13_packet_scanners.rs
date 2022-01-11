/// Returns true if a scanner with the specified `range` would catch a packet
/// at the specified `time`
const fn catches_at_time(range: usize, time: usize) -> bool {
    // We are interested in if the position of a scanner will be 0 at the specified time
    // For a range of 3 we are supposed to count 0,1,2,1,0
    // But as we just need to know if we are at 0 we can treat as 0,1,2,3,0
    time % ((range * 2) - 2) == 0
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Vec<(usize, usize)> {
    // Build a Vec with (depth, range) tuples
    input
        .lines()
        .map(|line| {
            let (depth, range) = line.split_once(": ").unwrap();
            (depth.parse().unwrap(), range.parse().unwrap())
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[(usize, usize)]) -> usize {
    let mut severity = 0;
    // Consider all depths that have a scanner
    for (depth, range) in input {
        // Time is just equal to depth into the firewall
        let time = depth;
        // Check if it catches us
        if catches_at_time(*range, *time) {
            // Have been caught
            severity += depth * *range;
        }
    }
    severity
}

#[aoc(day13, part2)]
fn part2(input: &[(usize, usize)]) -> usize {
    'Delay: for delay in 0.. {
        // Consider all depths that have a scanner
        for (depth, range) in input {
            // Time is the delay plus depth into the firewall
            let time = delay + depth;
            // Check if it catches us
            if catches_at_time(*range, time) {
                // Have been caught - try the next delay
                continue 'Delay;
            }
        }
        // Made it through without being caught
        return delay;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
        0: 3
        1: 2
        4: 4
        6: 4
        "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 24);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 10);
    }
}
