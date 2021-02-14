use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    // Compile regexes used to validate fields
    static ref RE_HGT: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn has_required(data: &HashMap<&str, &str>) -> bool {
    // Check all required fields (don't requir cid)
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for required in required_fields.iter() {
        if !data.contains_key(required) {
            return false;
        }
    }
    return true;
}

fn is_valid(key: &str, value: &str) -> bool {
    return match key {
        "byr" => {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            (1920..=2002).contains(&value.parse::<usize>().unwrap())
        },
        "iyr" => {
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            (2010..=2020).contains(&value.parse::<usize>().unwrap())
        },
        "eyr" => {
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            (2020..=2030).contains(&value.parse::<usize>().unwrap())
        },
        "hgt" => {
            // hgt (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76.
            let capture = RE_HGT.captures(value);
            if capture.is_none() { 
                false
            } else {
                let hgt = capture.unwrap();
                if hgt.get(2).unwrap().as_str() == "cm" {
                    (150..=193).contains(&hgt.get(1).unwrap().as_str().parse::<usize>().unwrap())
                } else if hgt.get(2).unwrap().as_str() == "in" {
                    (59..=76).contains(&hgt.get(1).unwrap().as_str().parse::<usize>().unwrap())
                } else {
                    false
                }
            }
        },
        "hcl" => {
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            RE_HCL.is_match(value)
        },
        "ecl" => {
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value)
        },
        "pid" => {
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            RE_PID.is_match(value)
        },
        _ => { true } // Don't care about other fields
    }
}

fn all_valid(data: &HashMap<&str, &str>) -> bool {
    // Check all required fields (don't requir cid)
    if !has_required(data) {
        return false;
    }
    // Now that we know they are all there for the second part check they are in range
    for (key, value) in data.iter() {
        if !is_valid(key, value) { return false; }
    }
    return true;
}

pub fn gen(input: &str) -> Vec<HashMap<&str, &str>> {
    return input.split("\r\n\r\n") // Split into sections based on empty lines
        .map(|section| section.split_whitespace().map(|field| field.splitn(2, ':').collect_tuple().unwrap()).collect()) // Build HashMap of key/value
        .collect(); // Build a Vector of the HashMaps
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    // Count number of passports that have all the required fields
    return gen(input).iter().filter(|&passport| has_required(passport)).count();
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    // Count number of passports where all fields are valid
    return gen(input).iter().filter(|&passport| all_valid(passport)).count();
}
