use crate::intcode::Intcode;
use std::collections::HashMap;

struct RepairDroid {
    controller: Intcode,
    pos: (isize, isize),
    tiles: HashMap<(isize, isize), isize>,
}

impl RepairDroid {
    fn from(program: &str) -> RepairDroid {
        RepairDroid {
            controller: Intcode::from_with(program, 1024 * 1024),
            pos: (0, 0),
            tiles: HashMap::new(),
        }
    }

    /// Returns all other directions
    fn others(direction: usize) -> Vec<usize> {
        match direction {
            1 => [2, 3, 4].to_vec(),
            2 => [1, 3, 4].to_vec(),
            3 => [1, 2, 4].to_vec(),
            4 => [1, 2, 3].to_vec(),
            _ => panic!("Unexpected direction {}", direction),
        }
    }

    /// Returns the opposite direction
    fn back(direction: usize) -> usize {
        if direction % 2 == 0 {
            direction - 1
        } else {
            direction + 1
        }
    }

    /// Works out what direction we should go in next based on where we have tried previously and how we got here
    fn next_direction(
        unexplored_directions: &mut HashMap<(isize, isize), Vec<usize>>,
        direction_stack: &mut Vec<usize>,
        pos: (isize, isize),
    ) -> usize {
        let options = unexplored_directions.entry(pos).or_default();
        if !options.is_empty() {
            // We have an unexplored direction - try that!
            return options.pop().unwrap();
        }
        // We've tried all options at this location, need to backtrack
        if let Some(direction) = direction_stack.pop() {
            // direction is how we got here, so need turn it into the direction to go backwards
            RepairDroid::back(direction)
        } else {
            // If there is nowhere to go then return 0 to indicate we have fully explored everything
            0
        }
    }

    /// Works out what the next position will be if moving in the supplied direction
    fn next(pos: (isize, isize), direction: usize) -> (isize, isize) {
        match direction {
            1 => (pos.0, pos.1 + 1), // North
            2 => (pos.0, pos.1 - 1), // South
            3 => (pos.0 + 1, pos.1), // East
            4 => (pos.0 - 1, pos.1), // West
            _ => panic!("Unexpected direction {}", direction),
        }
    }

    /// Explores the map from the current starting position and returns either
    /// the path length to the oxygen system (if in find oxygen mode) else
    /// the maximum path length if exploring the whole map
    fn explore(&mut self, find_oxygen: bool) -> usize {
        // Track how we moved (so that we can backtrack as well as check path lengths)
        let mut direction_stack: Vec<usize> = Vec::new();
        // Track which directions we haven't explored yet for a given position
        let mut unexplored_directions: HashMap<(isize, isize), Vec<usize>> = HashMap::new();
        // At our start position (which may not be (0,0) if called more than once) we could go in any direction
        unexplored_directions.insert(self.pos, [1, 2, 3, 4].to_vec());
        // Track max path length
        let mut max_path_length = 0;
        // Run the controller in a loop until we get to the oxygen system if requested or fully explore the map
        loop {
            // Work out where to go next
            let direction = RepairDroid::next_direction(
                &mut unexplored_directions,
                &mut direction_stack,
                self.pos,
            );
            if direction == 0 {
                // We have come all the way back to the start, must have fully explored the map!
                return max_path_length;
            }
            let next = RepairDroid::next(self.pos, direction);
            // Try to move to it
            self.controller.inputs().push_back(direction as isize);
            self.controller.run();
            // Check status to see if we were able to move
            let status = self.controller.outputs().pop_front().unwrap();
            self.tiles.insert(next, status);
            match status {
                0 => {} // Hit a wall, we will try going in a different direction on next loop
                1 | 2 => {
                    // Moved OK, update position
                    self.pos = next;

                    unexplored_directions.entry(self.pos).or_insert_with(|| {
                        // We haven't been here before, record how we got here
                        direction_stack.push(direction);
                        if direction_stack.len() > max_path_length {
                            max_path_length += 1;
                        }
                        // Then init with all directions except where we came from
                        RepairDroid::others(RepairDroid::back(direction))
                    });
                    if status == 2 && find_oxygen {
                        // Found the oxygen system!
                        return direction_stack.len();
                    }
                }
                _ => panic!("Unexpected status {}", status),
            }
        }
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    let mut droid = RepairDroid::from(input);
    // Find the oxygen system and then return the path length to it
    droid.explore(true)
}

#[aoc(day15, part2)]
fn part2(input: &str) -> usize {
    let mut droid = RepairDroid::from(input);
    // Find the oxygen system
    droid.explore(true);
    // Then start exploring again (don't reset position) to discover the whole map returning the maximum
    // path length from the oxygen system
    droid.explore(false)
}
