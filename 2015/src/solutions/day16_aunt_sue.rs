use lazy_static::lazy_static;
use std::collections::HashMap;

type SueId = u32;

lazy_static! {
    static ref MFCSAM: HashMap<&'static str, u32> = {
        // Map of the detected values from the MFCSAM
        let mut m = HashMap::new();
        m.insert("children", 3);
        m.insert("cats", 7);
        m.insert("samoyeds", 2);
        m.insert("pomeranians", 3);
        m.insert("akitas", 0);
        m.insert("vizslas", 0);
        m.insert("goldfish", 5);
        m.insert("trees", 3);
        m.insert("cars", 2);
        m.insert("perfumes", 1);
        m
    };
}

// Generators an iterator where each item is the (sue id, properties)
fn gen(input: &str) -> impl Iterator<Item = (SueId, HashMap<&'_ str, u32>)> {
    input.lines().map(|line| {
        let (sue, properties) = line.split_once(": ").unwrap();
        (
            sue.chars().skip(4).collect::<String>().parse().unwrap(),
            properties
                .split(", ")
                .map(|x| {
                    let (name, value) = x.split_once(": ").unwrap();
                    (name, value.parse().unwrap())
                })
                .collect(),
        )
    })
}

// Finds the id that matches what the MFCSAM detected based on the validate closure
fn which_sue<'a, I>(mut sues: I, validate: fn(&str, u32, u32) -> bool) -> SueId
where
    I: Iterator<Item = (SueId, HashMap<&'a str, u32>)>,
{
    sues.find(|(_, properties)| {
        properties
            .iter() // All remembered properties must validate against MFCSAM results
            .all(|(k, v)| validate(k, *v, *MFCSAM.get(k).unwrap()))
    })
    .map(|(sue, _)| sue)
    .unwrap()
}

#[aoc(day16, part1)]
fn part1(input: &str) -> u32 {
    which_sue(gen(input), |_, remembered, detected| detected == remembered)
}

#[aoc(day16, part2)]
fn part2(input: &str) -> u32 {
    which_sue(gen(input), |name, remembered, detected| match name {
        "cats" | "trees" => detected < remembered,
        "pomeranians" | "goldfish" => detected > remembered,
        _ => detected == remembered,
    })
}
