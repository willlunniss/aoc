#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut valid = 0;
    // Check each entry for validity and count the number that are valid
    /* 
    Each line gives the password policy and then the password. 
    The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
    For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
    */ 
    for s in input.lines() {
        let parts : Vec<&str> = s.split(": ").collect();
        let password = parts[1]; // Get the password
        let policy = parts[0]; // Extrat the policy fields (can't assume single digits)
        let policy_parts : Vec<&str> = policy.split(&['-', ' '][..]).collect();
        let min = policy_parts[0].parse::<i64>().unwrap();
        let max = policy_parts[1].parse::<i64>().unwrap();
        let char = policy_parts[2];
        //println!("Checking {} against {}:{}/{}", password, char, min, max);
        let occurrences = password.matches(char).count() as i64;
        // Check we have the right number of occurrences of the policy char
        if occurrences >= min && occurrences <= max {
           valid += 1;
        }
    }
    return valid;
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let mut valid = 0;
    // Check each entry for validity and count the number that are valid
    /* 
    Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on.
    (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter.
    Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
    */ 
    for s in input.lines() {
        let parts : Vec<&str> = s.split(": ").collect();
        let password = parts[1]; // Get the password
        let policy = parts[0]; // Extrat the policy fields (can't assume single digits)
        let policy_parts : Vec<&str> = policy.split(&['-', ' '][..]).collect();
        // Get positions (and convert from 1 (puzzle input) to 0 (rust) indexing)
        let pos1 = policy_parts[0].parse::<usize>().unwrap() - 1;
        let pos2 = policy_parts[1].parse::<usize>().unwrap() - 1;
        let char = policy_parts[2].chars().next();
        // Check we have exactly 1 instance of char at the possitions pos1/pos2
        let mut matches = 0;
        if password.chars().nth(pos1) == char {
            matches += 1;
        }
        if password.chars().nth(pos2) == char {
           matches += 1;
       }
       if matches == 1 {
           // Found just 1 match, password is valid
           valid += 1;
       }
    }
    return valid;
}
