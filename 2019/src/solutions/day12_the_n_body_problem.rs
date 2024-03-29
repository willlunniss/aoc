use itertools::Itertools;
use num::integer::lcm;
use std::cmp::Ordering;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
struct Moon {
    position: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    /// Calculates the energy of the moon as the sum of the absolute values of it's position's parts multiplied by the absolute values of it's velocity's parts
    fn energy(&self) -> isize {
        return self
            .position
            .iter()
            .map(|val| isize::abs(*val))
            .sum::<isize>()
            * self
                .velocity
                .iter()
                .map(|val| isize::abs(*val))
                .sum::<isize>();
    }

    fn apply_velocity(&mut self, axes: &[usize]) {
        // For each axis
        for &axis in axes {
            self.position[axis] += self.velocity[axis];
        }
    }
}

impl FromStr for Moon {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Vec::new();
        let mut pos = [0, 0, 0];
        let mut index = 0;
        // Parse <x=-1, y=0, z=2> into a [-1, 0, 2]
        for c in s.chars() {
            match c {
                '-' | '0'..='9' => value.push(c), // Part of a number, append to our current value
                ',' | '>' => {
                    // End of a number, parse what we have in value into the complete number
                    pos[index] = value
                        .into_iter()
                        .collect::<String>()
                        .parse::<isize>()
                        .unwrap();
                    // Reset
                    value = Vec::new();
                    index += 1;
                }
                _ => {} // Ignored
            }
        }
        Ok(Self {
            position: pos,
            velocity: [0, 0, 0],
        })
    }
}

fn apply_gravity(moons: &mut Vec<Moon>, axes: &[usize]) {
    // Generate all of the different pairs
    for pair in (0..moons.len()).combinations(2) {
        // Then process specified axes
        for &axis in axes {
            // Adjust velocity to pull moons together e.g. if 0 > 1 then decrease 0's velocity and increase 1's
            let (pos1, pos2) = (moons[pair[0]].position[axis], moons[pair[1]].position[axis]);
            match pos1.cmp(&pos2) {
                Ordering::Greater => {
                    moons[pair[0]].velocity[axis] -= 1;
                    moons[pair[1]].velocity[axis] += 1;
                }
                Ordering::Less => {
                    moons[pair[1]].velocity[axis] -= 1;
                    moons[pair[0]].velocity[axis] += 1;
                }
                Ordering::Equal => {} // Don't do anything
            }
        }
    }
}

fn gen(input: &str) -> Vec<Moon> {
    return input.lines().map(|moon| moon.parse().unwrap()).collect();
}

#[aoc(day12, part1)]
fn part1(input: &str) -> isize {
    let mut moons = gen(input);
    let axes = [0, 1, 2].to_vec();
    for _step in 1..=1_000 {
        // Apply gravity to all pairs of moons
        apply_gravity(&mut moons, &axes);
        // Now apply the velocity to all moons
        moons.iter_mut().for_each(|moon| moon.apply_velocity(&axes));
    }
    // Result is total energy in the system
    return moons.iter().map(Moon::energy).sum();
}

#[aoc(day12, part2)]
fn part2(input: &str) -> isize {
    let moons = gen(input);
    let mut periods = Vec::new();
    // First calculate the period for each axis (as they operate independently)
    for axis in 0..3 {
        // Find how many steps it takes for us to get back to the initial state for each axis
        let axes = [axis].to_vec();
        let mut step = 0;
        let mut moons = moons.clone();
        // Record the initial position for each moon on this axis
        let initial_pos = moons
            .iter()
            .map(|moon| moon.position[axis])
            .collect::<Vec<_>>();
        loop {
            step += 1;
            // Apply gravity to all pairs of moons
            apply_gravity(&mut moons, &axes);
            // Now apply the velocity to all moons
            moons.iter_mut().for_each(|moon| moon.apply_velocity(&axes));
            // Check to see if all moons are back to their initial position (and with 0 velocity)
            let back_at_initial = moons.iter().enumerate().all(|(index, moon)| {
                moon.position[axis] == initial_pos[index] && moon.velocity[axis] == 0
            });
            if back_at_initial {
                // Got back to initial state, number of steps is the period for this axis
                break;
            }
        }
        // Save the period for this axis (the number of steps taken to get back to the initial state)
        periods.push(step);
    }
    // Whole system period is the least common multiple of all axes periods
    lcm(periods[0], lcm(periods[1], periods[2]))
}
