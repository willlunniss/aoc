use utils::grid::{Direction, VecGrid};

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

/// Uses the list of `instructions` to work out the code for the `keypad`
fn decode(instructions: &[Vec<Direction>], keypad: &VecGrid<char>) -> String {
    // Find the button labelled 5 to start on
    let (mut pos, _) = keypad
        .into_iter()
        .find(|(_, value)| **value == '5')
        .unwrap();
    let mut code = Vec::new();
    for line in instructions {
        // Each line in the instructions corresponds to one button push of the code
        for direction in line {
            let next = pos.next(*direction);
            // Only follow directions that stay within the valid buttons
            pos = match keypad.get(next) {
                Some('1'..='9' | 'A'..='Z') => next,
                _ => pos,
            }
        }
        // Add current button to the code
        code.push(keypad.get(pos).unwrap());
    }
    code.iter().collect::<String>()
}

static KEYPAD_1: &str = "   
123
456
789
";

static KEYPAD_2: &str = "     
  1  
 234 
56789
 ABC 
  D  
";

#[aoc(day2, part1)]
fn part1(input: &[Vec<Direction>]) -> String {
    decode(input, &KEYPAD_1.parse().unwrap())
}

#[aoc(day2, part2)]
fn part2(input: &[Vec<Direction>]) -> String {
    decode(input, &KEYPAD_2.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    ULL
    RRDDD
    LURDL
    UUUUD
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), "1985");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), "5DB3");
    }
}
