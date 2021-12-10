use nalgebra::Matrix5;
use std::collections::HashSet;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Copy, Clone)]
struct Board {
    grid: Matrix5<usize>,
}

#[derive(Debug)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl FromStr for Bingo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the input by blank lines into sections
        let mut sections = s.split("\n\n");

        // First section has the list of numbers to draw
        let numbers = sections
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        // Remaining sections has the boards
        let boards = sections.map(|section| section.parse().unwrap()).collect();

        Ok(Self { numbers, boards })
    }
}

impl FromStr for Board {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Read in the grid into the 5x5 matrix value by value
        Ok(Self {
            grid: Matrix5::from_iterator(s.split_ascii_whitespace().map(|x| x.parse().unwrap())),
        })
    }
}

impl Board {
    /// Calculates the score for the board based on the
    /// drawn numbers and the last drawn number
    fn score(self, drawn: &HashSet<usize>, last: usize) -> usize {
        // Sum up the undrawn numbers and multiply by the last drawn
        self.grid
            .iter()
            .filter(|n| !drawn.contains(n))
            .sum::<usize>()
            * last
    }

    /// Checks if a board has won given the drawn numbers
    fn wins(self, drawn: &HashSet<usize>) -> bool {
        // Check for any row or any column being fully drawn
        self.grid
            .row_iter()
            .any(|row| row.iter().all(|n| drawn.contains(n)))
            || self
                .grid
                .column_iter()
                .any(|col| col.iter().all(|n| drawn.contains(n)))
    }
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Bingo {
    input.parse().unwrap()
}

#[aoc(day4, part1)]
fn part1(input: &Bingo) -> Option<usize> {
    let mut drawn = HashSet::new();

    for &number in &input.numbers {
        // Draw the number
        drawn.insert(number);

        // Check to see if any boards have won
        if let Some(winner) = input.boards.iter().find(|board| board.wins(&drawn)) {
            // and return their score if they have
            return Some(winner.score(&drawn, number));
        }
    }
    None
}

#[aoc(day4, part2)]
fn part2(input: &Bingo) -> Option<usize> {
    let mut active = input.boards.clone();
    let mut drawn = HashSet::new();

    for &number in &input.numbers {
        // Draw the number
        drawn.insert(number);

        if active.len() == 1 {
            // If last board, return it's score if it won
            let last = active.first().unwrap();
            if last.wins(&drawn) {
                return Some(last.score(&drawn, number));
            }
        } else {
            // Otherwise remove winners
            active.retain(|board| !board.wins(&drawn));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), Some(4512));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), Some(1924));
    }
}
