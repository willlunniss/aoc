#[aoc_generator(day17)]
fn gen(input: &str) -> usize {
    input.parse().unwrap()
}

fn spin(steps: usize, cycles: usize) -> Vec<usize> {
    // Values holds at each index, the next value in the buffer
    // e.g. [0,3,4,2,1] is represented as [3,0,1,4,2]
    let mut values = vec![0];
    let mut current = 0;
    for new in 1..cycles {
        // Advance forward the set number of steps to get the next current position
        // FIXME: This is really slow for high number of steps
        for _ in 0..steps {
            current = values[current];
        }
        // Insert the new value in between the current and whatever comes next
        let next = values[current];
        values[current] = new;
        current = new;
        values.push(next);
    }
    values
}

#[aoc(day17, part1)]
fn part1(input: &usize) -> usize {
    *spin(*input, 2018).last().unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &usize) -> usize {
    *spin(*input, 50_000_000).first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&3), 638);
    }
}
