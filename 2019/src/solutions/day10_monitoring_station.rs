use std::collections::HashMap;
use std::collections::HashSet;

/// Calculates the angle of p2 from p1 between 0->359 degrees (0 = North, 90 = East)
fn angle(p1: (isize, isize), p2: (isize, isize)) -> f64 {
    return -((p2.0 - p1.0) as f64)
        .atan2((p2.1 - p1.1) as f64)
        .to_degrees()
        + 180f64;
}

/// Finds the asteroid that has the most other asteroids in los
fn find_best_base(asteroids: &Vec<(isize, isize)>) -> ((isize, isize), usize) {
    // Consider each asteroid as a potential base
    // Calculate the angle to every other asteroid
    // Count the number of unique angles (duplicates would not be in los)
    // Take the maximum
    let mut best_base = (0, 0);
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
    return (best_base, max_asteroids);
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<(isize, isize)> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Get the positions of all asteroids
    let mut asteroids = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, state) in row.iter().enumerate() {
            if *state == '#' {
                asteroids.push((x as isize, y as isize))
            }
        }
    }
    return asteroids;
}

#[aoc(day10, part1)]
fn part1(input: &Vec<(isize, isize)>) -> usize {
    let (_, max_asteroids) = find_best_base(input);
    return max_asteroids;
}

#[aoc(day10, part2)]
fn part2(input: &Vec<(isize, isize)>) -> isize {
    // Find the best base location again
    let (base, _) = find_best_base(input);

    // Store a list of the positions of all asteroids at each angle
    let mut asteroids_by_angle: HashMap<usize, Vec<((isize, isize), usize)>> = HashMap::new();
    for asteroid in input.iter().filter(|asteroid| **asteroid != base) {
        let angle = (angle(base, *asteroid) * 1_000f64) as usize; // Store angle rounded to 3 decimal places as can't key off of a float
        let distance = (isize::abs(asteroid.0 - base.0) + isize::abs(asteroid.1 - base.1)) as usize;
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
    let mut angles: Vec<usize> = asteroids_by_angle.keys().map(|x| *x).collect();
    angles.sort();
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
                    return (asteroid.0 * 100) + asteroid.1;
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
        assert_eq!(angle((10, 10), (10, 0)) as isize, 0); // N
        assert_eq!(angle((10, 10), (20, 10)) as isize, 90); // E
        assert_eq!(angle((10, 10), (10, 20)) as isize, 180); // S
        assert_eq!(angle((10, 10), (0, 10)) as isize, 270); // w
    }
}
