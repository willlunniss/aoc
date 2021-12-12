use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Cave<'a> {
    to: Vec<&'a str>,
}

fn gen(input: &str) -> HashMap<&str, Cave> {
    let mut caves: HashMap<&str, Cave> = HashMap::new();
    for (from, to) in input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
    {
        // For each each connected cave record that we can go between them
        caves
            .entry(from)
            .and_modify(|x| x.to.push(to))
            .or_insert(Cave { to: vec![to; 1] });
        caves
            .entry(to)
            .and_modify(|x| x.to.push(from))
            .or_insert(Cave { to: vec![from; 1] });
    }
    caves
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(char::is_lowercase)
}

/// Counts the number of distinct paths from `start` --> `end`
///
/// For each path, big caves can be visited multiple times and small caves can be visited once
/// Exception being is that if `extra_visit` is true, then a single small cave can
/// be visited twice
fn count_paths(caves: &HashMap<&str, Cave>, extra_visit: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut paths = 0;
    queue.push_back(("start", HashSet::new(), !extra_visit));
    while let Some((name, visited_small, used_extra_visit)) = queue.pop_front() {
        // Consider all the places we could move to from this cave
        let cave = caves.get(name).unwrap();
        for next in &cave.to {
            if next == &"start" {
                // don't go back to the start
                continue;
            }
            if next == &"end" {
                // Found a complete path
                paths += 1;
                continue;
            }
            // Haven't been here yet, for this path
            if !is_small_cave(next) {
                // Can visit big caves multiple times, move into it
                queue.push_back((next, visited_small.clone(), used_extra_visit));
            } else if !visited_small.contains(next) {
                // Haven't visited this small cave before, move into it recording that
                // we have visited it
                let mut visited_small = visited_small.clone();
                visited_small.insert(next);
                queue.push_back((next, visited_small, used_extra_visit));
            } else if !used_extra_visit {
                // Have visited this small cave before, but haven't used our extra visit yet
                // Use it up and move into it
                queue.push_back((next, visited_small.clone(), true));
            }
        }
    }
    paths
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    // Count all paths, only visiting small caves once per path
    count_paths(&gen(input), false)
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    // Count all paths, allowing a single extra visit to a small cave per path
    count_paths(&gen(input), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 10);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 36);
    }
}
