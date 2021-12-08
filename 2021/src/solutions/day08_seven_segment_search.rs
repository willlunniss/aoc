use itertools::Itertools;
use std::collections::HashMap;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, outputs) = line.split_once("|").unwrap();
            outputs
                .split_ascii_whitespace()
                .filter(|digit| {
                    digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
                })
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    // Build map of all valid post-mapped outputs to their 7 segment value
    let mut valid = HashMap::new();
    valid.insert("abcefg".to_owned(), 0);
    valid.insert("cf".to_owned(), 1);
    valid.insert("acdeg".to_owned(), 2);
    valid.insert("acdfg".to_owned(), 3);
    valid.insert("bcdf".to_owned(), 4);
    valid.insert("abdfg".to_owned(), 5);
    valid.insert("abdefg".to_owned(), 6);
    valid.insert("acf".to_owned(), 7);
    valid.insert("abcdefg".to_owned(), 8);
    valid.insert("abcdfg".to_owned(), 9);

    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    input
        .lines()
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
                for test in signals.split_ascii_whitespace() {
                    let mut mapped = Vec::new();
                    for c in test.chars() {
                        mapped.push(*map.get(&c).unwrap());
                    }
                    let result = mapped.iter().sorted().collect::<String>();
                    if !valid.contains_key(&result) {
                        works = false;
                        break;
                    }
                }
                if !works {
                    // Didn't work for at least one signal, try the next mapping
                    continue;
                }
                // Found valid mappings
                let mut total = 0;
                for (index, output) in outputs.split_ascii_whitespace().rev().enumerate() {
                    // For each output (last -> first), map it and look up which digit it relates to
                    let mut mapped = Vec::new();
                    for c in output.chars() {
                        mapped.push(*map.get(&c).unwrap());
                    }
                    let result = mapped.iter().sorted().collect::<String>();
                    let value = valid.get(&result).unwrap();
                    // Multiple by 10^index to shift into the right digit position and sum
                    total += value * (10_usize.pow(index as u32));
                }
                return total;
            }
            0
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
    static EXAMPLE_INPUT_SMALL: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&EXAMPLE_INPUT), 26);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&EXAMPLE_INPUT_SMALL), 5353);
        assert_eq!(part2(&EXAMPLE_INPUT), 61229);
    }
}
