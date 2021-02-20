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

    /// Plays the game for the specified number of moves
    pub fn play(&mut self, moves: usize) {
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
    }
}

#[aoc_generator(day23)]
pub fn gen(input: &str) -> CrabCups {
    CrabCups { cups: input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect() }
}

#[aoc(day23, part1)]
fn part1(input: &CrabCups) -> String {
    let mut game = input.clone();
    // Play for 100 turns
    game.play(100);

    // Find the index of cup 1 and rotate round to it
    let index = game.find_cup_index(1);
    game.cups.rotate_left(index.unwrap());
    // Skip cup 1 and then collect the other cups' labels clockwise into a single string
    return game.cups.iter().skip(1).map(|c| c.to_string()).collect::<Vec<String>>().concat();
}

#[aoc(day23, part2)]
fn part2(input: &CrabCups) -> usize {
    let highest = input.cups.iter().max().unwrap().clone();
    let mut cups = VecDeque::with_capacity(1000000);
    input.cups.iter().for_each(|cup| cups.push_back(*cup));
    // Add cups starting from highest until we have 1 million
    for cup in highest + 1 ..=1000000 {
        cups.push_back(cup);
    }
    // Build a new game from our very big list of cups!
    let mut game = CrabCups { cups: cups };
    // Play for 10 million turns
    // TODO: Need to optimise solution as this is incredibly slow
    game.play(10000000);
    
    // Find the index of cup 1 and rotate round to it
    let index = game.find_cup_index(1);
    game.cups.rotate_left(index.unwrap());

    // Get the next two cups after cup 1 and multiply their values
    return game.cups.get(1).unwrap() * game.cups.get(2).unwrap();
}
