use std::collections::{HashMap, HashSet};

#[aoc_generator(day12)]
fn gen(input: &str) -> HashMap<usize, Vec<usize>> {
    input
        .lines()
        .map(|line| {
            // Read in each line e.g.
            // 2 <-> 0, 3, 4
            // as from and a list of to values
            let (from, to) = line.split_once(" <-> ").unwrap();
            (
                from.parse().unwrap(),
                to.split(", ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

/// Returns a set containing all of the programs connected to the start program
fn find_connected(programs: &HashMap<usize, Vec<usize>>, start: usize) -> HashSet<usize> {
    let mut connected = HashSet::new();
    // Queue up the start program
    let mut queue = vec![start];
    connected.insert(start);
    while let Some(from) = queue.pop() {
        // For every program in the queue, see what it's connected to
        for connected_to in programs.get(&from).unwrap() {
            // If we haven't visited it yet, then go to it
            if connected.insert(*connected_to) {
                queue.push(*connected_to);
            }
        }
    }
    connected
}

#[aoc(day12, part1)]
fn part1(input: &HashMap<usize, Vec<usize>>) -> usize {
    // Answer is the number of programs that are connected to 0 (inc 0)
    find_connected(input, 0).len()
}

#[aoc(day12, part2)]
fn part2(input: &HashMap<usize, Vec<usize>>) -> usize {
    let mut programs = input.clone();
    let mut groups = 0;
    while !programs.is_empty() {
        // Pick somewhere to start
        let start = programs.keys().next().unwrap();
        // Find everywhere it's connected to
        let group = find_connected(&programs, *start);
        // Remove all the programs that exist in the group
        programs.retain(|x, _| !group.contains(x));
        // Increment group counter
        groups += 1;
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
    0 <-> 2
    1 <-> 1
    2 <-> 0, 3, 4
    3 <-> 2, 4
    4 <-> 2, 3, 6
    5 <-> 6
    6 <-> 4, 5
"
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 6);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 2);
    }
}
