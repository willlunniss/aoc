use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Program<'a> {
    weight: usize,
    disc: Vec<&'a str>,
}

fn parse_line(s: &str) -> (&str, Program) {
    // Split input that looks like
    // ktlj (57)
    // fwft (72) -> ktlj, cntj, xhth
    let parts = s
        .split(&[' ', '(', ')', ','][..])
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();
    let name = parts[0];
    let weight = parts[1].parse().unwrap();
    let disc = parts.iter().skip(3).copied().collect();
    (name, Program { weight, disc })
}

fn gen(input: &str) -> HashMap<&str, Program> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn find_root<'a>(programs: &HashMap<&'a str, Program>) -> &'a str {
    // Get a list of all child programs
    let children = programs
        .values()
        .flat_map(|p| p.disc.iter().collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    // Find the one program that isn't a child of another
    *programs
        .keys()
        .find(|name| !children.contains(*name))
        .unwrap()
}

fn check_weight(
    programs: &HashMap<&str, Program>,
    program: &Program,
) -> Result<(usize, usize), usize> {
    // Recursively check the program's weight either returning:
    // * If properly balanced: Ok(program weight, weight of program and all children)
    // * If not balanced: Err(required weight to be balanced)

    // If the disc is empty then can't be unbalanced
    if program.disc.is_empty() {
        return Ok((program.weight, program.weight));
    }
    // Check each child on the disc
    // If the child is unbalanced then return the error up the call stack
    let mut child_weights = Vec::new();
    for child in &program.disc {
        let child_weight = check_weight(programs, programs.get(child).unwrap())?;
        child_weights.push(child_weight);
    }

    // Check that the weights of all children are equal
    let (_, first) = child_weights[0];
    if child_weights.iter().all(|(_, weight)| *weight == first) {
        // Weights are equal, this program is balanced
        return Ok((
            program.weight,
            program.weight + (first * child_weights.len()),
        ));
    }
    // Child weights aren't equal, this program isn't balanced, find the one that is wrong
    Err(child_weights
        .iter()
        .tuple_windows::<(_, _, _)>()
        .find_map(|(a, b, c)| {
            if a.1 != b.1 {
                // Find out which one has the error
                let error = if a.1 == c.1 { b } else { a };
                // Calculate the required weight for this child to equal the others and balance this program
                return Some(error.0 + c.1 - error.1);
            }
            None
        })
        .unwrap())
}

#[aoc(day7, part1)]
fn part1(input: &str) -> String {
    let programs = gen(input);
    // Find the one program that isn't a child of another
    find_root(&programs).to_string()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let programs = gen(input);
    // Find the one program that isn't a child of another
    let root = find_root(&programs);

    // Check weights, expect to find an error indicating what the incorrect weight of the single incorrect program should be
    check_weight(&programs, programs.get(root).unwrap()).unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
        pbga (66)
        xhth (57)
        ebii (61)
        havc (66)
        ktlj (57)
        fwft (72) -> ktlj, cntj, xhth
        qoyq (66)
        padx (45) -> pbga, havc, qoyq
        tknk (41) -> ugml, padx, fwft
        jptl (61)
        ugml (68) -> gyxo, ebii, jptl
        gyxo (61)
        cntj (57)
    "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), "tknk");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 60);
    }
}
