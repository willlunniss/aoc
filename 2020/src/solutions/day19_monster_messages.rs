use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn resolve(rules: &HashMap<&str, &str>, key: &str) -> String {
    // TODO: This is very inefficient. Cache resolved rules and/or don't do all the final String allocation until all rules are resolved
    let rule = rules.get(key).unwrap();
    if rule.chars().nth(0).unwrap() == '\"' {
        // Terminating rule '"<char>"'
        return rule.chars().nth(1).unwrap().to_string();
    } else if !rule.chars().nth(0).unwrap().is_digit(10) {
        // Fully resolved rule - just return it directly
        return rule.to_string();
    } else {
        // Referencing rule, resolve it
        if rule.contains(" | ") {
            // Contains two alternatives parts
            let (part1, part2) = rule.splitn(2, " | ").collect_tuple().unwrap();
            let resolved1: String = part1
                .split(" ")
                .map(|sub_rule| resolve(&rules, sub_rule))
                .collect();
            let resolved2: String = part2
                .split(" ")
                .map(|sub_rule| resolve(&rules, sub_rule))
                .collect();
            return ["(", &resolved1, "|", &resolved2, ")"].concat();
        } else {
            return rule
                .split(" ")
                .map(|sub_rule| resolve(&rules, sub_rule))
                .collect();
        }
    }
}

/// Generates rule 11 for part 2 by manually expanding the recursive rule within reason
fn part2_gen_11(rule48: String, rule31: String) -> String {
    // 11: 42 31 | 42 11 31 can be handled as require 1 or more of an equal number of 42's followed by an equal number of 31's
    // Couldn't get grouping to work properly with recursive Regex so just manually expand it up to 'enough' iterations
    // By writing as 11: 42 (?:42 (?:42 (?:42 ... 31)? 31)? 31)? 31
    let mut builder = Vec::with_capacity(44);
    builder.push("(");
    builder.push(&rule48);
    for _ in 0..10 {
        builder.push("(?:");
        builder.push(&rule48);
    }
    for _ in 0..10 {
        builder.push(&rule31);
        builder.push(")?");
    }
    builder.push(&rule31);
    builder.push(")");
    builder.concat()
}

fn gen(input: &str) -> (HashMap<&str, &str>, Vec<&str>) {
    // Split into rules and messages
    let (rules_str, messages_str) = input.splitn(2, "\r\n\r\n").collect_tuple().unwrap();
    // Load rules into a HashMap
    let rules = rules_str
        .lines()
        .map(|rule| rule.splitn(2, ": ").collect_tuple().unwrap())
        .collect::<HashMap<&str, &str>>();
    return (rules, messages_str.lines().collect());
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (rules, messages) = gen(input);
    // Use Regex to count how many messages match rule 0
    let re = Regex::new(&["^", &resolve(&rules, "0"), "$"].concat()).unwrap();
    messages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> usize {
    let (mut rules, messages) = gen(input);
    // Patch rules (the introduced loops can be handled for these cases as meaning 1 or more instances of the references in the original rule)
    // 8: 42 | 42 8 can simply be represented as (42)+ for 1 or more
    let patched8 = &["(", &resolve(&rules, "42"), ")+"].concat();
    // 11: 42 31 | 42 11 31 is more complicated because require 1 or more of an equal number of 42's followed by an equal number of 31's
    let patched11 = &part2_gen_11(resolve(&rules, "42"), resolve(&rules, "31"));
    rules.insert("8", patched8);
    rules.insert("11", patched11);
    // Use Regex to count how many messages match the new rule 0
    let re = Regex::new(&["^", &resolve(&rules, "0"), "$"].concat()).unwrap();
    messages
        .iter()
        .filter(|message| re.is_match(message))
        .count()
}
