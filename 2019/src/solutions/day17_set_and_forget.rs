use crate::intcode::Intcode;
use std::collections::BTreeMap;
use utils::grid::{Direction, Pos, VecGrid};

/// Finds all indexes in source where target exists
fn exists_in<T>(source: &[T], target: &[T]) -> Vec<usize>
where
    T: Eq,
{
    let mut locations = Vec::new();
    let mut offset = 0;
    let length = target.len();
    while offset + length <= source.len() {
        if &source[offset..offset + length] == target {
            // Found a match, advance by length
            locations.push(offset);
            offset += length;
        } else {
            // Not matched, advance
            offset += 1;
        }
    }
    locations
}

struct ASCII {
    controller: Intcode,
}

impl ASCII {
    pub fn from(program: &str) -> Self {
        Self {
            controller: Intcode::from_with(program, 1024 * 1024),
        }
    }

    /// Aligns the cameras and returns the sum of the alignment parameters
    pub fn align_cameras(&mut self) -> isize {
        // Run the program and get the grid
        self.controller.run();
        let grid = Self::decode_camera_output(&mut self.controller.outputs().iter());
        // Find cross over points and return the sum of each alignment param (x*y)
        return Self::find_crossover_points(&grid)
            .iter()
            .map(|pos| pos.x * pos.y)
            .sum();
    }

    pub fn walk_scaffolding(&mut self) -> isize {
        // Run a copy of the program and get the grid
        let mut tmp = self.controller.clone();
        tmp.run();
        let grid = Self::decode_camera_output(&mut tmp.outputs().iter());

        let route = Self::build_route(&grid);

        let compiled = Self::compile_route(&route);

        // Configure robot with directions
        self.controller.set_mem(0, 2);
        for line in compiled {
            self.controller.inputln(&line);
        }
        // Tell it not to show a continuous feed
        self.controller.inputln("n");
        // Now run the robot
        self.controller.run();

        // Return collected dust as the final output value
        return self.controller.outputs().pop_back().unwrap();
    }

    /// Decodes ASCII camera output into a 2d Vector of chars
    fn decode_camera_output<'a, I>(ascii_outputs: &mut I) -> VecGrid<char>
    where
        I: Iterator<Item = &'a isize>,
    {
        let mut grid = Vec::new();
        let mut row = Vec::new();
        for c in ascii_outputs {
            if *c == 10 {
                // New line char, append row if not empty (last line is empty)
                if !row.is_empty() {
                    grid.push(row);
                    row = Vec::new();
                }
            } else {
                // Some other char, append to row
                row.push(*c as u8 as char);
            }
        }
        VecGrid::from(grid)
    }

    /// Finds all of the points where the scaffolding crosses over
    fn find_crossover_points(grid: &VecGrid<char>) -> Vec<Pos> {
        let mut crossovers = Vec::new();
        // Loop over the grid checking cell and neighbours state
        for (pos, value) in grid {
            if *value == '#' && grid.neighbours(pos).all(|x| x == Some('#')) {
                crossovers.push(pos);
            }
        }
        crossovers
    }

    /// Searches the grid for the robot
    fn find_robot(grid: &VecGrid<char>) -> Option<(Pos, Direction)> {
        for (pos, c) in grid {
            match c {
                '^' => return Some((pos, Direction::Up)),
                'v' => return Some((pos, Direction::Down)),
                '<' => return Some((pos, Direction::Left)),
                '>' => return Some((pos, Direction::Right)),
                _ => {}
            }
        }
        None
    }

    /// Builds a route that will let the robot traverse all the scaffolding
    fn build_route(grid: &VecGrid<char>) -> Vec<String> {
        let (mut pos, mut direction) = Self::find_robot(grid).unwrap();
        let mut route = Vec::new();
        loop {
            if let Some((dest, distance)) = Self::scaffold_length(grid, pos, direction) {
                // There is scaffolding in front of us, travel to the end of it
                route.push(distance.to_string());
                pos = dest;
            }
            // Can't go any further in this direction, need to try a different way
            // Try turning left
            if grid.get(pos.next(direction.rotate_left())) == Some('#') {
                direction = direction.rotate_left();
                route.push("L".to_owned());
                continue;
            }

            // Try turning right
            if grid.get(pos.next(direction.rotate_right())) == Some('#') {
                direction = direction.rotate_right();
                route.push("R".to_owned());
                continue;
            }
            // Can't turn left or right, must be the end
            break;
        }
        route
    }

