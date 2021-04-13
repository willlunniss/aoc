use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

/// Recursively find all the bag that can ultimately contain the supplied bag
fn resolve_containers(
    mappings: &HashMap<&str, HashSet<&str>>,
    bag: &str,
    solutions: &mut HashSet<String>,
) {
    let containers = mappings.get(bag).unwrap();
    for container in containers.iter() {
        solutions.insert((*container).to_string());
        if mappings.contains_key(container) {
            resolve_containers(mappings, container, solutions);
        }
    }
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let mut mappings: HashMap<&str, HashSet<&str>> = HashMap::new();
    let regex = Regex::new(r"(\d+)\s(\S+\s\S+)").unwrap();
    // Each rule looks something like:
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // <colour> bags contains (<quantity> <colour> bags)+
    for rule in input.lines() {
        let parts: Vec<&str> = rule.split(" bags contain ").collect();
        let outer_bag = parts[0];
        for item in parts[1].split(", ").collect::<Vec<&str>>() {
            if item == "no other bags." {
                break;
            }
            let capture = regex.captures(item).unwrap();
            let colour = capture.get(2).unwrap().as_str();
            if mappings.contains_key(colour) {
                mappings.get_mut(colour).unwrap().insert(outer_bag);
            } else {
                let mut set = HashSet::new();
                set.insert(outer_bag);
                mappings.insert(colour, set);
            }
        }
    }
    let bag = "shiny gold";
    let mut solutions: HashSet<String> = HashSet::new();
    resolve_containers(&mappings, bag, &mut solutions);
    solutions.len()
}

/// Recursively counts the number of bags contained within the specified bag
fn count_bags(contents: &HashMap<&str, HashSet<(&str, usize)>>, bag: &str) -> usize {
    let mut count = 0;
    let bags = contents.get(bag).unwrap();
    for (colour, quantity) in bags.iter() {
        if contents.contains_key(colour) {
            // Bag contains more bags, add the number of it we need plus
            // the number of it we need multiplied by it's contents
            count += quantity + quantity * count_bags(contents, colour);
        } else {
            // Bag doesn't contain anything else, just add the number of it we need
            count += quantity;
        }
    }
    count
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let mut contents: HashMap<&str, HashSet<(&str, usize)>> = HashMap::new();
    let regex = Regex::new(r"(\d+)\s(\S+\s\S+)").unwrap();
    // Each rule looks something like:
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // <colour> bags contains (<quantity> <colour> bags)+
    for rule in input.lines() {
        let parts: Vec<&str> = rule.split(" bags contain ").collect();
        let outer_bag = parts[0];
        for item in parts[1].split(", ").collect::<Vec<&str>>() {
            if item == "no other bags." {
                break;
            }
            let capture = regex.captures(item).unwrap();
            let quantity = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let colour = capture.get(2).unwrap().as_str();
            if contents.contains_key(outer_bag) {
                contents
                    .get_mut(outer_bag)
                    .unwrap()
                    .insert((colour, quantity));
            } else {
                let mut set = HashSet::new();
                set.insert((colour, quantity));
                contents.insert(outer_bag, set);
            }
        }
    }
    let bag = "shiny gold";
    count_bags(&contents, bag)
}
