use itertools::Itertools;
use utils::grid::Pos;
use utils::ocr::{OcrString, Point};

type FoldInstruction = (Axis, isize);

#[derive(Debug, Clone)]
struct Origami {
    points: Vec<Pos>,
    folds: Vec<FoldInstruction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Axis {
    Y,
    X,
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Origami {
    // Parse the input as a list of points and then a list of fold instructions
    let (points, folds) = input.split_once("\n\n").unwrap();
    Origami {
        points: points.lines().map(|line| line.parse().unwrap()).collect(),
        folds: folds
            .lines()
            .map(|line| {
                let (command, value) = line.split_once("=").unwrap();
                if command.ends_with('x') {
                    (Axis::X, value.parse().unwrap())
                } else {
                    (Axis::Y, value.parse().unwrap())
                }
            })
            .collect(),
    }
}

/// Folds a position over a line
fn origami_fold(pos: Pos, fold: &FoldInstruction) -> Pos {
    if fold.0 == Axis::Y && pos.y > fold.1 {
        Pos::from((pos.x, fold.1 - (pos.y - fold.1)))
    } else if fold.0 == Axis::X && pos.x > fold.1 {
        Pos::from((fold.1 - (pos.x - fold.1), pos.y))
    } else {
        // No folding needed
        pos
    }
}

#[aoc(day13, part1)]
fn part1(input: &Origami) -> usize {
    input
        .points
        .iter()
        .map(|&pos| origami_fold(pos, &input.folds[0]))
        .unique()
        .count()
}

#[aoc(day13, part2)]
fn part2(input: &Origami) -> String {
    input
        .points
        .iter()
        .map(|&pos| input.folds.iter().fold(pos, |pos, x| origami_fold(pos, x)))
        .unique()
        .map(|pos| Point::try_from(pos).unwrap())
        .collect::<OcrString>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 17);
    }
}
