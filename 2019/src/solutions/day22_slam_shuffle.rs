use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Technique {
    DealIntoNewStack,
    Cut(isize),
    DealWithIncrement(usize),
}

impl FromStr for Technique {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split string into words and use them to work out the technique:
        // * cut <N>
        // * deal into new stack
        // * deal with incremental <N>
        let parts = s.split(' ').collect::<Vec<&str>>();
        if parts[0] == "cut" {
            Ok(Self::Cut(parts[1].parse().unwrap()))
        } else if parts[1] == "into" {
            Ok(Self::DealIntoNewStack)
        } else {
            Ok(Self::DealWithIncrement(
                parts.last().unwrap().parse().unwrap(),
            ))
        }
    }
}

impl Technique {
    /// Shuffles a card by returning the new index of it after
    /// applying the shuffling technique
    fn shuffle(&self, index: usize, size: usize) -> usize {
        match self {
            Self::DealIntoNewStack => size - index - 1,
            Self::Cut(cards) => ((index + size) as isize - cards) as usize % size,
            Self::DealWithIncrement(increment) => (index * increment) % size,
        }
    }
}

/// Shuffles the deck of cards using the techniques
fn shuffle(deck: Vec<usize>, techniques: &[Technique]) -> Vec<usize> {
    let size = deck.len();
    let mut deck = deck;
    let mut next = vec![0; size];
    for technique in techniques {
        for (index, value) in deck.iter().enumerate() {
            next[technique.shuffle(index, size)] = *value;
        }
        std::mem::swap(&mut deck, &mut next);
    }
    deck
}

/// Shuffles a single card using the techniques
fn shuffle_card(index: usize, deck_size: usize, techniques: &[Technique]) -> usize {
    let mut index = index;
    for technique in techniques.iter() {
        index = technique.shuffle(index, deck_size);
    }
    index
}

#[aoc_generator(day22)]
fn gen(input: &str) -> Vec<Technique> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day22, part1, full_deck)]
fn part1_full_deck(input: &[Technique]) -> usize {
    // Build a standard deck of space cards
    let deck = (0..10_007).collect::<Vec<usize>>();
    // Shuffle it based on the input
    let shuffled = shuffle(deck, input);
    // Return the position of card 2019
    shuffled.iter().position(|&card| card == 2019).unwrap()
}

#[aoc(day22, part1, single_card)]
fn part1_single_card(input: &[Technique]) -> usize {
    // Shuffle just card 2019 and see where it ends up
    shuffle_card(2019, 10_007, input)
}

#[aoc(day22, part2)]
fn part2(input: &[Technique]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technique_cut() {
        let technique = Technique::Cut(3);
        let shuffled = technique.shuffle(2, 10);
        assert_eq!(shuffled, 9);
    }

    #[test]
    fn test_technique_deal_with_increment() {
        let technique = Technique::DealWithIncrement(3);
        let shuffled = technique.shuffle(7, 10);
        assert_eq!(shuffled, 1);
    }

    #[test]
    fn test_technique_deal_into_new_stack() {
        let technique = Technique::DealIntoNewStack;
        let shuffled = technique.shuffle(2, 10);
        assert_eq!(shuffled, 7);
    }

    #[test]
    fn test_sample() {
        let deck = (0..10).collect::<Vec<_>>();
        let techniques = [
            Technique::Cut(6),
            Technique::DealWithIncrement(7),
            Technique::DealIntoNewStack,
        ]
        .to_vec();
        let shuffled = shuffle(deck, &techniques);
        assert_eq!(shuffled, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }
}
