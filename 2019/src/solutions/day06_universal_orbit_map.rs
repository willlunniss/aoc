use itertools::Itertools;
use std::collections::HashMap;

/// Walks the path from object -> COM
fn walk<'a>(orbit_map: &'a HashMap<&str, &str>, object: &'a str) -> Vec<&'a str> {
    let mut path = Vec::new();
    let mut node = object;
    // Keep resolving until there isn't a parent
    while let Some(parent) = orbit_map.get(node) {
        node = parent;
        path.push(node);
    }
    return path;
}

fn gen(input: &str) -> HashMap<&str, &str> {
    return input
        .lines()
        .map(|line| line.splitn(2, ")").collect_tuple().unwrap())
        .map(|(parent, child)| (child, parent))
        .collect();
}

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let orbit_map = gen(input);
    return orbit_map
        .keys()
        .fold(0, |acc, object| acc + walk(&orbit_map, object).len());
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let orbit_map = gen(input);
    // Find the paths needed to get from YOU and SAN to COM
    let you = "YOU".to_owned();
    let san = "SAN".to_owned();
    let path1 = walk(&orbit_map, &you);
    let path2 = walk(&orbit_map, &san);

    // Walk backwards through the paths from COM -> [YOU|SAN]
    // Until we find the divergence point, we can then add up the number of
    // remaining nodes
    let mut shared_path_length = 0;
    for (n1, n2) in path1.iter().rev().zip(path2.iter().rev()) {
        if n1 != n2 {
            // Have diverged!
            break;
        }
        shared_path_length += 1;
    }
    // Orbital transfers from YOU -> SAN is len of the two paths - 2x shared_path_length
    return path1.len() + path2.len() - (2 * shared_path_length);
}
