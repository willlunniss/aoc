use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

struct PasswordPolicy {
    password: String,
    policy_char: char,
    val1: usize,
    val2: usize,
}

impl FromStr for PasswordPolicy {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Decode the password and policy of form
        // 1-3 a: abcde
        let (policy, password) = s.splitn(2, ": ").collect_tuple().unwrap();
        let (val1, val2, policy_char) = policy.splitn(3, &['-', ' '][..]).collect_tuple().unwrap();
        return Ok(Self {
            password: password.to_string(),
            policy_char: policy_char.chars().next().unwrap(),
            val1: val1.parse().unwrap(),
            val2: val2.parse().unwrap(),
        });
    }
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<PasswordPolicy> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordPolicy]) -> usize {
    // Check each entry for validity and count the number that are valid
    /*
    The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid.
    For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
    */
    input
        .iter()
        .filter(|p| (p.val1..=p.val2).contains(&p.password.matches(p.policy_char).count()))
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[PasswordPolicy]) -> usize {
    // Check each entry for validity and count the number that are valid
    /*
    Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on.
    (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter.
    Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
    */
    input
        .iter()
        .filter(|p| {
            (p.password.chars().nth(p.val1 - 1).unwrap() == p.policy_char)
                ^ (p.password.chars().nth(p.val2 - 1).unwrap() == p.policy_char)
        })
        .count()
}
