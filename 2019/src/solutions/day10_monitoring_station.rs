use std::collections::HashMap;
use std::collections::HashSet;
use utils::grid::Pos;

/// Calculates the angle of p2 from p1 between 0->359 degrees (0 = North, 90 = East)
fn angle(p1: Pos, p2: Pos) -> f64 {
    -((p2.x - p1.x) as f64)
        .atan2((p2.y - p1.y) as f64)
        .to_degrees()
        + 180f64
}

/// Finds the asteroid that has the most other asteroids in los
fn find_best_base(asteroids: &[Pos]) -> (Pos, usize) {
    // Consider each asteroid as a potential base
    // Calculate the angle to every other asteroid
    // Count the number of unique angles (duplicates would not be in los)
    // Take the maximum
    let mut best_base = Pos::new(0, 0);
    let mut max_asteroids = 0;
    for base in asteroids {
        let count = asteroids
            .iter()
            .filter(|asteroid| *asteroid != base)
            .map(|asteroid| {
                (angle(*base, *asteroid) * 1_000f64) as usize // Store angle rounded to 3 decimal places as can't key off of a float
            })
            .collect::<HashSet<_>>()
            .len();
        if count > max_asteroids {
            max_asteroids = count;
            best_base = *base;
        }
    }
    (best_base, max_asteroids)
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<Pos> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Get the positions of all asteroids
    let mut asteroids = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, state) in row.iter().enumerate() {
            if *state == '#' {
                asteroids.push(Pos::new(x, y))
            }
        }
    }
    asteroids
}

#[aoc(day10, part1)]
fn part1(input: &[Pos]) -> usize {
    let (_, max_asteroids) = find_best_base(input);
    max_asteroids
}

#[aoc(day10, part2)]
fn part2(input: &[Pos]) -> isize {
    // Find the best base location again
    let (base, _) = find_best_base(input);

    // Store a list of the positions of all asteroids at each angle
    let mut asteroids_by_angle: HashMap<usize, Vec<(Pos, usize)>> = HashMap::new();
    for asteroid in input.iter().filter(|asteroid| **asteroid != base) {
        let angle = (angle(base, *asteroid) * 1_000f64) as usize; // Store angle rounded to 3 decimal places as can't key off of a float
        let distance = (isize::abs(asteroid.x - base.x) + isize::abs(asteroid.y - base.y)) as usize;
        asteroids_by_angle
            .entry(angle)
            .or_default()
            .push((*asteroid, distance));
    }

    // Sort asteroids for each angle by distance from base (reversed so closest is at the end of the list and easy to pop)
    for asteroids in asteroids_by_angle.values_mut() {
        asteroids.sort_by(|a, b| b.1.cmp(&a.1));
    }

    // Go through the angles (from 0->359) popping the closest asteroid for each angle until we have removed the 200th
    let mut angles: Vec<usize> = asteroids_by_angle.keys().copied().collect();
    angles.sort_unstable();
    let mut fire = 0;
    loop {
        // May need to do multiple passes
        for angle in &angles {
            // For each angle see if there are any asteroids left
            if let Some(los) = asteroids_by_angle.get_mut(angle) {
                // Yes there are, FIRE!
                fire += 1;
                let (asteroid, _) = los.pop().unwrap();
                if los.is_empty() {
                    // That was the last one at this angle, remove it
                    // It would be slightly more efficient if we also removed this angle from angles
                    // in prep for another pass but the whole thing is so fast it's not really worth it
                    asteroids_by_angle.remove(angle);
                }
                if fire == 200 {
                    // We are interested in the 200th asteroid to be destroyed
                    return (asteroid.x * 100) + asteroid.y;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        assert_eq!(angle(Pos::new(10, 10), Pos::new(10, 0)) as isize, 0); // N
        assert_eq!(angle(Pos::new(10, 10), Pos::new(20, 10)) as isize, 90); // E
        assert_eq!(angle(Pos::new(10, 10), Pos::new(10, 20)) as isize, 180); // S
        assert_eq!(angle(Pos::new(10, 10), Pos::new(0, 10)) as isize, 270); // w
    }
}
