use std::collections::{HashMap, HashSet, VecDeque};

fn gen(input: &'_ str) -> (HashMap<&'_ str, Vec<&'_ str>>, &'_ str) {
    let mut lines = input.lines();
    let mut replacements: HashMap<&'_ str, Vec<&'_ str>> = HashMap::new();
    for (from, to) in lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|x| x.split_once(" => ").unwrap())
    {
        // Each replacement may map to multiple options, store in a map of lists
        replacements.entry(from).or_default().push(to);
    }
    (replacements, lines.next().unwrap())
}

#[aoc(day19, part1)]
fn part1(input: &str) -> usize {
    let (replacements, molecule) = gen(input);
    let mut molecules = HashSet::new();
    // For all possible replacements
    for (from, tos) in &replacements {
        // Find all ways it could be applied
        for (pos, _) in molecule.match_indices(from) {
            // And perform all possible replacements
            for to in tos {
                let mut new = molecule.to_owned();
                new.replace_range(pos..pos + from.len(), to);
                // Store in a set, so we keep only unique versions
                molecules.insert(new);
            }
        }
    }
    molecules.len()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> Option<usize> {
    // Calculates the number of steps to go from 'e' to the target molecule
    // This is done by starting at the target, and then reducing until we get to 'e'
    let (replacements, molecule) = gen(input);
    // Create a list of ways to reduce a molecule
    let mut reductions: Vec<(&str, &str)> = replacements
        .iter()
        .flat_map(move |(from, to)| to.iter().map(|x| (*x, *from)))
        .collect();
    reductions.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    // Track what to try next
    let mut queue = VecDeque::new();
    // Track what has been tried already
    let mut visited = HashSet::new();
    // Start with the target molecule
    queue.push_back((molecule.to_owned(), 0));
    while let Some((candidate, steps)) = queue.pop_front() {
        // See what can be done to reduce the candidate
        for (from, to) in &reductions {
            for (pos, _) in candidate.match_indices(from) {
                // Create all possible new versions
                let mut new = candidate.clone();
                new.replace_range(pos..pos + from.len(), to);

                if new == "e" {
                    // Have managed to reduce molecule down to the base e
                    // (with what would be the same number of steps to go from 'e' to the target molecule)
                    return Some(steps + 1);
                } else if !visited.contains(&new) {
                    // Something new, add to queue to try next
                    visited.insert(new.clone());
                    queue.push_front((new, steps + 1));
                }
            }
        }
    }
    None // Failed to reduce
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    e => H
    e => O
    H => HO
    H => OH
    O => HH

    HOHOHO
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), Some(6));
    }
}
