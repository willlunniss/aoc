#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[isize]) -> Option<usize> {
    let mut jumps = input.to_vec();
    let mut pos: isize = 0;
    for step in 1.. {
        // Get the jump value
        let jump = jumps[pos as usize];
        // Increment it for next time
        jumps[pos as usize] = jump + 1;
        // Move
        pos += jump;
        // See if we jumped out of range
        if pos < 0 || pos >= jumps.len() as isize {
            return Some(step);
        }
    }
    None
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> Option<usize> {
    let mut jumps = input.to_vec();
    let mut pos: isize = 0;
    for step in 1.. {
        // Get the jump value
        let jump = jumps[pos as usize];
        // Adjust it for next time
        jumps[pos as usize] = jump + if jump >= 3 { -1 } else { 1 };
        // Move
        pos += jump;
        // See if we jumped out of range
        if pos < 0 || pos >= jumps.len() as isize {
            return Some(step);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&[0, 3, 0, 1, -3]), Some(5));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&[0, 3, 0, 1, -3]), Some(10));
    }
}
