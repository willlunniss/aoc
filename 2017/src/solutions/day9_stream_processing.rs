#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut level = 0;
    let mut score = 0;
    let mut in_garbage = false;
    let mut canceled = false;
    // For each char in the stream, process as per rules
    for c in input.chars() {
        if canceled {
            canceled = false;
        } else if c == '!' {
            canceled = true;
        } else if in_garbage {
            if c == '>' {
                in_garbage = false;
            }
        } else if c == '<' {
            in_garbage = true;
        } else if c == '{' {
            // Increase level each time we open a new group
            level += 1;
        } else if c == '}' {
            // Increment score and decrease level each time we close one
            score += level;
            level -= 1;
        }
    }

    score
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut in_garbage = false;
    let mut canceled = false;
    // For each char in the stream, process as per rules
    for c in input.chars() {
        if canceled {
            canceled = false;
        } else if c == '!' {
            canceled = true;
        } else if in_garbage {
            if c == '>' {
                in_garbage = false;
            } else {
                // Count all chars contained within the garbage (excluding cancelled chars)
                count += 1;
            }
        } else if c == '<' {
            in_garbage = true;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2("<{!>}>"), 2);
        assert_eq!(part2("<{o\"i!a,<{i<a>"), 10);
    }
}
