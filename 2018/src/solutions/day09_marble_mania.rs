/// Plays the marble game for the set number of players and marbles and returns the highest score
fn play_marble_game(players: usize, marbles: usize) -> usize {
    // Store player scores
    let mut scores = vec![0; players];
    // Allocate an array to store the placed marbles
    // As we need fast insertion and removals, we effectively create a linked list in our array where each marbles
    // value is treated as an index into the array and the value at that index is the value of the marble
    // clockwise next to it e.g. 0 4 2 1 3 --> 4 3 1 0 2
    let mut placed = vec![0; marbles + 1];
    // Every 23 turns we need to remove current - 7 but as we only store the value of the next marble
    // this would require us to loop around all the placed marbles which gets increasingly expensive
    // To avoid this we also store the last 9 (7 + 1 for current and 1 for the one that points to -7th) values in a circular buffer
    let hist_size = 9;
    let mut history = vec![0; hist_size];
    let mut hist_pos = 0;
    // Start with player 1 and marble 0
    let mut player = 0;
    let mut current = 0;
    for marble in 1..=marbles {
        // Place each marble
        if marble % 23 == 0 {
            // Multiples of 23 result in removing a marble and assigning points to the current players score
            // Find which marble was just before the -7th one we are about to remove, and which one comes after it
            // No need to try to update history as it will be overwritten by the time we need it again (in another 23 turns)
            let before = history[(hist_pos + hist_size - 7 - 1) % hist_size];
            let remove = placed[before];
            let after = placed[remove];
            // Remove it by point before to after
            placed[before] = after;
            // Update score for the current player
            scores[player] += marble;
            scores[player] += remove;
            // Set current to the one after the one we just removed
            current = after;
        } else {
            // Add a marble 1 after the current
            // Ignoring our how the placed array works, we want to perform the following transform 
            // From:
            //          C  B  A
            // 0  4  2 (5) 1  3
            // To:
            //                C
            // 0  4  2  5  1 (6) 3 
            let before = placed[current];
            let after = placed[before];
            // Add marble between before and after
            placed[before] = marble;
            placed[marble] = after;
            // Update current to be the marble we just placed
            current = marble;
            // Update history
            history[(hist_pos + 1) % hist_size] = before;
            history[(hist_pos + 2) % hist_size] = current;
            hist_pos = (hist_pos + 2) % hist_size;
        }
        // Advance the active player
        player = (player + 1) % players;
    }
    // Return the maximum score
    *scores.iter().max().unwrap()
}

#[aoc_generator(day9)]
fn gen(input: &str) -> (usize, usize) {
    let parts = input.split(' ').collect::<Vec<_>>();
    // Extract the number of players and marbles from the input
    (parts[0].parse().unwrap(), parts[6].parse().unwrap())
}

#[aoc(day9, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (players, marbles) = *input;
    // Play the game normally
    play_marble_game(players, marbles)
}

#[aoc(day9, part2)]
fn part2(input: &(usize, usize)) -> usize {
    let (players, marbles) = *input;
    // Play the game with 100x the number of marbles
    play_marble_game(players, marbles * 100)
}
