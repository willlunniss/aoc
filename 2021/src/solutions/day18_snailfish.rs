use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, PartialEq)]
struct Number {
    x: Element,
    y: Element,
}

#[derive(Debug, PartialEq)]
enum ReduceResult {
    Reduced,
    Split,
    Exploded((u32, u32)),
}

#[derive(Clone, PartialEq)]
enum Element {
    Regular(u32),
    Pair(Box<Number>),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Element::Regular(value) => write!(f, "{:?}", value),
            Element::Pair(number) => write!(f, "{:?}", number),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.x, self.y)
    }
}

impl Number {
    /// Adds a number and reduces the result
    fn add(self, added: Self) -> Self {
        let mut number = Self {
            x: Element::Pair(Box::new(self)),
            y: Element::Pair(Box::new(added)),
        };

        loop {
            let outcome = number.reduce();
            if let ReduceResult::Reduced = outcome {
                break;
            }
        }

        number
    }

    fn magnitude(&self) -> u32 {
        (3 * self.x.magnitude()) + (2 * self.y.magnitude())
    }

    fn reduce(&mut self) -> ReduceResult {
        let result = self.explode(0);
        if let ReduceResult::Reduced = result {
            return self.split();
        }
        result
    }

    fn split(&mut self) -> ReduceResult {
        if self.x.split_if_needed() || self.y.split_if_needed() {
            ReduceResult::Split
        } else {
            ReduceResult::Reduced
        }
    }

    fn explode(&mut self, depth: usize) -> ReduceResult {
        if let Element::Pair(ref mut number) = self.x {
            if depth == 3 {
                // Explode
                let mut y = 0;
                let mut x = 0;
                if let Element::Regular(exploded) = number.x {
                    x = exploded;
                };
                if let Element::Regular(exploded) = number.y {
                    y = exploded;
                };
                self.x = Element::Regular(0);
                // Try to allocate y to the right
                if self.y.try_allocate_right(y) {
                    return ReduceResult::Exploded((x, 0));
                }
                // Failed to allocate, pass up the chain
                return ReduceResult::Exploded((x, y));
            }
            let reduced = number.explode(depth + 1);
            if reduced != ReduceResult::Reduced {
                if let ReduceResult::Exploded((x, y)) = reduced {
                    if y > 0 {
                        // Try to allocate y to the right
                        if self.y.try_allocate_right(y) {
                            return ReduceResult::Exploded((x, 0));
                        }
                        // Failed to allocate, pass up the chain
                        return ReduceResult::Exploded((x, y));
                    }
                }
                return reduced;
            }
        }
        if let Element::Pair(ref mut number) = self.y {
            if depth == 3 {
                // Explode
                let mut y = 0;
                let mut x = 0;
                if let Element::Regular(exploded) = number.x {
                    x = exploded;
                };
                if let Element::Regular(exploded) = number.y {
                    y = exploded;
                };
                self.y = Element::Regular(0);
                // Try to allocate x to the left
                if self.x.try_allocate_left(x) {
                    return ReduceResult::Exploded((0, y));
                }
                // Failed to allocate, pass up the chain
                return ReduceResult::Exploded((x, y));
            }
            let reduced = number.explode(depth + 1);
            if reduced != ReduceResult::Reduced {
                if let ReduceResult::Exploded((x, y)) = reduced {
                    if x > 0 {
                        // Try to allocate x to the left
                        if self.x.try_allocate_left(x) {
                            return ReduceResult::Exploded((0, y));
                        }
                        // Failed to allocate, pass up the chain
                        return ReduceResult::Exploded((x, y));
                    }
                }
                return reduced;
            }
        }
        ReduceResult::Reduced
    }
}

impl Element {
    /// Split's a Regular Element into a Pair of elements
    fn split_value(value: u32) -> Self {
        let v1 = value / 2;
        let v2 = (value / 2) + (value % 2);
        Self::Pair(Box::new(Number {
            x: Self::Regular(v1),
            y: Self::Regular(v2),
        }))
    }

