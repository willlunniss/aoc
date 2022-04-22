use std::str::FromStr;

use utils::grid::{Pos, VecGrid};

type Light = u32;

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    /// Applies an action using the rules from p1 to a light
    fn apply_p1(&self, light: &mut Light) {
        match self {
            Self::TurnOn => *light = 1,
            Self::TurnOff => *light = 0,
            Self::Toggle => *light ^= 1,
        }
    }

    /// Applies an action using the rules from p2 to a light
    fn apply_p2(&self, light: &mut Light) {
        match self {
            Self::TurnOn => *light += 1,
            Self::TurnOff => {
                if *light > 0 {
                    *light -= 1;
                }
            }
            Self::Toggle => *light += 2,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    start: Pos,
    end: Pos,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Work out the action
        let action = if s.starts_with("turn on") {
            Action::TurnOn
        } else if s.starts_with("turn off") {
            Action::TurnOff
        } else if s.starts_with("toggle") {
            Action::Toggle
        } else {
            unreachable!();
        };
        // Extract the start/end positions
        let points = s
            .split(&[' ', ','])
            .flat_map(str::parse)
            .collect::<Vec<usize>>();
        let start = Pos::new(points[0], points[1]);
        let end = Pos::new(points[2], points[3]);
        Ok(Self { action, start, end })
    }
}

/// Returns a grid lit using the instructions and action apply function
fn light_grid(
    instructions: &[Instruction],
    apply_fn: impl Fn(&Action, &mut Light),
) -> VecGrid<Light> {
    // Init the grid
    let mut grid = VecGrid::new_sized(0, 1000, 1000);
    for instr in instructions {
        // For each instruction
        for y in instr.start.y..=instr.end.y {
            for x in instr.start.x..=instr.end.x {
                // Update all lights within the start/end sub-grid
                apply_fn(&instr.action, &mut grid[Pos::from((x, y))]);
            }
        }
    }
    grid
}

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Instruction]) -> u32 {
    // Light the grid using p1 rules and then count the number of lights that are on (1)
    light_grid(input, Action::apply_p1).values().sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Instruction]) -> u32 {
    // Light the grid using p2 rules and then calculate the total brightness across all lights
    light_grid(input, Action::apply_p2).values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    turn on 0,0 through 999,999
    toggle 0,0 through 999,0
    turn off 499,499 through 500,500
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 998_996);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen("turn on 0,0 through 0,0")), 1);
        assert_eq!(part2(&gen("toggle 0,0 through 999,999")), 2_000_000);
    }
}
