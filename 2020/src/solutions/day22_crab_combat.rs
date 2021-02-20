use std::str::FromStr;
use std::convert::Infallible;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    deck: VecDeque<usize>,
}

impl FromStr for Player {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Player {
            deck: s.lines().skip(1).map(|card| card.parse::<usize>().unwrap()).collect::<VecDeque<usize>>(),
        });
    }
}

impl Player {
    /// Creates a new deck using the first size cards from the current one
    pub fn copy_deck(&self, size: usize) -> VecDeque<usize> {
        self.deck.iter().take(size).map(|card| *card).collect::<VecDeque<usize>>()
    }

    /// Creates a representation of the current deck that can be used to
    /// check we aren't playing the same decks again and again
    pub fn deck_img(&self) -> String {
        // TODO: Use something better than a String?
        self.deck.iter().map(|c| c.to_string()).collect::<Vec<String>>().concat()
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Game {
    players: Vec<Player>
}

impl Game {
    /// Creates a new sub game using the first deck_sizes number of cards for each player
    pub fn new_sub_game(&self, deck_sizes: [usize; 2]) -> Game {
        Game {
            players: [
                Player{ deck: self.players[0].copy_deck(deck_sizes[0])},
                Player{ deck: self.players[1].copy_deck(deck_sizes[1])},            
            ].to_vec()
        }
    }

    /// Plays a normal game of combat until someone wins
    pub fn play(&mut self) {
        while self.winner().is_none() {
            let (card1, card2) = self.draw();
            if card1 > card2 {
                self.complete_round(0, [card1, card2]);
            } else {
                self.complete_round(1, [card2, card1]);
            }
        }
    }

    /// Plays a recursive game of combat until someone wins
    pub fn play_recursive(&mut self) -> usize {
        let mut previous_hands = vec![HashSet::new(); 2];
        while self.winner().is_none() {
            // Check we haven't seen these hands before
            let deck1 = self.players[0].deck_img();
            let deck2 = self.players[1].deck_img();
            if previous_hands[0].contains(&deck1) || previous_hands[1].contains(&deck2) {
                // Already seen this set of decks before, player 1 immediately wins
                return 0;
            } else {
                // Store the hands for later comparison
                previous_hands[0].insert(deck1);
                previous_hands[1].insert(deck2);
            }

            // Draw cards and work out how to play the round
            let (card1, card2) = self.draw();
            let winner = if self.players[0].deck.len() >= card1 && self.players[1].deck.len() >= card2 {
                // Start a new sub game
                let mut sub_game = self.new_sub_game([card1, card2]);
                sub_game.play_recursive()
            } else {
                // Play a normal round - highest card wins
                if card1 > card2 { 0 } else { 1 }
            };
            // Add the winner and then the losers cards to the winners deck
            if winner == 0 {
                self.complete_round(0, [card1, card2]);
            } else {
                self.complete_round(1, [card2, card1]);
            }
        }
        return self.winner().unwrap();
    }

    /// Returns the winner of the game (or None if no one has won yet)
    pub fn winner(&self) -> Option<usize> {
        if self.players[0].deck.len() == 0 {
            Some(1)
        } else if self.players[1].deck.len() == 0 {
            Some(0)
        } else {
            None
        }
    }

    /// Returns the score of the winning player
    pub fn winning_score(&self) -> usize {
        // Reverse the cards and then multiply by 1..N and sum up
        return self.players[self.winner().unwrap()].deck.iter().rev().zip(1..).map(|(card, score)| *card * score).sum();
    }

    /// Draws the next round of cards
    pub fn draw(&mut self) -> (usize, usize) {
        (self.players[0].deck.pop_front().unwrap(), self.players[1].deck.pop_front().unwrap())
    }

    /// Completes a round by adding the cards to the end of the winning players deck
    pub fn complete_round(&mut self, winner: usize, cards: [usize; 2]) {
        cards.iter().for_each(|card| self.players[winner].deck.push_back(*card));
    }
}

#[aoc_generator(day22)]
pub fn gen(input: &str) -> Game {
    return Game{ players: input.split("\r\n\r\n").map(|player| player.parse().unwrap()).collect::<Vec<Player>>() };
}

#[aoc(day22, part1)]
fn part1(input: &Game) -> usize {
    let mut game = input.clone();
    game.play();
    return game.winning_score();
}

#[aoc(day22, part2)]
fn part2(input: &Game) -> usize {
    let mut game = input.clone();
    game.play_recursive();
    return game.winning_score();
}
