use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Default)]
struct Step {
    unlocks: Vec<char>,
    requires: Vec<char>,
}

/// Checks if a step is available i.e. all it's requirements have completed
fn is_available(steps: &HashMap<char, Step>, complete: &HashSet<char>, step: char) -> bool {
    if steps
        .get(&step)
        .unwrap()
        .requires
        .iter()
        .any(|r| !complete.contains(r))
    {
        // One or more of the requirements aren't complete yet
        return false;
    }
    true
}

/// Returns a list of steps that have no requirements
fn no_requirements(steps: &HashMap<char, Step>) -> Vec<char> {
    steps
        .iter()
        .filter(|(_, step)| step.requires.is_empty())
        .map(|(c, _)| *c)
        .collect()
}

#[aoc_generator(day7)]
fn gen(input: &str) -> HashMap<char, Step> {
    let mut steps: HashMap<char, Step> = HashMap::new();
    for (step, unlocks) in input
        .lines()
        .map(|line| (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap()))
    {
        // Record that this step unlocks another
        steps.entry(step).or_default().unlocks.push(unlocks);
        // Record that the other step is unlocked by this one
        steps.entry(unlocks).or_default().requires.push(step)
    }
    steps
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<char, Step>) -> String {
    let mut instructions = Vec::new();
    // Store available steps sorted by char from A->Z
    let mut available: BTreeSet<char> = BTreeSet::new();
    let mut complete = HashSet::new();
    // Start with steps that have no requirements
    available.extend(no_requirements(input));
    while !available.is_empty() {
        // Fetch the next available step (in alphabetical order) until done
        let step = &(*available.iter().next().unwrap()).clone();
        // Add to the ordered list of instructions
        instructions.push(*step);
        // And mark as completed
        complete.insert(*step);
        // Now make the steps that require this one available if all other requirements are complete
        let details = input.get(step).unwrap();
        for potentially_available in &details.unlocks {
            if is_available(input, &complete, *potentially_available) {
                // All requirements are complete, mark as available
                available.insert(*potentially_available);
            }
        }
        // Remove from queue
        available.remove(step);
    }
    // Return instructions in the order we handled them
    instructions.iter().collect()
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<char, Step>) -> usize {
    // Store available steps sorted by char from A->Z
    let mut available: BTreeSet<char> = BTreeSet::new();
    // Store active steps sorted by soonest to latest finish time
    let mut active: BTreeMap<usize, char> = BTreeMap::new();
    let mut complete = HashSet::new();
    // Start with steps that have no requirements
    available.extend(no_requirements(input));
    let workers = 5;
    let mut current_time = 0;
    while !available.is_empty() || !active.is_empty() {
        // Work out which step completes next (assume only 1 step can finish at the same time)
        if let Some((finish_time, step)) = active.iter().next() {
            // Advance time
            current_time = *finish_time;
            // And mark as completed
            complete.insert(*step);
            // Now make the steps that require this one available if all other requirements are complete
            let details = input.get(step).unwrap();
            for potentially_available in &details.unlocks {
                if is_available(input, &complete, *potentially_available) {
                    // All requirements are complete, mark as available
                    available.insert(*potentially_available);
                }
            }
        }
        // Remove from active the one that just completed
        active.remove(&current_time);

        // Now that allocate available steps to idle workers
        while active.len() < workers {
            // Fetch the next available step (in alphabetical order)
            if let Some(step) = available.iter().next() {
                let step = &step.clone();
                // Mark as active with a finish time of current time + 60 + char weight (A=1, B=2 etc...)
                active.insert(current_time + 60 + (*step as u8 - 64) as usize, *step);
                // Remove from queue
                available.remove(step);
            } else {
                // No more available steps right now
                break;
            }
        }
    }
    current_time
}
