use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
pub struct CrabCups {
    cups: VecDeque<usize>
}

impl CrabCups {

    /// Finds the index of the specified cup (or None if not found)
    pub fn find_cup_index(&self, cup: usize) -> Option<usize> {
        for (index, value) in self.cups.iter().enumerate() {
            if *value == cup {
                return Some(index);
            }
        }
        return None;
    }

    /// Plays the game for the specified number of moves and returns
    /// the final ordering of cups after cup 1
    pub fn play(&mut self, moves: usize) -> String {
        // Make a note of lowest and highest for later
        let lowest = self.cups.iter().min().unwrap().clone();
        let highest = self.cups.iter().max().unwrap().clone();
        for _ in 1..=moves {
            // Current cup is front of the queue
            let current = self.cups.front().unwrap().clone();
            // Pick the next 3 cups
            let picked = [self.cups.remove(1), self.cups.remove(1), self.cups.remove(1)];
            // Calculate the destination cup and then find it's index
            let mut destination_cup = current;
            let mut destination_index = None;
            while destination_index.is_none() {
                // Decrement the destination cup until we find it
                destination_cup -= 1;
                if destination_cup < lowest {
                    // Wrap back to highest cup
                    destination_cup = highest;
                }
                // See if the destination cup exists
                destination_index = self.find_cup_index(destination_cup);
            }
            // Insert the picked cups after the destination cup
            for cup in picked.iter().rev() {
                self.cups.insert(destination_index.unwrap() + 1, cup.unwrap().clone());
            }
            // Advance the current cup by 1
            self.cups.rotate_left(1);
        }

        // Find the index of cup 1 and rotate round to it
        let index = self.find_cup_index(1);
        let mut final_cups = self.cups.clone();
        final_cups.rotate_left(index.unwrap());
        // Skip cup 1 and then collect the other cups' labels clockwise into a single string
        return final_cups.iter().skip(1).map(|c| c.to_string()).collect::<Vec<String>>().concat();
    }
}

#[aoc_generator(day23)]
pub fn gen(input: &str) -> CrabCups {
    CrabCups { cups: input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect() }
}

#[aoc(day23, part1)]
fn part1(input: &CrabCups) -> String {
    let mut game = input.clone();
    return game.play(100);
}

#[aoc(day23, part2)]
fn part2(input: &CrabCups) -> usize {
    
    return 0;
}
