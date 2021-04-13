use itertools::Itertools;

/// Transforms a subject number loop size times to calculate a public key
fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 1..=loop_size {
        value *= subject_number;
        value %= 20_201_227;
    }
    value
}

/// Finds the loop size needed to calculate the supplied public key for the given
/// subject number
const fn find_loop_size(subject_number: usize, public_key: usize) -> usize {
    let mut value = 1;
    let mut iterations = 0;
    // Keep transforming the number until we get the target public key
    while value != public_key {
        iterations += 1;
        value *= subject_number;
        value = value % 20_201_227;
    }
    // The number of iterations we needed is the target loop size
    iterations
}

#[aoc_generator(day25)]
fn gen(input: &str) -> (usize, usize) {
    return input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
}

#[aoc(day25, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (card_public_key, door_public_key) = *input;
    let card_loop_size = find_loop_size(7, card_public_key);
    transform(door_public_key, card_loop_size)
}