    /// Try to allocate an `amount` to the left (of an explosion)
    ///
    /// Returns `true` if it is allocated, `false` if not
    fn try_allocate_left(&mut self, amount: u32) -> bool {
        match self {
            Element::Regular(value) => {
                *value += amount;
                true
            }
            Element::Pair(number) => {
                number.y.try_allocate_left(amount) || number.x.try_allocate_left(amount)
            }
        }
    }

    /// Try to allocate an `amount` to the right (of an explosion)
    ///
    /// Returns `true` if it is allocated, `false` if not
    fn try_allocate_right(&mut self, amount: u32) -> bool {
        match self {
            Element::Regular(value) => {
                *value += amount;
                true
            }
            Element::Pair(number) => {
                number.x.try_allocate_right(amount) || number.y.try_allocate_right(amount)
            }
        }
    }

    /// Recursively splits elements if needed
    ///
    /// Returns `true` if or any sub ones were split, `false` if it was not
    fn split_if_needed(&mut self) -> bool {
        match self {
            Element::Regular(value) => {
                if *value >= 10 {
                    *self = Self::split_value(*value);
                    return true;
                }
                false
            }
            Element::Pair(number) => number.split() == ReduceResult::Split,
        }
    }

    /// Recursively calculates the magnitude of the number
    fn magnitude(&self) -> u32 {
        match self {
            Element::Regular(value) => *value,
            Element::Pair(number) => number.magnitude(),
        }
    }
}

/// Recursively parse a `Number`
fn parse(iter: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Number {
    assert_eq!(iter.next().unwrap(), '[');
    let x = if let Some('[') = iter.peek() {
        Element::Pair(Box::new(parse(iter)))
    } else {
        Element::Regular(iter.next().unwrap().to_digit(10).unwrap())
    };
    assert_eq!(iter.next().unwrap(), ',');
    let y = if let Some('[') = iter.peek() {
        Element::Pair(Box::new(parse(iter)))
    } else {
        Element::Regular(iter.next().unwrap().to_digit(10).unwrap())
    };
    assert_eq!(iter.next().unwrap(), ']');
    Number { x, y }
}

#[aoc_generator(day18)]
fn gen(input: &str) -> VecDeque<Number> {
    input
        .lines()
        .map(|line| parse(&mut line.chars().peekable()))
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &VecDeque<Number>) -> u32 {
    let mut numbers = input.clone();

    // Add all numbers together
    let mut result = numbers.pop_front().unwrap();
    while let Some(next) = numbers.pop_front() {
        result = result.add(next);
    }
    // Return the magnitude
    result.magnitude()
}

#[aoc(day18, part2)]
fn part2(input: &VecDeque<Number>) -> u32 {
    // Find which permutation of two numbers results in the biggest magnitude
    let permutations = input.iter().permutations(2).collect::<Vec<_>>();
    permutations
        .par_iter()
        .map(|numbers| {
            let (&a, &b) = numbers.iter().collect_tuple().unwrap();
            a.clone().add(b.clone()).magnitude()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 4140);
    }

    #[test]
    fn test_explode_1() {
        let mut sfn = parse(&mut "[[[[[9,8],1],2],3],4]".chars().peekable());
        let result = sfn.reduce();
        assert_eq!(result, ReduceResult::Exploded((9, 0)));
        let result = sfn.reduce();
        assert_eq!(result, ReduceResult::Reduced);
    }
    #[test]
    fn test_explode_2() {
        let mut sfn = parse(&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars().peekable());
        let result = sfn.reduce();
        assert_eq!(result, ReduceResult::Exploded((0, 0)));
        let result = sfn.reduce();
        assert_eq!(result, ReduceResult::Exploded((0, 2)));
        let result = sfn.reduce();
        assert_eq!(result, ReduceResult::Reduced);
        assert_eq!(
            sfn,
            parse(&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars().peekable())
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(part1(&gen("[9,1]")), 29);
        assert_eq!(part1(&gen("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(part1(&gen("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(part1(&gen("[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(part1(&gen("[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
        assert_eq!(
            part1(&gen(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }
}
