use crate::intcode::Intcode;

/// Deploys a drone to the specified position and returns whether or not the
/// tractor beam is active there
fn deploy(drone_program: &Vec<isize>, x: isize, y: isize) -> bool {
    // Deploy a drone to the supplied position
    let mut drone = Intcode::new_with(drone_program, 512);
    drone.inputs().push_back(x);
    drone.inputs().push_back(y);
    drone.run();
    // Return status (0=nothing, 1=beam)
    drone.outputs().pop_front().unwrap() == 1
}

/// Finds the top left position of a square of size x size starting from start
#[allow(clippy::mut_range_bound)]
fn find_square(program: &Vec<isize>, size: isize, start: (isize, isize)) -> Option<(isize, isize)> {
    let mut next_x_start = start.0;
    for y in start.1..start.1 + (size * 100) {
        let mut in_beam = false;
        for x in next_x_start..next_x_start + (size * 100) {
            let current = deploy(program, x, y);
            if in_beam && !current {
                // Gone out the other side of the beam, move to next row (no point to keep searching in this y)
                break;
            } else if current {
                in_beam = true;
                // Set the start for x for the next y pass to be just a bit before where we first found the beam
                // This helps reduce searching as we move further down the beam
                // We turn off the mut_range_bound warning as this doesn't affect the current y pass which is what we want
                next_x_start = x - size;
            }
            // Inside the beam, check that we have a size x size grid and that on at least 1 axis we don't have more then size
            if (!deploy(program, x + size, y) || !deploy(program, x, y + size))
                && deploy(program, x + size - 1, y)
                && deploy(program, x, y + size - 1)
            {
                // Found it!
                return Some((x, y));
            }
        }
    }
    None
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let program = &input.split(',').map(|i| i.parse().unwrap()).collect();
    // Count the number of locations that are impacted by the tractor beam in a 50x50 grid
    (0..50)
        .flat_map(|y| (0..50).filter(move |&x| deploy(program, x, y)))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> isize {
    let program = input.split(',').map(|i| i.parse().unwrap()).collect();
    // We need to first the top left corner of the first 100x100 square
    // It would be too expensive to explore the whole grid looking for it so we start
    // by finding a small 5x5 square. Unfortunately we can't just multiply it's position
    // up to find the 100x100 square start due to rounding errors from the drones limited
    // resolution, so we scale up the x, y coordinates to get where we think a larger size
    // should be. We could do this just for 5x5 then 100x100 but it would put our search
    // start for 100x100 quite a bit out so it's actually quicker to search for 5x5, 25x25,
    // 50x50 and then finally 100x100
    let mut start = (10, 10);
    let mut sizes = [5, 25, 50, 100].iter().peekable();
    while let Some(size) = sizes.next() {
        // Scan at for a size x size square at start
        if let Some((x, y)) = find_square(&program, *size, start) {
            // Found the square of size x size
            if let Some(next) = sizes.peek() {
                // Not at target size yet, scale up start points ready for next scan
                let scale = *next / size;
                start = (x * scale, y * scale);
            } else {
                // Reached target size
                // Final answer is x * 10000 + y
                return (x * 10_000) + y;
            }
        }
    }
    0
}
