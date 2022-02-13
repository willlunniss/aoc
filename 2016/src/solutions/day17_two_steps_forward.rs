use indoc::indoc;
use std::collections::{HashSet, VecDeque};
use utils::grid::{Direction, Pos, VecGrid};

static GRID: &str = indoc! {"
#########
#S| | | #
#-#-#-#-#
# | | | #
#-#-#-#-#
# | | | #
#-#-#-#-#
# | | |  
####### V
"};

#[derive(Debug, Clone)]
struct State {
    pos: Pos,
    path: Vec<Direction>,
}

impl State {
    /// Returns the next `State` after moving in `Direction`
    fn next(&self, direction: Direction) -> Self {
        let mut next = self.clone();
        next.pos = next.pos.next_by(direction, 2);
        next.path.push(direction);
        next
    }

    fn full_path(&self) -> String {
        self.path.iter().map(|d| char::from(*d)).collect::<String>()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            pos: Pos::new(1, 1),
            path: Vec::new(),
        }
    }
}

/// Calculates the directions of the doors that are unlocked based on the
/// passcode and the current path that has been taken
fn unlocked(passcode: &str, path: &[Direction]) -> HashSet<Direction> {
    // First 4 chars of the MD5 hash if b-f give access to UDLR
    format!(
        "{:x}",
        md5::compute(format!(
            "{passcode}{}",
            path.iter().map(|d| char::from(*d)).collect::<String>()
        ))
    )
    .chars()
    .take(4)
    .zip(Direction::all())
    .filter_map(|(c, d)| (c > 'a').then(|| d))
    .collect()
}

#[aoc(day17, part1)]
fn part1(input: &str) -> String {
    let grid: VecGrid<char> = GRID.parse().unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(State::default());
    while let Some(state) = queue.pop_front() {
        // Work out where we could go based on the current state
        let unlocked = unlocked(input, &state.path);
        for (direction, _, value) in grid.neighbours_ex(state.pos) {
            match value {
                Some('|' | '-') => {
                    if unlocked.contains(&direction) {
                        // Move through the unlocked door
                        queue.push_back(state.next(direction));
                    }
                }
                Some(' ') => return state.full_path(),
                _ => {}
            }
        }
    }
    "?".to_owned()
}

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    let grid: VecGrid<char> = GRID.parse().unwrap();
    let mut queue = VecDeque::new();
    queue.push_back(State::default());
    let mut paths = Vec::new();
    while let Some(state) = queue.pop_front() {
        if grid.neighbours(state.pos).any(|value| value == Some(' ')) {
            // Found a valid path to the vault, make a note of it's length
            // and then stop processing any paths from this state
            // (but continue searching from other states)
            paths.push(state.path.len());
            continue;
        }
        // Not completed yet, see where we could go next
        let unlocked = unlocked(input, &state.path);
        for direction in grid
            .neighbours_ex(state.pos)
            .filter_map(|(d, _, v)| (v != Some('#')).then(|| d))
            .filter(|d| unlocked.contains(d))
        {
            queue.push_back(state.next(direction));
        }
    }
    *paths.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("ihgpwlah"), "DDRRRD");
        assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("ihgpwlah"), 370);
        assert_eq!(part2("kglvqrro"), 492);
        assert_eq!(part2("ulqzkmiv"), 830);
    }
}
