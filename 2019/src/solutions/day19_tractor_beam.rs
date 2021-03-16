use crate::intcode::Intcode;

#[aoc(day19, part1)]
fn part1(input: &str) -> isize {
    let program = input.split(',').map(|i| i.parse().unwrap()).collect();
    let mut count = 0;
    // Find out which locations are impacted by the tractor beam
    for y in 0..50 {
        for x in 0..50 {
            // Provide the coordinates
            let mut drone = Intcode::new_with(&program, 512);
            drone.inputs().push_back(x);
            drone.inputs().push_back(y);
            drone.run();
            // Sum up the status values (0=nothing, 1=beam)
            let status = drone.outputs().pop_front().unwrap();
            count += status;
        }
    }
    // Return the number of locations affected by the beam
    count
}

#[aoc(day19, part2)]
fn part2(input: &str) -> isize {
    0
}