    /// Follows the scaffolding from the current pos in the direction specified
    /// until the end of it and then returns the ending position and distance travelled. If the scaffolding
    /// cannot be followed in this direction returns None
    fn scaffold_length(
        grid: &VecGrid<char>,
        start: Pos,
        direction: Direction,
    ) -> Option<(Pos, usize)> {
        let mut distance = 0;
        let mut current = start;
        loop {
            let next = current.next(direction);
            if grid.get(next) == Some('#') {
                // Found scaffolding, move to it
                current = next;
                distance += 1;
            } else {
                break;
            }
        }
        if distance > 0 {
            Some((current, distance))
        } else {
            None
        }
    }

    /// Takes a route and turns it into a number of common functions
    ///
    /// Eg. R,8,R,8,R,4,R,4,L,6,L,2,R,8,R,8,L,6,L,2,R,4,R,4->
    /// Main: A,B,C,A,C,B
    /// Function A: R,8,R,8
    /// Function B: R,4,R,4
    /// Function C: L,6,L,2
    fn compile_route(source: &[String]) -> Vec<String> {
        let mut functions = Vec::new();
        let mut found_valid = false;
        // FIXME: This is a hack to try a range of size adjustments to matches
        // For each attempt, we adjust the size of the match by the value to allow other matches to claim more
        for non_greedy_backtrack in [[0, 0, 0], [2, 0, 0], [0, 2, 0], [0, 0, 2]].to_vec() {
            // Each pass through try different settings for backtracking
            let mut index = 0;
            let mut found = vec![false; source.len()];
            functions.clear();
            // Min pattern is length 4 and want at least 1 match so stop 8 before the end
            while index + 8 <= source.len() {
                // Keep increasing the target that we look for to find the best match
                // It may not be the longest, as a function that is 5 long and occurs 8 times
                // is better than one that is 6 long that only occurs 3 times
                let mut prev_matches = 1;
                let mut size = 2;
                while index + size + 2 < source.len() - 2 && !found[index + size + 1] {
                    // Get the slice we are looking for
                    let target = &source[index..index + size + 2];
                    // See how many instances there are of it (+1 for this one)
                    let matches = exists_in(&source[index..source.len()], target);
                    let current_matches = matches.len() as isize + 1;
                    if matches.len() > 1 && (current_matches as f64 / prev_matches as f64) >= 0.49 {
                        // Found some matches and doesn't drop too much
                        size += 2;
                        prev_matches = current_matches
                    } else {
                        // Either no match, or it reduces total entries matched
                        break;
                    }
                }
                if size > 2 {
                    // Found repeated pattern!
                    // See if we should adjust it's size
                    // This is a hack to deal with the fact that a greedy approach won't work
                    let function_id = functions.len();
                    if non_greedy_backtrack[function_id] > 0 {
                        size -= non_greedy_backtrack[function_id];
                    }

                    // Get the matches for the adjusted size
                    let matched = source[index..index + size].to_vec();
                    // Mark the matched parts so we don't try to process them again
                    let indexes = exists_in(source, &matched);
                    for idx in &exists_in(source, &matched) {
                        for i in 0..size {
                            found[idx + i] = true;
                        }
                    }
                    // Record the function info
                    functions.push((matched, indexes));
                    if function_id == 2 {
                        // Should have finished now as we are looking for exactly 3 functions
                        // If not we will try again with different settings
                        break;
                    }
                    // Haven't found them all yet, reset ready to find the next
                    index += size;
                } else {
                    // Failed to find a match, give up and we will try again with different settings
                    break;
                }
                // Advance through vec, skipping over bits that we had previously matched
                while found[index] {
                    index += 2;
                }
            }
            // Check to see that we found 3 functions and that they represent the whole route
            if functions.len() == 3 && found.iter().all(|x| *x) {
                // Found a set of valid functions
                found_valid = true;
                break;
            }
        }
        // Check we did actually find a set of functions that cover the whole route
        assert!(found_valid);

        // Work out what order we need to call the movement functions (BTreeMap will sort by key)
        let mut ordering = BTreeMap::new();
        for (function_id, (_, indexes)) in functions.iter().enumerate() {
            for index in indexes.iter() {
                // index is the position in the route where this function occurs
                ordering.insert(index, function_id);
            }
        }
        // Now build the compiled route
        let mut compiled = Vec::new();
        // First the main routine which is a comma separated list of functions to call
        // (0 -> "A", 1 -> "B", 2 -> "C")
        compiled.push(
            ordering
                .values()
                .map(|function| ((function + 65) as u8 as char).to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        // Then add the 3 movement functions as comma separated
        for (_, (function, _)) in functions.iter().enumerate() {
            compiled.push(function.join(","));
        }
        compiled
    }
}

#[aoc(day17, part1)]
fn part1(input: &str) -> isize {
    let mut computer = ASCII::from(input);
    // Return the result of aligning the cameras
    computer.align_cameras()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> isize {
    let mut computer = ASCII::from(input);
    // Walk scaffolding and return the amount of space dust collected
    computer.walk_scaffolding()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exists_in() {
        // Test finding patterns in a route
        let source = [
            "L", "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "4", "L", "4", "R", "8", "R",
            "10", "L", "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "10", "R", "10", "L",
            "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "10", "R", "10", "R", "4", "L", "4",
            "L", "4", "R", "8", "R", "10", "R", "4", "L", "10", "R", "10", "R", "4", "L", "10",
            "R", "10", "R", "4", "L", "4", "L", "4", "R", "8", "R", "10",
        ];

        let target = ["L", "4", "L", "4", "L", "10", "R", "4"];
        let matches = exists_in(&source, &target);
        assert_eq!(matches, [0, 18, 32].to_vec());

        let target = ["R", "4", "L", "4", "L", "4", "R", "8", "R", "10"];
        let matches = exists_in(&source, &target);
        assert_eq!(matches, [8, 46, 68].to_vec());

        let target = ["R", "4", "L", "10", "R", "10"];
        let matches = exists_in(&source, &target);
        assert_eq!(matches, [26, 40, 56, 62].to_vec());
    }

    #[test]
    fn test_compress_route_sample() {
        // Check using the sample route
        let route = [
            "R", "8", "R", "8", "R", "4", "R", "4", "R", "8", "L", "6", "L", "2", "R", "4", "R",
            "4", "R", "8", "R", "8", "R", "8", "L", "6", "L", "2",
        ];
        // Compile the route and then reconstruct it
        let compiled =
            ASCII::compile_route(&route.iter().map(|r| (*r).to_string()).collect::<Vec<_>>()[0..]);
        let reconstructed = reconstruct_route(&compiled);
        // Check it matches the original route
        assert_eq!(route.to_vec(), reconstructed)
    }

    #[test]
    fn test_compress_route_big() {
        // Checks using my puzzle input's route
        let route = [
            "L", "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "4", "L", "4", "R", "8", "R",
            "10", "L", "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "10", "R", "10", "L",
            "4", "L", "4", "L", "10", "R", "4", "R", "4", "L", "10", "R", "10", "R", "4", "L", "4",
            "L", "4", "R", "8", "R", "10", "R", "4", "L", "10", "R", "10", "R", "4", "L", "10",
            "R", "10", "R", "4", "L", "4", "L", "4", "R", "8", "R", "10",
        ];
        // Compile the route and then reconstruct it
        let compiled =
            ASCII::compile_route(&route.iter().map(|r| (*r).to_string()).collect::<Vec<_>>()[0..]);
        let reconstructed = reconstruct_route(&compiled);
        // Check it matches the original route
        assert_eq!(route.to_vec(), reconstructed)
    }

    /// Transforms a compiled route back to the original route for testing
    fn reconstruct_route(compiled: &[String]) -> Vec<&str> {
        let mut route = Vec::new();
        for part in compiled[0].split(',') {
            // Add each referenced function
            route.extend(match part {
                "A" => compiled[1].split(','),
                "B" => compiled[2].split(','),
                "C" => compiled[3].split(','),
                _ => panic!(),
            });
        }
        route
    }
}
