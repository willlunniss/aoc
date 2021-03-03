#[derive(PartialEq, Debug, Clone)]
pub struct CrabCups {
    // An array of cups where the values are the values of the next cup along
    cup_indexes: Vec<usize>,
    // The value of the current cup
    current: usize,
}

impl<'a> IntoIterator for &'a CrabCups {
    type Item = usize;
    type IntoIter = CrabCupsIterator<'a>;

    /// Create an iterator that can be used to get each cup in their current order starting with the current cup
    fn into_iter(self)  -> Self::IntoIter {
        CrabCupsIterator {
            crab_cups: self,
            cup: self.current,
        }
    }
}

pub struct CrabCupsIterator<'a> {
    crab_cups: &'a CrabCups,
    cup: usize,
}

impl <'a> Iterator for CrabCupsIterator<'a> {
    // Custom iterator to emit the values in the right order
    // Needed because we store the next values (linked list style) at each index rather than keeping an ordered array/list of values
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.cup == 0 {
            // Looped back around, stop
            return None;
        }
        // Result will be the current cup
        let result = self.cup;
        // Look up next cup
        self.cup = self.crab_cups.cup_indexes[self.cup];
        if self.cup == self.crab_cups.current {
            // About to loop back around, set to cup 0 (which doesn't exist) to indicate we should stop
            self.cup = 0;
        }
        return Some(result);
    }
}

impl CrabCups {

    /// Create a new CrabCups game
    /// 
    /// input: Starting cup numbers
    /// pad_to_size: Total number of cups at play (will add sequentially from highest number in the input)
    pub fn new(input: &Vec<usize>, pad_to_size: usize) -> CrabCups {
        // Build a Vec where each index represents a specific cup value and the data in that index is the value of the next cup
        // Cup 0 doesn't exist but we include it to make indexing easier
        // We represent it like this as it makes the act of moving picked cups to a new destination
        // significantly faster than other solutions

        // e.g. 389125467 is represented as
        //     0258647391 with current = 3

        let mut indexes = Vec::with_capacity(pad_to_size + 1);
        // First init the initial fields so that we can index into them
        for _ in 0..=input.len() {
            indexes.push(0);
        }
        // Now set the starting input
        for i in 0..input.len() {
            indexes[input[i]] = input[(i + 1) % input.len()]
        }
        // If applicable, generate extra cups needed to pad it out
        if pad_to_size > input.len() {
            for i in input.len() + 1..pad_to_size {
                indexes.push(i + 1);
            }
            // Edit the last cup of the input to point to the first extra cup
            indexes[*input.last().unwrap()] = input.len() + 1;
            // And then add a final cup pointing back to the first input cup
            indexes.push(input[0]);
        }
        // Create the new instance, with the current cup pointing at the first from the input
        return CrabCups { cup_indexes: indexes, current: input[0] };
    }

    /// Sets the current cup
    pub fn set_current(&mut self, cup: usize) {
        self.current = cup;
    }

    /// Plays the game for the specified number of moves
    pub fn play(&mut self, moves: usize) {
        let max_cup = self.cup_indexes.len() - 1;
        for _game_move in 1..=moves {
            // We are interested in the 4 cups after the current
            // 0-2: picked
            // 3: next
            let active : Vec<usize> = self.into_iter().skip(1).take(4).collect();
            // Pick the three cups after the current cup
            let picked = &active[0..=2];
            // And then the next cup to be the current next move is the one after that
            let next = active[3];
            // Calculate the designation cup
            let mut destination = if self.current > 1 {self.current - 1} else {max_cup};
            while picked.contains(&destination) {
                destination -= 1;
                if destination < 1 {
                    destination = max_cup;
                }
            }

            // Update indexes
            // From: current -> picked -> next -> destination -> rest
            // To  : current -> next -> destination -> picked -> rest
            self.cup_indexes[self.current] = next;
            let rest = self.cup_indexes[destination];
            self.cup_indexes[destination] = picked[0];
            self.cup_indexes[picked[2]] = rest;

            // Set current to be the next cup along for the next move
            self.current = next;
        }
    }
}

#[aoc_generator(day23)]
pub fn gen(input: &str) -> Vec<usize> {
    input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
}

#[aoc(day23, part1)]
fn part1(input: &Vec<usize>) -> String {
    // Build a new game using our input 
    let mut game = CrabCups::new(input, 0);

    // Play for 100 turns
    game.play(100);

    // Move round to cup 1
    game.set_current(1);

    // Then skip it and collect the other cups' labels into a single string
    return game.into_iter().skip(1).map(|c| c.to_string()).collect::<Vec<String>>().concat();
}

#[aoc(day23, part2)]
fn part2(input: &Vec<usize>) -> usize {
    // Build a new game using our input + enough cups to make 1 million in total
    let mut game = CrabCups::new(input, 1_000_000);

    // Play for 10 million turns
    game.play(10_000_000);

    // Move round to cup 1
    game.set_current(1);

    // Then skip it and take the next two cups after cup 1 and multiply their values
    return game.into_iter().skip(1).take(2).product();
}
