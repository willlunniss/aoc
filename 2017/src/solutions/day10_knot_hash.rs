use itertools::Itertools;

#[derive(Debug)]
struct KnotHasher<const N: usize> {
    buffer: [usize; N],
    position: usize,
    skip_size: usize,
}

impl<const N: usize> KnotHasher<N> {
    /// Creates a new `KnotHasher`
    fn new() -> Self {
        Self {
            buffer: (0..N).collect::<Vec<_>>().try_into().unwrap(),
            position: 0,
            skip_size: 0,
        }
    }

    /// Applies a single length operation
    fn apply_length(&mut self, length: usize) {
        // Reverse length values in the circular buffer starting at the current position
        let values = (self.position..self.position + length)
            .map(|idx| self.buffer[idx % N])
            .collect::<Vec<_>>();
        for (offset, value) in values.iter().rev().enumerate() {
            self.buffer[(self.position + offset) % N] = *value;
        }
        // Advance position and increment skip size
        self.position = (self.position + length + self.skip_size) % N;
        self.skip_size += 1;
    }

    /// Applies a multiple lengths
    fn apply<'a>(&mut self, lengths: impl IntoIterator<Item = &'a usize>) {
        lengths
            .into_iter()
            .for_each(|length| self.apply_length(*length));
    }

    /// Returns the basic check value of the product of the first two values
    const fn check(&self) -> usize {
        self.buffer[0] * self.buffer[1]
    }

    /// Calculates the dense hash
    fn dense_hash(&mut self) -> String {
        // For each block of 16 values
        // xor all values in the block together
        // format each as a hex value with leading zero if needed
        self.buffer
            .into_iter()
            .chunks(16)
            .into_iter()
            .map(|block| block.fold(0, |acc, x| acc ^ x))
            .map(|xord| format!("{:0x}", xord))
            .join("")
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    // Treat input as a list of lengths and use it to apply a single round
    let lengths = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut hasher = KnotHasher::<256>::new();
    hasher.apply(lengths.iter());
    hasher.check()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    // Treat into as a list of ASCII bytes and append extra lengths
    let lengths = input
        .chars()
        .map(|c| c as u8 as usize)
        .chain([17, 31, 73, 47, 23])
        .collect::<Vec<_>>();

    let mut hasher = KnotHasher::<256>::new();
    // Apply lengths 64 times
    hasher.apply(lengths.iter().cycle().take(lengths.len() * 64));
    hasher.dense_hash()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let mut hasher = KnotHasher::<5>::new();
        let lengths = [3, 4, 1, 5];
        for length in lengths {
            hasher.apply_length(length);
        }
        assert_eq!(hasher.check(), 12);
    }

    #[test]
    fn test_part2_example_dense() {
        let mut hasher = KnotHasher::<16>::new();
        hasher.buffer = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        assert_eq!(hasher.dense_hash(), "40");
    }
}
