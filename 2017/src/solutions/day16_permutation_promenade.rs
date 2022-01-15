use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.as_bytes();
        match chars[0] as char {
            's' => Ok(Self::Spin(s[1..].parse().unwrap())),
            'x' => {
                let (from, to) = s[1..]
                    .split('/')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                Ok(Self::Exchange(from, to))
            }
            'p' => Ok(Self::Partner(chars[1] as char, chars[3] as char)),
            _ => unreachable!(),
        }
    }
}

fn dance(input: &[DanceMove], dances: usize) -> String {
    let mut programs = if input.len() == 3 {
        // Test mode
        ('a'..='e').collect::<Vec<_>>()
    } else {
        ('a'..='p').collect::<Vec<_>>()
    };
    let initial = programs.clone();
    let len = programs.len();
    let mut indexes = (0..len).collect::<Vec<_>>();
    let mut head = 0;
    let mut dance = 0;
    // For target number of dances
    while dance < dances {
        // Perform all the moves
        for dance_move in input {
            match dance_move {
                DanceMove::Spin(x) => {
                    // Shift head position
                    head = (head + *x) % len;
                }
                DanceMove::Exchange(x, y) => {
                    // Swap programs an indexes x,y
                    // Get index of programs given current head
                    let x = (len - head + *x) % len;
                    let y = (len - head + *y) % len;
                    // Lookup positions in indexes
                    let a = programs[x] as usize - 'a' as usize;
                    let b = programs[y] as usize - 'a' as usize;
                    // Swap
                    programs.swap(x, y);
                    indexes.swap(a, b);
                }
                DanceMove::Partner(a, b) => {
                    // Swap programs a,b
                    // Lookup the positions of a,b
                    let a = *a as usize - 'a' as usize;
                    let b = *b as usize - 'a' as usize;
                    let x = indexes[a];
                    let y = indexes[b];
                    // Swap
                    indexes.swap(a, b);
                    programs.swap(x, y);
                }
            }
        }
        dance += 1;
        // Check to see if we have reached the start ordering yet
        if (0..len)
            .map(|idx| &programs[(head + idx) % len])
            .eq(initial.iter())
        {
            // Hit a repeating cycle!
            let period = dance;
            // Advance to the last point that we will have this position before reaching the target number of dances
            dance = dances - (dances % period);
        }
    }
    // Finished, return the final ordering
    programs.rotate_right(head);
    programs.iter().collect()
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<DanceMove> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day16, part1)]
fn part1(input: &[DanceMove]) -> String {
    dance(input, 1)
}

#[aoc(day16, part2)]
fn part2(input: &[DanceMove]) -> String {
    dance(input, 1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), "baedc");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(dance(&gen(EXAMPLE_INPUT), 2), "ceadb");
        assert_eq!(dance(&gen(EXAMPLE_INPUT), 3), "ecbda");
    }
}
