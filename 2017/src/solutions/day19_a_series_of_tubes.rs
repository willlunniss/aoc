use utils::grid::{Direction, VecGrid};

#[aoc_generator(day19)]
fn gen(input: &str) -> VecGrid<char> {
    input.parse().unwrap()
}

fn route(map: &VecGrid<char>) -> (usize, String) {
    let mut letters = Vec::new();
    let mut steps = 0;
    // Find the start
    let (start, _) = map.into_iter().find(|(_, x)| **x == '|').unwrap();
    let mut direction = Direction::Down;
    let mut pos = start;
    loop {
        // Advance by 1 in the current direction
        pos = pos.next(direction);
        steps += 1;
        match map.get(pos) {
            Some(letter @ 'A'..='Z') => letters.push(letter),
            Some('+') => {
                // Change direction
                for dir in &[direction.rotate_left(), direction.rotate_right()] {
                    if !matches!(map.get(pos.next(*dir)), Some(' ') | None) {
                        direction = *dir;
                        break;
                    }
                }
            }
            Some(' ') | None => break, // End
            Some('|' | '-') => {}      // Just keep go
            _ => unreachable!(),
        }
    }

    (steps, letters.iter().collect())
}

#[aoc(day19, part1)]
fn part1(input: &VecGrid<char>) -> String {
    let (_, letters) = route(input);
    letters
}

#[aoc(day19, part2)]
fn part2(input: &VecGrid<char>) -> usize {
    let (steps, _) = route(input);
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "    |          
    |  +--+    
    A  |  C    
F---|----E|--+ 
    |  |  |  D 
    +B-+  +--+ 
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), "ABCDEF");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 38);
    }
}
