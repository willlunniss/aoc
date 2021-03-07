use crate::intcode::Intcode;

struct BlocksGame {
    computer: Intcode,
    tiles: Vec<Vec<isize>>,
    score: isize,
}

impl BlocksGame {
    fn from(program: &str) -> BlocksGame {
        BlocksGame {
            computer: Intcode::from_with(program, 1024 * 1024),
            tiles: vec![vec![0; 42]; 24], // Fix screen size to simplify logic,
            score: 0,
        }
    }

    fn add_quarters(&mut self) {
        // Set memory address 0 to 2
        self.computer.set_mem(0, 2);
    }

    /// Prints the game screen to the console
    fn _print(&self) {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    1 => '#',
                    2 => '*',
                    3 => '=',
                    4 => '@',
                    _ => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
        println!("Score: {}\n", self.score);
    }

    /// Plays the game in auto mode until it is beaten
    fn play(&mut self) {
        let mut ball = 0;
        let mut paddle = 0;
        // Run the game loop until done
        loop {
            let finished = self.computer.run();
            // Read the outputs (groups of 3s)
            for cmd in self.computer.outputs().iter().collect::<Vec<_>>().chunks(3) {
                // Unpack the instruction
                let (x, y, tile_id) = (cmd[0], cmd[1], cmd[2]);
                if *x == -1 && *y == 0 {
                    // Special score instruction
                    self.score = *tile_id;
                } else {
                    // Update the grid
                    self.tiles[*y as usize][*x as usize] = *tile_id;
                    // Track the paddle/ball horizontal positions
                    if *tile_id == 3 {
                        paddle = *x;
                    } else if *tile_id == 4 {
                        ball = *x;
                    } 
                }
            }
            if finished {
                return;
            }
            // Update the joystick to make the paddle track the ball
            let joystick = if ball < paddle {
                -1
            } else if ball > paddle {
                1
            } else {
                0
            };
            // Provide the joystick position as input
            self.computer.inputs().push_back(joystick);
        }
    }
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    // Start the game and count the block (2) tiles
    let mut game = BlocksGame::from(input);
    game.play();
    return game.tiles.iter().flat_map(|row| row.iter().filter(|tile| **tile == 2)).collect::<Vec<_>>().len();
}

#[aoc(day13, part2)]
fn part2(input: &str) -> isize {
    // Play the game and report the final score
    let mut game = BlocksGame::from(input);
    game.add_quarters();
    game.play();
    return game.score;
}