
#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Vec<usize>) -> usize {
    // Naive approach - simulate what happens one day at a time
    let mut fishes = input.clone();
    for _day in 0..80 {
        let mut next = Vec::new();
        for fish in fishes {
            // For each fish, if their timer is at 0 create a new one at 8 and add them at 6
            if fish == 0 {
                next.push(6);
                next.push(8);
            } else {
                // For all other values add -1 current timer
                next.push(fish - 1);
            }
        }
        fishes = next;
    }
    fishes.len()
}

#[aoc(day6, part2)]
fn part2(input: &Vec<usize>) -> usize {
    // Naive approach of simulating one day at a time isn't feasible for the amount of data in part 2
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"3,4,3,1,2"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 5934);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 26_984_457_539);
    }
}