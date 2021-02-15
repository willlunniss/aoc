
#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<usize>) -> usize {
    let mut sorted = input.clone();
    sorted.push(0); // Add outlet which is 0
    sorted.sort(); // Sort smallest to largest
    sorted.push(sorted.last().unwrap() + 3); // Add built in adapter is always +3 more than last
    let mut differences = vec![0; 4]; // Expect no more than +3 difference
    for i in 1 .. sorted.len() {
        // For each adapter, record the difference between it and the previous
        differences[sorted[i] - sorted[i - 1]] += 1;
    }
    return differences[1] * differences[3];
}

#[aoc(day10, part2)]
fn part2(input: &Vec<usize>) -> usize {
    
    return 0;
}
