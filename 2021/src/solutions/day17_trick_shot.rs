use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Probe {
    position: (isize, isize),
    velocity: (isize, isize),
    max_height: isize,
}

impl Probe {
    const fn new(initial_velocity: (isize, isize)) -> Self {
        Self {
            position: (0, 0),
            velocity: initial_velocity,
            max_height: 0,
        }
    }

    fn step(&mut self) {
        // Move the probe
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Apply drag
        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        }

        // Apply gravity
        self.velocity.1 -= 1;

        // Track max height
        if self.position.1 > self.max_height {
            self.max_height = self.position.1;
        }
    }

    /// Checks if the probe is in the target area
    fn in_target(&self, target: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> bool {
        target.0.contains(&self.position.0) && target.1.contains(&self.position.1)
    }

    /// Checks if the probe has missed the target area
    fn missed_target(&self, target: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> bool {
        let min = isize::min(*target.1.start(), *target.1.end());
        self.position.1 < min
    }
}

/// Calculates the triangular number of n
const fn tri_num(n: isize) -> isize {
    (n * (n + 1)) / 2
}

#[aoc_generator(day17)]
fn gen(input: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let parts = input.split(&['=', '.', ','][..]).collect::<Vec<_>>();
    (
        (parts[1].parse().unwrap()..=parts[3].parse().unwrap()),
        (parts[5].parse().unwrap()..=parts[7].parse().unwrap()),
    )
}

/// Fires a probe and returns the max height achieved if it hits the target
/// or None if it misses
fn fire(
    initial_velocity: (isize, isize),
    target: &(RangeInclusive<isize>, RangeInclusive<isize>),
) -> Option<isize> {
    let mut probe = Probe::new(initial_velocity);
    for _step in 1.. {
        probe.step();
        if probe.in_target(target) {
            // In side the target
            return Some(probe.max_height);
        } else if probe.missed_target(target) {
            // Gone past it, give up
            return None;
        }
    }
    None
}

#[aoc(day17, part1)]
const fn part1(target: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    // Find the max height a probe can reach and still hit the target by aiming up such that the probe will then drop
    // down from it's highest point and hit the target

    // Ideal x velocity results in forward movement reaching 0 due to wind resistance as the probe is over the target
    // As forward movement reduces by 1 each step, an ideal x velocity is n which gives a triangular number that is equal
    // to the start of the target
    // We don't actually need it though for this calculation

    // Ideal y velocity is as high as possible without it over shooting the target
    // Due to gravity, any initial positive y velocity will return to 0 with a speed of -y, so want asb(bottom of target) -1 to ensure we hit it
    let y_velocity = isize::abs(*target.1.start()) - 1;
    // Max height achieved can then be calculated as the triangular number for initial y velocity
    tri_num(y_velocity)
}

#[aoc(day17, part2)]
fn part2(target: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> usize {
    // Find how many initial velocities we could fire a probe with that would hit the target
    let mut hits = HashSet::new();
    for x in 0..*target.0.end() * 2 {
        for y in *target.1.start()..isize::abs(*target.1.start()) {
            if fire((x, y), target).is_some() {
                hits.insert((x, y));
            }
        }
    }

    hits.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 45);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 112);
    }
}
