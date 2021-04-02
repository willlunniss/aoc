use modinverse::modinverse;
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

    /// Transforms the technique into a linear congruential function of form f(x) = ax + b mod m
    /// Returns (a, b)
    ///
    /// Based on <https://codeforces.com/blog/entry/72593>
    fn lcf(&self) -> (i128, i128) {
        match self {
            Self::DealIntoNewStack => (-1, -1),
            Self::Cut(cards) => (1, -cards as i128),
            Self::DealWithIncrement(increment) => (*increment as i128, 0),
        }
    }
}

/// Composes the supplied techniques as lcfs into a single lcf given the supplied deck size
fn compose_techniques_lcfs(techniques: &[Technique], deck_size: i128) -> (i128, i128) {
    techniques
        .iter()
        .fold((1, 0), |acc, x| compose_lcfs(acc, x.lcf(), deck_size))
}

/// Composes two lcf given deck size m
///
/// Turning f(x)=ax+b  mod m  and g(x)=cx+d  mod m into
/// (a,b) ;(c,d)=(ac mod m,bc+d  mod m)
/// Based on <https://codeforces.com/blog/entry/72593>
fn compose_lcfs(f: (i128, i128), g: (i128, i128), m: i128) -> (i128, i128) {
    let (a, b) = f;
    let (c, d) = g;
    ((a * c).rem_euclid(m), (b * c + d).rem_euclid(m))
}

/// Composes a lcf k times based on deck size m using exponentiation by squaring
///
/// Based on <https://codeforces.com/blog/entry/72593>
fn pow_compose(f: (i128, i128), k: i128, m: i128) -> (i128, i128) {
    let mut g = (1, 0);
    let mut k = k;
    let mut f = f;
    while k > 0 {
        if k % 2 != 0 {
            g = compose_lcfs(g, f, m);
        }
        k /= 2;
        f = compose_lcfs(f, f, m);
    }
    g
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

/// Simple version of part 1 that shuffles the whole deck
#[aoc(day22, part1, full_deck)]
fn part1_full_deck(input: &[Technique]) -> usize {
    // Build a standard deck of space cards
    let deck = (0..10_007).collect::<Vec<usize>>();
    // Shuffle it based on the input
    let shuffled = shuffle(deck, input);
    // Return the position of card 2019
    shuffled.iter().position(|&card| card == 2019).unwrap()
}

/// Optimised version of part 1 that shuffles a single card
#[aoc(day22, part1, single_card)]
fn part1_single_card(input: &[Technique]) -> usize {
    // Shuffle just card 2019 and see where it ends up
    shuffle_card(2019, 10_007, input)
}

/// Version of part 1 using modular arithmetic based on <https://codeforces.com/blog/entry/72593>
#[aoc(day22, part1, modular_arithmetic)]
fn part1_modular_arithmetic(input: &[Technique]) -> i128 {
    // Compose the lcfs for all techniques based on our deck size into a single lcf
    let deck_size = 10_007;
    let (a, b) = compose_techniques_lcfs(input, deck_size);
    // Calculate where card 2019 will end up by computing f(x)=ax+b  mod m
    (a * 2019 + b).rem_euclid(deck_size)
}

/// Part 2 using modular arithmetic based on <https://codeforces.com/blog/entry/72593>
#[aoc(day22, part2)]
fn part2_modular_arithmetic(input: &[Technique]) -> i128 {
    // Compose the lcfs for all techniques based on our deck size into a single lcf
    let deck_size = 119_315_717_514_047;
    let (a, b) = compose_techniques_lcfs(input, deck_size);
    // Compose them passes times into a new single lcf
    let passes = 101_741_582_076_661;
    let (a, b) = pow_compose((a, b), passes, deck_size);
    // Calculate what cards ends up at 2020 by calculating the inverse
    ((2020 - b) * modinverse(a, deck_size).unwrap()).rem_euclid(deck_size)
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
