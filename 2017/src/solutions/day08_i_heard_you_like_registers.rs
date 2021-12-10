use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Op {
    Inc,
    Dec,
}

#[derive(Debug)]
struct Instruction<'a> {
    r: &'a str,
    op: Op,
    value: isize,
    condition: Condition<'a>,
}

#[derive(Debug)]
struct Condition<'a> {
    cr: &'a str,
    test: &'a str,
    value: isize,
}

fn parse_instruction(s: &str) -> Instruction {
    // Parse instructions in the form
    // c inc -20 if c == 10
    let parts = s.split(' ').collect::<Vec<_>>();
    Instruction {
        r: parts[0],
        op: parts[1].parse().unwrap(),
        value: parts[2].parse().unwrap(),
        condition: Condition {
            cr: parts[4],
            test: parts[5],
            value: parts[6].parse().unwrap(),
        },
    }
}

fn gen(input: &str) -> Vec<Instruction> {
    input.lines().map(|line| parse_instruction(line)).collect()
}

fn evaluate(instructions: &[Instruction]) -> (isize, isize) {
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut max = 0;
    for instruction in instructions {
        let cr = *registers.get(instruction.condition.cr).unwrap_or(&0);
        // For each instruction, if the condition evaluates to true
        // update the value in the specified resister
        if match instruction.condition.test {
            ">" => cr > instruction.condition.value,
            "<" => cr < instruction.condition.value,
            ">=" => cr >= instruction.condition.value,
            "<=" => cr <= instruction.condition.value,
            "==" => cr == instruction.condition.value,
            "!=" => cr != instruction.condition.value,
            _ => unreachable!(),
        } {
            let value = registers.entry(instruction.r).or_insert(0);
            *value += match instruction.op {
                Op::Inc => instruction.value,
                Op::Dec => -instruction.value,
            };
            if *value > max {
                max = *value;
            }
        }
    }
    (*registers.values().max().unwrap(), max)
}

#[aoc(day8, part1)]
fn part1(input: &str) -> isize {
    // Find the highest value once we have evaluated all instructions
    let (highest, _) = evaluate(&gen(input));
    highest
}

#[aoc(day8, part2)]
fn part2(input: &str) -> isize {
    // Find the maximum ever value from evaluating all instructions
    let (_, max_ever) = evaluate(&gen(input));
    max_ever
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
    b inc 5 if a > 1
    a inc 1 if b < 5
    c dec -10 if a >= 1
    c inc -20 if c == 10
    "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 10);
    }
}
