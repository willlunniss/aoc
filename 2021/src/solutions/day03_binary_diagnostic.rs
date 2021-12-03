#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let mut counts = vec![0; 12];
    for number in input.lines() {
        for (index, c) in number.chars().enumerate() {
            if c == '1' {
                counts[index] += 1;
            } else {
                counts[index] -= 1;
            }
        }
    }
    let gamma_rate = counts.iter().rev().enumerate().map(|(i, x)| if *x > 0 { 1 << i } else { 0 } ).sum::<usize>();
    let epsilon_rate = gamma_rate ^ ((1 << 12) - 1);
    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let length = 12;
    let mut oxygen = input.lines().collect::<Vec<_>>();
    let mut o2 = oxygen.clone();
    for index in 0..length {
        if oxygen.len() > 1 {
            let mut keep = Vec::new();
            let mut count = 0;
            for number in &oxygen {
                if number.chars().nth(index).unwrap() == '1' {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            let test = if count >= 0 { '1' } else { '0' };
            for number in &oxygen {
                if number.chars().nth(index).unwrap() == test {
                    keep.push(*number);
                }
            }
            oxygen = keep;
        }
        
        if o2.len() > 1 {
            let mut keep = Vec::new();
            let mut count = 0;
            for number in &o2 {
                if number.chars().nth(index).unwrap() == '1' {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            let test = if count >= 0 { '0' } else { '1' };
            for number in &o2 {
                if number.chars().nth(index).unwrap() == test {
                    keep.push(*number);
                }
            }
            o2 = keep;
        }
    }
    usize::from_str_radix(oxygen.first().unwrap(), 2).unwrap() * usize::from_str_radix(o2.first().unwrap(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
"};

    #[test]
    fn test_part1_example() {
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 230);
    }
}