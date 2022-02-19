use std::collections::VecDeque;

#[aoc_generator(day19)]
fn gen(input: &str) -> u32 {
    input.parse::<u32>().unwrap()
}

#[aoc(day19, part1)]
const fn part1(elves: &u32) -> u32 {
    // k=2 special case of the https://en.wikipedia.org/wiki/Josephus_problem
    let highest = 32 - elves.leading_zeros();
    !2u32.pow(highest) & (*elves << 1 | 1)
}

#[aoc(day19, part2)]
fn part2(elves: &u32) -> u32 {
    // Arrange the elves into two lists of elves to the left and the right of the current elf
    // (with the current elf included at the front of the left list)
    let mut left = (1..=(elves / 2)).collect::<VecDeque<_>>();
    let mut right = ((elves / 2) + 1..=*elves).rev().collect::<VecDeque<_>>();
    while left.len() > 1 {
        // Remove the elf that is opposite (back of the list), favouring the left if there are two opposite
        // (> rather than >= because the left list also contains the current elf)
        if left.len() > right.len() {
            left.pop_back().unwrap()
        } else {
            right.pop_back().unwrap()
        };
        // Move to the next elf
        // Transfer the current elf from the left to the right list
        right.push_front(left.pop_front().unwrap());
        // Transfer the elf that was to the right of the one that was removed to the back of the left
        left.push_back(right.pop_back().unwrap());
    }

    left[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&5), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&5), 2);
        assert_eq!(part2(&65535), 6486);
    }
}
