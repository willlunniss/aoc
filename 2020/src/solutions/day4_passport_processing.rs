use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Compile regexes used to validate fields
    static ref RE_HGT: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9,a-f]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

fn validate_passport_data(data: &HashMap<&str, &str>) -> (bool, bool) {
    // Check all required fields (don't requir cid)
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for required in required_fields.iter() {
        if !data.contains_key(required) {
            return (false, false);
        }
    }
    // Now that we know they are all there for the second part check they are in range
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr = data.get("byr").unwrap().parse::<usize>().unwrap();
    if byr < 1920 || byr > 2002 { return (true, false); }
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr = data.get("iyr").unwrap().parse::<usize>().unwrap();
    if iyr < 2010 || iyr > 2020 { return (true, false); }
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr = data.get("eyr").unwrap().parse::<usize>().unwrap();
    if eyr < 2020 || eyr > 2030 { return (true, false); }
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    let capture = RE_HGT.captures(data.get("hgt").unwrap());
    if capture.is_none() { return (true, false); } // Doesn't match
    let hgt = capture.unwrap();
    let hgt_value = hgt.get(1).unwrap().as_str().parse::<usize>().unwrap();
    if hgt.get(2).unwrap().as_str() == "cm" {
        if hgt_value < 150 || hgt_value > 193 { return (true, false); }
    } else if hgt.get(2).unwrap().as_str() == "in" {
        if hgt_value < 59 || hgt_value > 76 { return (true, false); }
    }    
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let matches = RE_HCL.captures(data.get("hcl").unwrap());
    if matches.is_none() { return (true, false); } // Doesn't match

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let ecl = *data.get("ecl").unwrap();
    if ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" && ecl != "grn" && ecl != "hzl" && ecl != "oth" { return (true, false); } // Doesn't match

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let matches = RE_PID.captures(data.get("pid").unwrap());
    if matches.is_none() { return (true, false); } // Doesn't match

    // cid (Country ID) - ignored, missing or not.

    return (true, true);
}


#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    // Process the passport data in key:value pairs seperated by spaces and new lines
    // A blank line indicates start of a new passport
    let mut complete = 0;
    let mut data : HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            // End of current passport, validate
            let (fields_present, _) = validate_passport_data(&data);
            if fields_present {
                complete += 1;
            }
            data.clear();
            continue;
        }
        // Each row may have multiple fields
        let fields = line.split(' ');
        for field in fields {
            // Split out key/pair values and add to data hash
            let parts : Vec<&str> = field.split(':').collect();
            data.insert(parts[0], parts[1]);
        }
    }
    // Check any non-terminated data at the end
    let (fields_present, _) = validate_passport_data(&data);
    if fields_present {
        complete += 1;
    }
    return complete;
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {    
    // Process the passport data in key:value pairs seperated by spaces and new lines
    // A blank line indicates start of a new passport
    let mut valid = 0;
    let mut data : HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            // End of current passport, validate
            let (_, fields_valid) = validate_passport_data(&data);
            if fields_valid {
                valid += 1;
            }
            data.clear();
            continue;
        }
        // Each row may have multiple fields
        let fields = line.split(' ');
        for field in fields {
            // Split out key/pair values and add to data hash
            let parts : Vec<&str> = field.split(':').collect();
            data.insert(parts[0], parts[1]);
        }
    }
    // Check any non-terminated data at the end
    let (_, fields_valid) = validate_passport_data(&data);
    if fields_valid {
        valid += 1;
    }
    return valid;
}
