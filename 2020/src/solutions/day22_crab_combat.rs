use std::str::FromStr;
use std::convert::Infallible;
use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone)]
pub struct Player {
    name: String,
    deck: VecDeque<usize>,
}

impl FromStr for Player {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Player {
            name: s.lines().nth(0).unwrap().to_string(),
            deck: s.lines().skip(1).map(|card| card.parse::<usize>().unwrap()).collect::<VecDeque<usize>>(),
        });
    }
}

#[aoc_generator(day22)]
pub fn gen(input: &str) -> Vec<Player> {
    return input.split("\r\n\r\n").map(|player| player.parse().unwrap()).collect::<Vec<Player>>()
}

#[aoc(day22, part1)]
fn part1(input: &Vec<Player>) -> usize {
    let mut player1 = input[0].clone();
    let mut player2 = input[1].clone();
    while player1.deck.len() > 0 && player2.deck.len() > 0 {
        let card1 = player1.deck.pop_front().unwrap();
        let card2 = player2.deck.pop_front().unwrap();
        if card1 > card2 {
            player1.deck.push_back(card1);
            player1.deck.push_back(card2);
        } else {
            player2.deck.push_back(card2);
            player2.deck.push_back(card1);
        }
    }
    let winner = if player1.deck.len() > 0 {
        player1
    } else {
        player2
    };
    // Reverse the cards and then multiply by 1..N and sum up
    return winner.deck.iter().rev().zip(1..).map(|(card, score)| *card * score).sum();
}

#[aoc(day22, part2)]
fn part2(input: &Vec<Player>) -> usize {
    
    return 0;
}
