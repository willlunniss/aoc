use utils::grid::{MapGrid, Pos};

#[derive(Debug, Clone)]
struct Origami {
    points: Vec<Pos>,
    folds: Vec<(Axis, isize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Axis {
    Y,
    X,
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Origami {
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

fn fold(input: &Origami, single_fold: bool) -> MapGrid<char> {
    static MARKER: char = '█';

    // Build the initial grid
    let mut grid = MapGrid::new();
    for point in &input.points {
        grid.insert(*point, MARKER);
    }

    // Fold it
    for fold in &input.folds {
        if fold.0 == Axis::Y {
            // Find all points past the fold
            let folded = grid
                .keys()
                .filter(|pos| pos.y > fold.1)
                .copied()
                .collect::<Vec<_>>();
            for pos in folded {
                // Remove the old position
                grid.remove(&pos);
                // Add a new one the other side of the fold
                let new_y = fold.1 - (pos.y - fold.1);
                grid.insert(Pos::from((pos.x, new_y)), MARKER);
            }
        } else {
            // Find all points past the fold
            let folded = grid
                .keys()
                .filter(|pos| pos.x > fold.1)
                .copied()
                .collect::<Vec<_>>();
            for pos in folded {
                // Remove the old position
                grid.remove(&pos);
                // Add a new one the other side of the fold
                let new_x = fold.1 - (pos.x - fold.1);
                grid.insert(Pos::from((new_x, pos.y)), MARKER);
            }
        }
        if single_fold {
            break;
        }
    }

    grid
}

#[aoc(day13, part1)]
fn part1(input: &Origami) -> usize {
    fold(input, true).keys().len()
}

#[aoc(day13, part2)]
fn part2(input: &Origami) -> String {
    fold(input, false).print(' ');
    println!();
    "↑ Check the printed image ↑".to_owned()
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

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), "↑ Check the printed image ↑");
    }
}
