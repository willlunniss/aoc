use itertools::Itertools;
use std::cmp::Ordering;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
struct Moon {
    position: Vec<isize>,
    velocity: Vec<isize>,
}

impl Moon {
    /// Calculates the energy of the moon as the sum of the absolute values of it's position's parts multiplied by the absolute values of it's velocity's parts
    fn energy(&self) -> usize {
        return self
            .position
            .iter()
            .map(|val| isize::abs(*val) as usize)
            .sum::<usize>()
            * self
                .velocity
                .iter()
                .map(|val| isize::abs(*val) as usize)
                .sum::<usize>();
    }

    fn apply_velocity(&mut self) {
        // For each axis
        for axis in 0..3 {
            self.position[axis] += self.velocity[axis];
        }
    }
}

impl FromStr for Moon {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = Vec::new();
        let mut pos = Vec::new();
        // Parse <x=-1, y=0, z=2> into a [-1, 0, 2]
        for c in s.chars() {
            match c {
                '-' | '0'..='9' => value.push(c), // Part of a number, append to our current value
                ',' | '>' => {
                    // End of a number, parse what we have in value into the complete number
                    pos.push(
                        value
                            .into_iter()
                            .collect::<String>()
                            .parse::<isize>()
                            .unwrap(),
                    );
                    // Reset
                    value = Vec::new();
                }
                _ => {} // Ignored
            }
        }
        Ok(Moon {
            position: pos,
            velocity: vec![0; 3],
        })
    }
}

fn apply_gravity(moons: &mut Vec<Moon>) {
    // Generate all of the different pairs
    for pair in (0..moons.len()).combinations(2) {
        // Then process each axis
        for axis in 0..3 {
            // Adjust velocity's to pull moons together e.g. if 0 > 1 then decrease 0's velocity and increase 1's
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
fn part1(input: &str) -> usize {
    let mut moons = gen(input);
    for _step in 1..=1_000 {
        // Apply gravity to all pairs of moons
        apply_gravity(&mut moons);
        // Now apply the velocity to all moons
        moons.iter_mut().for_each(|moon| moon.apply_velocity());
    }
    // Result is total energy in the system
    return moons.iter().map(|moon| moon.energy()).sum();
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let initial_state = gen(input);
    let mut moons = gen(input);
    let mut step = 0;
    // Find how many steps it takes for us to get back to the initial state
    // FIXME: This will take forever, need to do it differently
    loop {
        step += 1;
        // Apply gravity to all pairs of moons
        apply_gravity(&mut moons);
        // Now apply the velocity to all moons
        moons.iter_mut().for_each(|moon| moon.apply_velocity());
        // Check vs initial state
        if moons == initial_state {
            // Got back to initial state, return number of steps
            return step;
        }
        if step > 10_000 {
            return 0;
        }
    }
}
