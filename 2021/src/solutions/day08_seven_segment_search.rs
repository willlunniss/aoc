use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

lazy_static! {
    // Build map of all valid post-mapped outputs to their 7 segment value
    static ref DISPLAYS: HashMap<u8, u32> = {
        let mut m = HashMap::new();
        m.insert("abcefg".chars().map(char_to_bit_flag).sum(), 0);
        m.insert("cf".chars().map(char_to_bit_flag).sum(), 1);
        m.insert("acdeg".chars().map(char_to_bit_flag).sum(), 2);
        m.insert("acdfg".chars().map(char_to_bit_flag).sum(), 3);
        m.insert("bcdf".chars().map(char_to_bit_flag).sum(), 4);
        m.insert("abdfg".chars().map(char_to_bit_flag).sum(), 5);
        m.insert("abdefg".chars().map(char_to_bit_flag).sum(), 6);
        m.insert("acf".chars().map(char_to_bit_flag).sum(), 7);
        m.insert("abcdefg".chars().map(char_to_bit_flag).sum(), 8);
        m.insert("abcdfg".chars().map(char_to_bit_flag).sum(), 9);
        m
    };
}

/// Represents the chars 'a' -> 'g' using 7 bit flags
const fn char_to_bit_flag(x: char) -> u8 {
    1 << ((x as u8) - 97)
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    // For each line, sum up the number of outputs that can be uniquely identified
    // (1, 4, 7 and 8 are uniq in that they are the only numbers with 2, 3, 4 and 7 segments active)
    input
        .lines()
        .map(|line| {
            let (_, outputs) = line.split_once("|").unwrap();
            outputs
                .split_ascii_whitespace()
                .filter(|segments| {
                    segments.len() == 2
                        || segments.len() == 3
                        || segments.len() == 4
                        || segments.len() == 7
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2, brute_force)]
fn part2_brute(input: &str) -> u32 {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    input
        .par_lines()
        .map(|line| {
            let (signals, outputs) = line.split_once("|").unwrap();

            // For all permutations of char mappings
            for mapping in chars.iter().permutations(7) {
                // Build a hashmap of char -> char mappings by converting to index and then to char
                // e.g.
                // [d,e,a,f,g,b,c] -> (d,0),(e,1),(a,2),(f,4),(g,5),(b,6),(c,7) -> (d,a),(e,b),(a,c),(f,d),(g,e),(b,f),(c,g)
                let map = mapping
                    .iter()
                    .enumerate()
                    .map(|(index, &&letter)| (letter, (index + 97) as u8 as char))
                    .collect::<HashMap<_, _>>();

                let mut works = true;
                for signal in signals.split_ascii_whitespace() {
                    let display = signal
                        .chars()
                        .map(|x| *map.get(&x).unwrap())
                        .map(char_to_bit_flag)
                        .sum();
                    if !DISPLAYS.contains_key(&display) {
                        works = false;
                        break;
                    }
                }
                if !works {
                    // Didn't work for at least one signal, try the next mapping
                    continue;
                }
                // Found valid mappings
                return outputs
                    .split_ascii_whitespace()
                    .rev()
                    .enumerate()
                    .map(|(index, output)| {
                        // We immediately know what some of them will be based on their length
                        let value = match output.len() {
                            2 => 1,
                            3 => 7,
                            4 => 4,
                            7 => 8,
                            _ => {
                                // For all others decode using the mappings
                                let display = output
                                    .chars()
                                    .map(|x| *map.get(&x).unwrap())
                                    .map(char_to_bit_flag)
                                    .sum();
                                *DISPLAYS.get(&display).unwrap()
                            }
                        };
                        // Multiply by 10^index to shift into the right digit position and sum
                        value * (10_u32.pow(index as u32))
                    })
                    .sum();
            }
            0
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> u32 {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    // Calculate the frequency of each char across all displays
    let mut frequency_map = vec![Vec::new(); 10];
    for (z, x) in chars.map(|c| (c, char_to_bit_flag(c))) {
        let frequency = DISPLAYS
            .keys()
            .filter(|&&display| (display & x) != 0)
            .count();
        frequency_map[frequency].push(z);
    }

    input
        .lines()
        .map(|line| {
            let (mut signals, outputs) = line
                .split('|')
                .map(|part| part.split_ascii_whitespace().collect::<Vec<_>>())
                .collect_tuple()
                .unwrap();

            // For each line, work out the mappings needed to decode the outputs
            let mut mapped: HashMap<char, u8> = HashMap::new();

            // We will have a sample of each digit in signals
            assert_eq!(signals.len(), 10);

            // We know that 1 (cf) and 7 (acf) have only 1 char different (a)
            // Sort from shortest to longest
            signals.sort_by_key(|s| s.len());
            let digit_1 = signals[0];
            let digit_7 = signals[1];
            let a = digit_7
                .chars()
                .find(|&c| !digit_1.chars().any(|x| x == c))
                .unwrap();
            mapped.insert(a, char_to_bit_flag('a'));

            // We can deduce the rest of the digits based on their frequency
            for z in chars {
                let frequency = signals
                    .iter()
                    .filter(|display| display.chars().any(|x| x == z))
                    .count();
                let candidates = &frequency_map[frequency];
                if candidates.len() == 1 {
                    // Only one possibility
                    mapped.insert(z, char_to_bit_flag(*candidates.first().unwrap()));
                } else if candidates.len() == 2 {
                    // Two options, work it out which is which
                    if z == a {
                        // Already know what a is so can skip it
                    } else if candidates.iter().any(|&x| x == 'a') {
                        // This pair of possibilities contains the one we know must be 'a' so can work out the other
                        let other = candidates.iter().find(|&&x| x != 'a').unwrap();
                        mapped.insert(z, char_to_bit_flag(*other));
                    } else {
                        // This pair of possibilities must be d and g
                        // d is present in 4 (which will be the 4th signal due to it's number of chars) but 7 isn't
                        if let Some(d) = signals[2].chars().find(|&x| x == z) {
                            mapped.insert(d, char_to_bit_flag('d'));
                        } else {
                            // Must be g
                            mapped.insert(z, char_to_bit_flag('g'));
                        }
                    }
                }
            }

            assert_eq!(mapped.len(), 7);

            // Found valid mappings, work out what digit each signal actually is
            outputs
                .iter()
                .rev()
                .enumerate()
                .map(|(index, output)| {
                    // We immediately know what some of them will be based on their length
                    let value = match output.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        _ => {
                            // For all others decode using the mappings
                            let display = output.chars().map(|x| *mapped.get(&x).unwrap()).sum();
                            *DISPLAYS.get(&display).unwrap()
                        }
                    };
                    // Multiply by 10^index to shift into the right digit position and sum
                    value * (10_u32.pow(index as u32))
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"};
    static EXAMPLE_INPUT_SINGLE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 26);
    }

    #[test]
    fn test_part2_brute_example() {
        assert_eq!(part2_brute(EXAMPLE_INPUT_SINGLE), 5353);
        assert_eq!(part2_brute(EXAMPLE_INPUT), 61229);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT_SINGLE), 5353);
        assert_eq!(part2(EXAMPLE_INPUT), 61229);
    }
}
