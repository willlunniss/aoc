use itertools::Itertools;

struct PatternGenerator {
    base_pattern: Vec<i16>,
    element: usize,
}

impl PatternGenerator {
    fn new(base_pattern: &Vec<i16>, element: usize) -> PatternGenerator {
        return PatternGenerator {
            base_pattern: base_pattern.clone(),
            element: element,
        };
    }
}

impl<'a> IntoIterator for &'a PatternGenerator {
    type Item = i16;
    type IntoIter = PatternGeneratorIterator<'a>;

    fn into_iter(self)  -> Self::IntoIter {
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

impl <'a> Iterator for PatternGeneratorIterator<'a> {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
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
    base_pattern: Vec<i16>,
    signal: Vec<i16>,
}

impl FFT {
    fn new(base_pattern: &Vec<i16>, signal: &Vec<i16>) -> FFT {
        FFT {
            base_pattern: base_pattern.clone(),
            signal: signal.clone(),
        }
    }

    fn calculate(&mut self, phases: usize) {
        let mut output = vec![0; self.signal.len()];
        for phase in 1..=phases {
            println!("Phase {}", phase);
            // Run for the requested number of phases
            for digit in 0..self.signal.len() {
                // Calculate the result for this digit
                // Multiply each digit by the corresponding pattern value and then add up all the results
                let result : i16 = self.signal.iter().zip(PatternGenerator::new(&self.base_pattern, digit).into_iter().take(self.signal.len()))
                    .map(|(sig_digit, p_value)| sig_digit * p_value).sum();
                // Then convert to absolute and take the last digit (library doesn't support negative numbers)
                output[digit] = i16::abs(result) % 10;
            }
            std::mem::swap(&mut self.signal, &mut output);
        }
    }
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<i16> {
    return input.chars().map(|c| c.to_digit(10).unwrap() as i16).collect();
}

#[aoc(day16, part1)]
fn part1(input: &Vec<i16>) -> String {
    // Run the input directly through 100 phases and return the first 8 digits
    let mut fft = FFT::new(&[0, 1, 0, -1].to_vec(), input);
    fft.calculate(100);
    return fft.signal.iter().take(8).join("");
}

#[aoc(day16, part2)]
fn part2(input: &Vec<i16>) -> String {
    // FIXME: Need to find a different way to do this as can't scale up to a signal of this size 
    // Repeat the signal 10000 times
    let mut signal = Vec::new();
    for copy in 0..10_000 {
        signal.extend(input);
    }
    let mut fft = FFT::new(&[0, 1, 0, -1].to_vec(), &signal);
    fft.calculate(100);
    // Get the message offset as the first 7 digits of the input
    let message_offset = input.iter().take(7).join("").parse::<usize>().unwrap();
    // Return the 8 digits at the message offset
    return fft.signal.iter().skip(message_offset).take(8).join("");
}