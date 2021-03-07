use std::collections::HashMap;
use itertools::Itertools;

/// Splits a reaction component into it's units and chemical
fn parse_component(input: &str) -> (usize, &str) {
    // 2 D => (2, D)
    let (units, chemical) = input.splitn(2, " ").collect_tuple().unwrap();
    return (units.parse().unwrap(), chemical);
}

/// Generates the list of reactions from the input
fn gen(input: &str) -> HashMap<&str, (usize, Vec<(usize, &str)>)> {
    let mut reactions = HashMap::new();
    for reaction in input.lines() {
        // Parse recipe e.g. 1 A, 2 B, 3 C => 2 D
        let (inputs, output) = reaction.splitn(2, " => ").collect_tuple().unwrap();
        let produces = parse_component(output);
        let requires = inputs.split(", ").map(|x| parse_component(x)).collect();
        // Store D = (2, [(1, A), (2, B), (3, C)])
        reactions.insert(produces.1, (produces.0, requires));
    }
    return reactions;
}

/// Produces units of chemical, updating available with any spare by products and ore with what was needed to make it all
fn produce<'a>(reactions: &'a HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>, available: &mut HashMap<&'a str, usize>, ore: &mut usize, units: usize, chemical: &'a str) {
    // We need units of chemical
    // If it's ORE, then we have unlimited but need to track it
    if chemical == "ORE" {
        *ore += units;
        return;
    }
    // Not ORE, need to produce it
    // See if we have any spare from previous reactions
    let spare = available.entry(chemical).or_default();
    if *spare >= units {
        // We have enough, decrement it and return
        *spare -= units;
        return;
    }
    // We don't have enough, look up the reaction to see how to make it
    let (produces, inputs) = reactions.get(chemical).unwrap();
    // Work out how many times we need to run the reaction
    // (What we need - what we have) / how much we produce in one go, rounded up to nearest whole number
    let required_runs = f64::ceil((units - *spare) as f64 / *produces as f64) as usize;
    // And what will be left over afterwards
    let spare_after_runs = *spare + (required_runs * produces) - units;
    // Now make it by producing all of the chemicals needed to do a run
    for input in inputs {
        produce(reactions, available, ore, input.0 * required_runs, input.1);
    }
    // Now we have made it, update the spare
    available.insert(chemical, spare_after_runs);
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    // Get the available reactions
    let reactions = gen(input);
    // Work out how much ORE we need to make 1 FUEL
    let mut ore = 0;
    let mut available = HashMap::new();
    produce(&reactions, &mut available, &mut ore, 1, &"FUEL".to_owned());
    return ore;
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    // Get the available reactions
    let reactions = gen(input);
    // Work out how much FUEL we can make with 1 trillion fuel (can keep using left over available chemicals)
    let mut available = HashMap::new();
    let mut ore = 0;
    let mut can_make = 0;
    let fuel = "FUEL".to_owned();
    // TODO: This is pretty slow, need to find a way to optimise
    loop {
        produce(&reactions, &mut available, &mut ore, 1, &fuel);
        if ore > 1_000_000_000_000 {
            // Used up our supply
            break;
        }
        // Made 1 FUEL without using up our supply, increment the counter
        can_make += 1;
    }
    return can_make;
}