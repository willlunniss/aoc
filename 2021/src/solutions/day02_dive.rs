use strum_macros::EnumString;
use utils::grid::{Direction, Pos};

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Action {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Command {
    action: Action,
    units: isize,
}

impl Command {
    fn from(s: &str) -> Self {
        let parts = s.split(' ').collect::<Vec<_>>();
        Self {
            action: parts[0].parse().unwrap(),
            units: parts[1].parse().unwrap(),
        }
    }
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Command> {
    input.lines().map(|line| Command::from(line)).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Command]) -> isize {
    let mut pos = Pos::new(0, 0);
    for command in input {
        // Process each command by updating the position of the submarine
        let direction = match command.action {
            Action::Forward => Direction::Right,
            Action::Down => Direction::Down,
            Action::Up => Direction::Up,
        };
        pos = pos.next_by(direction, command.units);
    }
    pos.x * pos.y
}

#[aoc(day2, part2)]
fn part2(input: &[Command]) -> isize {
    let mut aim = 0;
    let mut pos = Pos::new(0, 0);
    for command in input {
        // Process each command by updating the aim and position of the submarine
        match command.action {
            Action::Down => aim += command.units,
            Action::Up => aim -= command.units,
            Action::Forward => {
                pos = pos + (command.units, aim * command.units);
            }
        };
    }
    pos.x * pos.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 150);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 900);
    }
}
