use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use serde_json::Value;

#[aoc(day12, part1)]
fn part1(input: &str) -> i64 {
    // Split into values, keep only numbers and sum
    input
        .split(&[':', ',', '[', ']', '{', '}'][..])
        .flat_map(str::parse::<i64>)
        .sum()
}

// Recursively sums all numbers that are not within objects that contain a 'red' property
fn sum_valid(data: &Value) -> i64 {
    match data {
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_valid).sum(),
        Value::Object(obj) => obj
            .values()
            .fold_while(0, |sum, value| {
                if value == &Value::String("red".to_owned()) {
                    Done(0) // Discard all numbers in this object
                } else {
                    Continue(sum + sum_valid(value))
                }
            })
            .into_inner(),
        _ => 0,
    }
}

#[aoc(day12, part2)]
fn part2(input: &str) -> i64 {
    sum_valid(&serde_json::from_str(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
    }
}
