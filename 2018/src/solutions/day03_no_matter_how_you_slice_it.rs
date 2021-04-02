use std::collections::{HashMap, HashSet};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split input into different parts
        // From: #1 @ 808,550: 12x22
        //   To: [1, 808, 550, 12, 22]
        let parts = s
            .split(&['#', ' ', '@', ',', ':', 'x'][..])
            .filter(|&x| !x.is_empty())
            .collect::<Vec<_>>();
        Ok(Self {
            id: parts[0].parse().unwrap(),
            x: parts[1].parse().unwrap(),
            y: parts[2].parse().unwrap(),
            width: parts[3].parse().unwrap(),
            height: parts[4].parse().unwrap(),
        })
    }
}

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Claim> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &[Claim]) -> usize {
    let mut fabric = HashMap::new();
    for claim in input {
        // For each claim mark the area of the fabric by incrementing a counter for each position
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                fabric.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    // Result is number of positions on the fabric with 2 or more claims
    fabric.values().filter(|&&e| e >= 2).count()
}

#[aoc(day3, part2)]
fn part2(input: &[Claim]) -> usize {
    let mut fabric = HashMap::new();
    let mut intact = HashSet::new();
    for claim in input {
        // For each claim mark the fabric
        let mut no_overlaps = true;
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                if let Some(claimed_id) = fabric.insert((x, y), claim.id) {
                    // This bit was already claimed!
                    // Remove the previously claimed id from intact and mark us as having overlaps
                    no_overlaps = false;
                    intact.remove(&claimed_id);
                }
            }
        }
        if no_overlaps {
            // Managed to claim the whole area without overlapping, save ID in intact
            intact.insert(claim.id);
        }
    }
    // Result is the single ID left in intact
    *intact.iter().next().unwrap()
}
