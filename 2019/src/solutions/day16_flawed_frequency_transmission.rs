use itertools::Itertools;

struct PatternGenerator {
    base_pattern: Vec<isize>,
    element: usize,
}

impl PatternGenerator {
    fn new(base_pattern: &Vec<isize>, element: usize) -> PatternGenerator {
        return PatternGenerator {
            base_pattern: base_pattern.clone(),
            element: element,
        };
    }
}

impl<'a> IntoIterator for &'a PatternGenerator {
    type Item = isize;
    type IntoIter = PatternGeneratorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PatternGeneratorIterator {
            generator: self,
            index: 0,
            repeat: 0,
        }
    }
}

/// Infinite pattern generator iterator, use with take(n) to limit the size
pub struct PatternGeneratorIterator<'a> {
    generator: &'a PatternGenerator,
    index: usize,
    repeat: usize,
}

impl<'a> Iterator for PatternGeneratorIterator<'a> {
    type Item = isize;
    fn next(&mut self) -> Option<isize> {
        // Create a pattern from the base pattern by repeating each value element number of times
        // Keep emitting the next entry in the pattern indefinitely (on the first pass we skip the first element)
        self.repeat += 1;
        if self.repeat > self.generator.element {
            // Have finished repeating this entry in the base pattern, move to next
            self.repeat = 0;
            self.index += 1;
        }
        if self.index >= self.generator.base_pattern.len() {
            // Reached the end, loop back
            self.index = 0;
            self.repeat = 0;
        }
        return Some(self.generator.base_pattern[self.index]);
    }
}

struct FFT {
    base_pattern: Vec<isize>,
    signal: Vec<isize>,
}

impl FFT {
    fn new(base_pattern: &Vec<isize>, signal: &Vec<isize>) -> FFT {
        FFT {
            base_pattern: base_pattern.clone(),
            signal: signal.clone(),
        }
    }

    fn calculate(&mut self, phases: usize) {
        let mut output = vec![0; self.signal.len()];
        for _phase in 1..=phases {
            // Run for the requested number of phases
            for digit in 0..self.signal.len() {
                // Calculate the result for this digit
                // Multiply each digit by the corresponding pattern value and then add up all the results
                let result: isize = self
                    .signal
                    .iter()
                    .zip(
                        PatternGenerator::new(&self.base_pattern, digit)
                            .into_iter()
                            .take(self.signal.len()),
                    )
                    .map(|(sig_digit, p_value)| sig_digit * p_value)
                    .sum();
                // Then convert to absolute and take the last digit
                output[digit] = isize::abs(result) % 10;
            }
            std::mem::swap(&mut self.signal, &mut output);
        }
    }
}

/// Uses a trick that the value calculated by the FFT (for the second half only) is just the
/// sum of it and previous (backwards) numbers mod 10
///
/// e.g. 12345678 -> ????6158
/// 8 + 7 + 6 + 5 % 10 = 6
/// 8 + 7 + 6 % 10 = 1
/// 8 + 7 % 10 = 5
/// 8 % 10 = 8
fn backwards_sum_2nd_half(input: &Vec<isize>, phases: usize) -> Vec<isize> {
    let mut input = input.clone();
    let mut output = vec![0; input.len()];
    for _phase in 0..phases {
        let mut sum = 0;
        // Start from the end of the input and work backwards (stopping at the halfway point)
        for index in (input.len() / 2..input.len()).rev() {
            // Add value for this index to the sum
            sum += input[index];
            // Output is sum mod 10
            output[index] = sum % 10;
        }
        std::mem::swap(&mut input, &mut output);
    }
    return input;
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<isize> {
    return input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();
}

#[aoc(day16, part1)]
fn part1_fft(input: &Vec<isize>) -> String {
    // Run the input directly through 100 phases and return the first 8 digits
    let mut fft = FFT::new(&[0, 1, 0, -1].to_vec(), input);
    fft.calculate(100);
    return fft.signal.iter().take(8).join("");
}

#[aoc(day16, part2)]
fn part2(input: &Vec<isize>) -> String {
    // Get the message offset as the first 7 digits of the input
    let message_offset = input.iter().take(7).join("").parse::<usize>().unwrap();
    // Append 10 thousand copies of the input
    let mut signal = Vec::new();
    for _copy in 0..10_000 {
        signal.extend(input);
    }
    // Take a shortcut to calculate the output (only creates valid values for the 2nd half of the output)
    // Can only make use of this because the data we want at the message offset is in the 2nd half
    let output = backwards_sum_2nd_half(&signal, 100);
    // Return the 8 digits at the message offset
    return output.iter().skip(message_offset).take(8).join("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backwards_sum() {
        let signal = [1, 2, 3, 4, 5, 6, 7, 8].to_vec();
        let output = backwards_sum_2nd_half(&signal, 4);
        assert_eq!(output[4..8], [9, 4, 9, 8])
    }
}
