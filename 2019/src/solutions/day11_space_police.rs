use crate::intcode::Intcode;
use std::collections::HashMap;
use utils::ocr::OcrString;
use utils::grid::Pos;

struct EmergencyHullPainingRobot {
    controller: Intcode,
    tiles: HashMap<(isize, isize), isize>,
    direction: usize,
    pos: (isize, isize),
}

impl EmergencyHullPainingRobot {
    fn new(program: &str) -> Self {
        Self {
            controller: Intcode::from_with(program, 1024 * 1024),
            tiles: HashMap::new(),
            direction: 0,
            pos: (0, 0),
        }
    }

    /// Paints the hull
    fn paint(&mut self) {
        // Keep running until the controller tells us to stop
        loop {
            // Access camera to provide input (assume 0/black if not yet painted)
            let tile = self.tiles.entry(self.pos).or_default();
            self.controller.inputs().push_back(*tile);
            // Run the controller, it will process the input and give us some outputs
            if self.controller.run() {
                // Complete
                return;
            }
            // Act on outputs (expect two)
            // Update the tile colour based on the first output
            *tile = self.controller.outputs().pop_front().unwrap();
            // Change the direction based on the second output
            if self.controller.outputs().pop_front().unwrap() == 0 {
                // Turn left
                self.direction = (self.direction + 4 - 1) % 4;
            } else {
                // Turn right
                self.direction = (self.direction + 1) % 4;
            }
            // Move forward 1
            match self.direction {
                0 => self.pos = (self.pos.0, self.pos.1 + 1),
                1 => self.pos = (self.pos.0 + 1, self.pos.1),
                2 => self.pos = (self.pos.0, self.pos.1 - 1),
                3 => self.pos = (self.pos.0 - 1, self.pos.1),
                _ => panic!("Unexpected direction {}", self.direction),
            }
        }
    }

    fn set_current_tile(&mut self, colour: isize) {
        self.tiles.insert(self.pos, colour);
    }

    /// Returns a nested vector represented what has been painted
    #[allow(dead_code)]
    fn get_painted(&self) -> Vec<Vec<isize>> {
        // 1st pass: Find the size of the grid needed
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        for pos in self.tiles.keys() {
            if pos.0 < min_x {
                min_x = pos.0;
            } else if pos.0 > max_x {
                max_x = pos.0;
            }
            if pos.1 < min_y {
                min_y = pos.1;
            } else if pos.1 > max_y {
                max_y = pos.1;
            }
        }
        let mut grid = vec![vec![0; 1 + (max_x - min_x) as usize]; 1 + (max_y - min_y) as usize];
        // 2nd pass: Build the grid (inverse Y axis)
        for (pos, colour) in &self.tiles {
            grid[(max_y - pos.1) as usize][(pos.0 - min_x) as usize] = *colour;
        }
        grid
    }

    fn get_painted_points(&self) -> impl Iterator <Item=Pos> + '_ {
        self.tiles.iter().filter(|(_, &value)| value == 1).map(|(&(x, y), _)| Pos::from((x - 1, -y)))
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    // Create a new robot and tell it to paint all of the tiles
    let mut robot = EmergencyHullPainingRobot::new(input);
    robot.paint();
    // Result is simply the number of tiles visited
    robot.tiles.len()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> Option<String> {
    // Create a new robot
    let mut robot = EmergencyHullPainingRobot::new(input);
    // Set the starting tile to white
    robot.set_current_tile(1);
    // Paint the hull
    robot.paint();
    // Render the output
    let ocr : OcrString = robot.get_painted_points().collect();
    ocr.decode()
}
