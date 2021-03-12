use itertools::Itertools;
use std::collections::HashMap;

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
fn produce<'a>(
    reactions: &'a HashMap<&'a str, (usize, Vec<(usize, &'a str)>)>,
    available: &mut HashMap<&'a str, usize>,
    produced: &mut HashMap<&'a str, usize>,
    units: usize,
    chemical: &'a str,
) {
    // We need units of chemical
    if chemical == "ORE" {
        // ORE is special as it's the base chemical that is readily available - no reaction needed
        produced
            .entry(chemical)
            .and_modify(|x| *x += units)
            .or_insert(units);
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
    let producing = required_runs * produces;
    // Update what will be spare afterwards
    *spare += producing - units;
    // Update what we are producing
    produced
        .entry(chemical)
        .and_modify(|x| *x += producing)
        .or_insert(producing);
    // Now make it by producing all of the chemicals needed to do a run
    for input in inputs {
        produce(
            reactions,
            available,
            produced,
            input.0 * required_runs,
            input.1,
        );
    }
}

#[aoc(day14, part1)]
fn part1(input: &str) -> usize {
    // Get the available reactions
    let reactions = gen(input);
    // Work out how much ORE we need to make 1 FUEL
    let mut available = HashMap::new();
    let mut produced = HashMap::new();
    let fuel = "FUEL".to_owned();
    produce(&reactions, &mut available, &mut produced, 1, &fuel);
    return *produced.get("ORE").unwrap();
}

#[aoc(day14, part2)]
fn part2(input: &str) -> usize {
    // Work out how much FUEL we can make with 1 trillion ORE
    let ore_limit: usize = 1_000_000_000_000;
    // Get the available reactions
    let reactions = gen(input);
    // First make 1 FUEL
    let mut available = HashMap::new();
    let mut produced = HashMap::new();
    let fuel = "FUEL".to_owned();
    produce(&reactions, &mut available, &mut produced, 1, &fuel);
    // See how much ORE we used out of our supply
    let fuel_requires = *produced.get("ORE").unwrap();

    // Now reset things and make the minimum we think we can
    // It won't use up all the ORE though due to intermediates having left overs that can be used
    available.clear();
    produced.clear();
    let mut target = ore_limit / fuel_requires;
    loop {
        // Make the target amount of FUEL
        produce(&reactions, &mut available, &mut produced, target, &fuel);
        // Find out how much ORE we have left (as we won't have actually used it all)
        let remaining = ore_limit - *produced.get("ORE").unwrap();
        if remaining < fuel_requires {
            // Won't be able to make a whole additional FUEL from scratch but we might still be able to
            // make one with the bits we have left over, so try to make just one more
            produce(&reactions, &mut available, &mut produced, 1, &fuel);
            // See if we stayed under the limit making that last FUEL
            if *produced.get("ORE").unwrap() > ore_limit {
                // Making that final FUEL put us over the limit, so can't count it
                return *produced.get("FUEL").unwrap() - 1;
            } else {
                // Managed to make that final FUEL with out going over the limit
                return *produced.get("FUEL").unwrap();
            };
        }
        // We have enough ORE left to definitely make more FUEL, set the target to make the minimum we known we can
        target = remaining / fuel_requires;
    }
}
