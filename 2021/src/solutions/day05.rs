use utils::grid::{MapGrid, Pos};

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<(Pos, Pos)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(&[',', ' '][..]).collect::<Vec<_>>();
            (
                Pos::from((
                    parts[0].parse::<isize>().unwrap(),
                    parts[1].parse().unwrap(),
                )),
                Pos::from((
                    parts[3].parse::<isize>().unwrap(),
                    parts[4].parse().unwrap(),
                )),
            )
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Vec<(Pos, Pos)>) -> usize {
    let mut grid = MapGrid::new();
    for (from, to) in input {
        if from.y == to.y {
            if to.x > from.x {
                for x in from.x..=to.x {
                    grid.entry(Pos::from((x, from.y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            } else {
                for x in (to.x..=from.x) {
                    grid.entry(Pos::from((x, from.y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        } else if from.x == to.x {
            if to.y > from.y {
                for y in from.y..=to.y {
                    grid.entry(Pos::from((from.x, y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            } else {
                for y in (to.y..=from.y) {
                    grid.entry(Pos::from((from.x, y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
    }
    grid.values().filter(|x| **x > 1).count()
}

#[aoc(day5, part2)]
fn part2(input: &Vec<(Pos, Pos)>) -> usize {
    let mut grid = MapGrid::new();
    for (from, to) in input {
        if from.y == to.y {
            if to.x > from.x {
                for x in from.x..=to.x {
                    grid.entry(Pos::from((x, from.y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            } else {
                for x in to.x..=from.x {
                    grid.entry(Pos::from((x, from.y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        } else if from.x == to.x {
            if to.y > from.y {
                for y in from.y..=to.y {
                    grid.entry(Pos::from((from.x, y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            } else {
                for y in to.y..=from.y {
                    grid.entry(Pos::from((from.x, y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        } else {
            // diagonal
            let x_inc = if from.x == to.x {
                0
            } else if from.x > to.x {
                -1
            } else {
                1
            };
            let y_inc = if from.y == to.y {
                0
            } else if from.y > to.y {
                -1
            } else {
                1
            };

            if y_inc == 1 {
                for (i, y) in (from.y..=to.y).enumerate() {
                    grid.entry(Pos::from((from.x + (i as isize * x_inc), y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            } else if y_inc == -1 {
                for (i, y) in (to.y..=from.y).enumerate() {
                    grid.entry(Pos::from((to.x + (i as isize * x_inc * -1), y)))
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                }
            }
        }
    }
    grid.values().filter(|x| **x > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 12);
    }
}
