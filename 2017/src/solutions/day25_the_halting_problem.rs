use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Action {
    write: usize,
    shift: isize,
    next: char,
}

struct Blueprint {
    initial_state: char,
    steps: usize,
    states: HashMap<char, [Action; 2]>,
}

#[aoc_generator(day25)]
fn gen(input: &str) -> Blueprint {
    // Split input into sections
    let mut sections = input.split("\n\n");
    // First section is the header
    let header = sections.next().unwrap().split('\n').collect::<Vec<_>>();
    let initial_state = header[0].split(' ').last().unwrap().chars().next().unwrap();
    let steps = header[1].split(' ').nth(5).unwrap().parse().unwrap();

    // Remaining sections are the states
    let re = Regex::new(r"In state (?P<state>\w):\n.*0:\n.*(?P<write0>\d).\n.*(?P<move0>right|left).\n.*(?P<next0>\w).\n.*1:\n.*(?P<write1>\d).\n.*(?P<move1>right|left).\n.*(?P<next1>\w).").unwrap();
    let mut states: HashMap<char, [Action; 2]> = HashMap::new();
    for section in sections {
        let caps = re.captures(section).unwrap();
        let mut actions = Vec::new();
        // Each state contains two potential actions based on the current value
        for current in [0, 1] {
            let write = caps.name(&format!("write{}", current)).unwrap().as_str();
            let shift = match caps.name(&format!("move{}", current)).unwrap().as_str() {
                "right" => 1,
                "left" => -1,
                _ => unreachable!(),
            };
            let next = caps.name(&format!("next{}", current)).unwrap().as_str();
            actions.push(Action {
                write: write.parse().unwrap(),
                shift,
                next: next.chars().next().unwrap(),
            });
        }
        states.insert(
            caps.name("state").unwrap().as_str().chars().next().unwrap(),
            actions.try_into().unwrap(),
        );
    }
    Blueprint {
        initial_state,
        steps,
        states,
    }
}

#[aoc(day25, part1)]
fn part1(blueprint: &Blueprint) -> usize {
    let mut tape: HashMap<isize, usize> = HashMap::new();
    let mut cursor = 0;
    let mut state = blueprint.initial_state;
    // For the stated number of steps
    for _ in 0..blueprint.steps {
        // Determine what action to perform based on the current state and value
        let value = tape.entry(cursor).or_insert(0);
        let action = &blueprint.states.get(&state).unwrap()[*value];
        // Update the value on the tape, shift the cursor and set the next state
        *value = action.write;
        cursor += action.shift;
        state = action.next;
    }
    // Result is the number of 1s on the tape
    tape.values().filter(|value| **value == 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc!(
        "
    Begin in state A.
    Perform a diagnostic checksum after 6 steps.
    
    In state A:
      If the current value is 0:
        - Write the value 1.
        - Move one slot to the right.
        - Continue with state B.
      If the current value is 1:
        - Write the value 0.
        - Move one slot to the left.
        - Continue with state B.
    
    In state B:
      If the current value is 0:
        - Write the value 1.
        - Move one slot to the left.
        - Continue with state A.
      If the current value is 1:
        - Write the value 1.
        - Move one slot to the right.
        - Continue with state A.
    "
    );

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 3);
    }
}
