use std::collections::HashMap;

/// Returns the frequencies of characters in each column across all messages
fn frequencies(messages: &str) -> Vec<HashMap<char, usize>> {
    let mut frequencies = vec![HashMap::new(); messages.lines().next().unwrap().len()];
    for message in messages.lines() {
        for (column, c) in message.chars().enumerate() {
            *frequencies[column].entry(c).or_insert(0) += 1;
        }
    }
    frequencies
}

/// Recovers a message using the supplied `decoder` that selects a single character
/// given a `HashMap<char, usize>` of character frequencies for a column
fn recover(messages: &str, decoder: impl Fn(&HashMap<char, usize>) -> Option<char>) -> String {
    frequencies(messages).iter().filter_map(decoder).collect()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> String {
    // Recover the message using the most common char for each column
    recover(input, |x| {
        x.iter().max_by_key(|(_, count)| **count).map(|(c, _)| *c)
    })
}

#[aoc(day6, part2)]
fn part2(input: &str) -> String {
    // Recover the message using the least common char for each column
    recover(input, |x| {
        x.iter().min_by_key(|(_, count)| **count).map(|(c, _)| *c)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    eedadn
    drvtee
    eandsr
    raavrd
    atevrs
    tsrnev
    sdttsa
    rasrtv
    nssdts
    ntnada
    svetve
    tesnvt
    vntsnd
    vrdear
    dvrsen
    enarar
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), "easter");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), "advent");
    }
}
