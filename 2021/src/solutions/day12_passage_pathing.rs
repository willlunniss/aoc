use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Cave<'a> {
    to: Vec<&'a str>,
}

fn gen(input: &str) -> HashMap<&str, Cave> {
    let mut caves : HashMap<&str, Cave> = HashMap::new();
    for (from, to) in input
        .lines()
        .map(|line| line.split('-').collect_tuple().unwrap())
    {
        caves.entry(from).and_modify(|x| x.to.push(to)).or_insert(Cave{to: vec![to; 1]});
        caves.entry(to).and_modify(|x| x.to.push(from)).or_insert(Cave{to: vec![from; 1]});
    }

    caves
}

fn is_small_cave(name: &str) -> bool {
    name.chars().all(char::is_lowercase)
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    let caves = gen(input);
    let mut queue = VecDeque::new();
    let mut paths = Vec::new();
    queue.push_back(("start", vec!["start"; 1], HashSet::new()));
    while let Some((name, route, visited_small)) = queue.pop_front() {
        // Get the cave
        let cave = caves.get(name).unwrap();
        for next in &cave.to {
            let mut route = route.clone();
            if next == &"start" {
                // don't go back to the start
                continue;
            } 
            if next == &"end" {
                // Found a complete route
                route.push(next);
                paths.push(route);
                continue;
            } 
            // Haven't been here yet, for this route
            if !is_small_cave(next) {
                // Move to it
                //route.push(next);
                queue.push_back((next, route, visited_small.clone()));
            } else if !visited_small.contains(next) {
                // Move to it
                //route.push(next);
                let mut visited_small_next = visited_small.clone();
                visited_small_next.insert(next);
                queue.push_back((next, route, visited_small_next));
            }
        }
    }
    paths.len()
}

#[aoc(day12, part2)]
fn part2(input: &str) -> usize {
    let caves = gen(input);
    let mut queue = VecDeque::new();
    let mut paths = Vec::new();
    queue.push_back(("start", vec!["start"; 1], HashMap::new(), false));
    while let Some((name, route, visited_small, used_double_visit)) = queue.pop_front() {
        // Get the cave
        let cave = caves.get(name).unwrap();
        for next in &cave.to {
            let mut route = route.clone();
            if next == &"start" {
                // don't go back to the start
                continue;
            } 
            if next == &"end" {
                // Found a complete route
                route.push(next);
                paths.push(route);
                continue;
            } 
            // Haven't been here yet, for this route
            if !is_small_cave(next) {
                // Move to it
                route.push(next);
                queue.push_back((next, route, visited_small.clone(), used_double_visit));
            } else {
                let mut visited_small_next = visited_small.clone();
                let visits = visited_small_next.entry(next).and_modify(|x| { *x += 1 }).or_insert(1);
                if *visits < 2 || (!used_double_visit && *visits < 3) {
                    // Move to it
                    route.push(next);
                    let mut have_now_used_it = used_double_visit;
                    if *visits > 1 {
                        have_now_used_it = true;
                    }
                    queue.push_back((next, route, visited_small_next, have_now_used_it));
                }
            }
        }
    }
    paths.len()
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
