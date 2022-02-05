type CharIter<'a> = Box<&'a mut dyn Iterator<Item = char>>;

/// Consumes the iterator until (and including) the first non-numeric char is encountered
/// Returns the value of the numeric chars.
fn take_number(iter: &mut CharIter) -> usize {
    iter.take_while(|x| x.is_numeric())
        .map(|x| x.to_digit(10).unwrap() as usize)
        .fold(0, |result, digit| (result * 10) + digit)
}

/// Computes the decompressed length
fn decompressed_length(iter: &mut CharIter, fully_expand: bool) -> usize {
    let mut length = 0;
    while let Some(c) = iter.next() {
        match c {
            '(' => {
                // Start of repetition marker
                let mut section_length = take_number(iter);
                let repeat_count = take_number(iter);
                if fully_expand {
                    // Recursively evaluate the section's decompressed length
                    section_length = decompressed_length(
                        &mut CharIter::new(&mut iter.take(section_length)),
                        true,
                    );
                } else {
                    // Skip over the section ignoring any repetition markers in it
                    iter.nth(section_length - 1);
                };
                length += section_length * repeat_count;
            }
            _ => length += 1, // Count all other chars
        }
    }
    length
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    decompressed_length(&mut CharIter::new(&mut input.chars()), false)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    decompressed_length(&mut CharIter::new(&mut input.chars()), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
