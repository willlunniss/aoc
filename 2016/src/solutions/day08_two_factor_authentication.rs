use std::str::FromStr;
use utils::ocr::OcrString;

#[derive(Debug, PartialEq)]
enum Instruction {
    DrawRect { x: usize, y: usize },
    RotateRow { y: usize, pixels: usize },
    RotateColumn { x: usize, pixels: usize },
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "rect" => {
                let (x, y) = parts[1].split_once('x').unwrap();
                Ok(Self::DrawRect {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                })
            }
            "rotate" => {
                let (_, index) = parts[2].split_once('=').unwrap();
                match parts[1] {
                    "row" => Ok(Self::RotateRow {
                        y: index.parse().unwrap(),
                        pixels: parts[4].parse().unwrap(),
                    }),
                    "column" => Ok(Self::RotateColumn {
                        x: index.parse().unwrap(),
                        pixels: parts[4].parse().unwrap(),
                    }),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

/// Renders the display using the `instructions`
fn render(instructions: &[Instruction], width: usize, height: usize) -> Vec<Vec<char>> {
    let mut display: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for instruction in instructions {
        match instruction {
            Instruction::DrawRect { x, y } => {
                for y in 0..*y {
                    for x in 0..*x {
                        display[y][x] = '#';
                    }
                }
            }
            Instruction::RotateRow { y, pixels } => {
                display[*y].rotate_right(*pixels);
            }
            Instruction::RotateColumn { x, pixels } => {
                let col = display.iter().map(|row| row[*x]).collect::<Vec<_>>();
                for (i, c) in col.iter().enumerate() {
                    display[(i + *pixels) % height][*x] = *c;
                }
            }
        }
    }
    display
}

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> usize {
    // Render the display and count the on pixels
    render(input, 50, 6)
        .iter()
        .map(|row| row.iter().filter(|x| **x == '#').count())
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> OcrString {
    // Render the display and then extract the message
    render(input, 50, 6).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    rect 3x2
    rotate column x=1 by 1
    rotate row y=0 by 4
    rotate column x=1 by 1
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(
            render(&gen(EXAMPLE_INPUT), 7, 3)
                .iter()
                .map(|row| row.iter().filter(|x| **x == '#').count())
                .sum::<usize>(),
            6
        );
    }
}
