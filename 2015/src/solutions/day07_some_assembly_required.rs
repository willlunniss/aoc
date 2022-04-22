use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Signal {
    Value(u32),
    Wire((char, char)),
}

impl FromStr for Signal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse() {
            Ok(Self::Value(value))
        } else {
            Ok(Self::new_wire(s))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Gate {
    Direct(Signal),
    Not(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, Signal),
    RShift(Signal, Signal),
}

/// Parses line into an output signal and gate tuple
fn read_line(s: &str) -> (Signal, Gate) {
    let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
    match parts[1] {
        "AND" => (
            parts[4].parse().unwrap(),
            Gate::And(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
        ),
        "OR" => (
            parts[4].parse().unwrap(),
            Gate::Or(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
        ),
        "LSHIFT" => (
            parts[4].parse().unwrap(),
            Gate::LShift(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
        ),
        "RSHIFT" => (
            parts[4].parse().unwrap(),
            Gate::RShift(parts[0].parse().unwrap(), parts[2].parse().unwrap()),
        ),
        "->" => (
            parts[2].parse().unwrap(),
            Gate::Direct(parts[0].parse().unwrap()),
        ),
        _ => {
            assert_eq!(parts[0], "NOT");
            (
                parts[3].parse().unwrap(),
                Gate::Not(parts[1].parse().unwrap()),
            )
        }
    }
}

impl Signal {
    /// Creates a new `Signal::Wire`
    fn new_wire(wire: &str) -> Self {
        // Wire names are 1/2 chars
        let mut chars = wire.chars();
        let name = (chars.next().unwrap(), chars.next().unwrap_or(' '));
        Self::Wire(name)
    }

    /// Recursively computes the value of the `Signal`
    /// Updates `circuit` with the value to avoid re-evaluating a `Signal` many times
    fn value(&self, circuit: &mut HashMap<Self, Gate>) -> u32 {
        match self {
            Self::Value(value) => *value,
            Self::Wire(_) => {
                // Compute the value of the wire by recursively evaluating all wires/gates connected to it
                let value = circuit.get(self).unwrap().clone().output(circuit);
                // Replace this circuit with a direct value to avoid later re-evaluation
                circuit.insert(*self, Gate::Direct(Self::Value(value)));
                value
            }
        }
    }
}

impl Gate {
    /// Computes the output of a Gate
    fn output(&self, circuit: &mut HashMap<Signal, Self>) -> u32 {
        match self {
            Self::Direct(a) => a.value(circuit),
            Self::Not(a) => !a.value(circuit),
            Self::And(a, b) => a.value(circuit) & b.value(circuit),
            Self::Or(a, b) => a.value(circuit) | b.value(circuit),
            Self::LShift(a, b) => a.value(circuit) << b.value(circuit),
            Self::RShift(a, b) => a.value(circuit) >> b.value(circuit),
        }
    }
}

#[aoc_generator(day7)]
fn gen(input: &str) -> HashMap<Signal, Gate> {
    input.lines().map(read_line).collect()
}

#[aoc(day7, part1)]
fn part1(input: &HashMap<Signal, Gate>) -> u32 {
    // Compute the value of a
    let a = input.get(&Signal::new_wire("a")).unwrap();
    a.output(&mut input.clone())
}

#[aoc(day7, part2)]
fn part2(input: &HashMap<Signal, Gate>) -> u32 {
    // Compute the value of a
    let a = input.get(&Signal::new_wire("a")).unwrap();
    let value = a.output(&mut input.clone());
    // Reset the circuit with b overridden
    let mut circuit = input.clone();
    circuit.insert(Signal::new_wire("b"), Gate::Direct(Signal::Value(value)));
    // Compute new value of a
    a.output(&mut circuit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i
    d AND e -> a
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 72);
    }
}
