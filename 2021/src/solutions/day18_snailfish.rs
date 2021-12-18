use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, PartialEq)]
struct SailFishNumber {
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
    Pair(Box<SailFishNumber>),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Element::Regular(value) => write!(f, "{:?}", value),
            Element::Pair(number) => write!(f, "{:?}", number),
        }
    }
}

impl fmt::Debug for SailFishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.x, self.y)
    }
}

impl SailFishNumber {
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
        let x = match &self.x {
            Element::Regular(value) => *value,
            Element::Pair(number) => number.magnitude(),
        };
        let y = match &self.y {
            Element::Regular(value) => *value,
            Element::Pair(number) => number.magnitude(),
        };
        (3 * x) + (2 * y)
    }

    fn allocate_exploded_y(&mut self, leftover: u32) -> u32 {
        match &self.x {
            Element::Regular(value) => {
                self.x = Element::Regular(*value + leftover);
                return 0;
            }
            Element::Pair(number) => {
                let mut number = *number.clone();
                let remaining = number.allocate_exploded_y(leftover);
                self.x = Element::Pair(Box::new(number));
                if remaining == 0 {
                    return 0;
                }
            }
        }
        leftover
    }

    fn allocate_exploded_x(&mut self, leftover: u32) -> u32 {
        match &self.y {
            Element::Regular(value) => {
                self.y = Element::Regular(*value + leftover);
                return 0;
            }
            Element::Pair(number) => {
                let mut number = *number.clone();
                let remaining = number.allocate_exploded_x(leftover);
                self.y = Element::Pair(Box::new(number));
                if remaining == 0 {
                    return 0;
                }
            }
        }
        leftover
    }

    fn reduce(&mut self) -> ReduceResult {
        let result = self.explode(0);
        if let ReduceResult::Reduced = result {
            return self.split();
        }
        result
    }

    fn split(&mut self) -> ReduceResult {
        match &self.x {
            Element::Regular(value) => {
                if *value >= 10 {
                    self.x = Element::split(*value);
                    return ReduceResult::Split;
                }
            }
            Element::Pair(number) => {
                let mut number = *number.clone();
                let reduced = number.split();
                self.x = Element::Pair(Box::new(number));
                if let ReduceResult::Split = reduced {
                    return reduced;
                }
            }
        }
        match &self.y {
            Element::Regular(value) => {
                if *value >= 10 {
                    self.y = Element::split(*value);
                    return ReduceResult::Split;
                }
            }
            Element::Pair(number) => {
                let mut number = *number.clone();
                let reduced = number.split();
                self.y = Element::Pair(Box::new(number));
                if let ReduceResult::Split = reduced {
                    return reduced;
                }
            }
        }
        ReduceResult::Reduced
    }

    fn explode(&mut self, depth: usize) -> ReduceResult {
        if let Element::Pair(number) = &self.x {
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
                match &self.y {
                    Element::Regular(has) => {
                        //println!("Allocating {} right to {}", y, has);
                        self.y = Element::Regular(has + y);
                        return ReduceResult::Exploded((x, 0));
                    }
                    Element::Pair(other_number) => {
                        let mut other_number = *other_number.clone();
                        let remaining = other_number.allocate_exploded_y(y);
                        self.y = Element::Pair(Box::new(other_number));
                        if remaining == 0 {
                            return ReduceResult::Exploded((x, 0));
                        }
                        // Failed to allocate, pass up the chain
                        return ReduceResult::Exploded((x, y));
                    }
                }
            }
            let mut number = *number.clone();
            let reduced = number.explode(depth + 1);
            self.x = Element::Pair(Box::new(number));
            if reduced != ReduceResult::Reduced {
                if let ReduceResult::Exploded((x, y)) = reduced {
                    if y > 0 {
                        // Try to allocate y to the right
                        match &self.y {
                            Element::Regular(has) => {
                                //println!("Allocating {} right to {}", y, has);
                                self.y = Element::Regular(has + y);
                                return ReduceResult::Exploded((x, 0));
                            }
                            Element::Pair(other_number) => {
                                let mut other_number = *other_number.clone();
                                let remaining = other_number.allocate_exploded_y(y);
                                self.y = Element::Pair(Box::new(other_number));
                                if remaining == 0 {
                                    return ReduceResult::Exploded((x, 0));
                                }
                                // Failed to allocate, pass up the chain
                                return ReduceResult::Exploded((x, y));
                            }
                        }
                    }
                }
                return reduced;
            }
        }
        if let Element::Pair(number) = &self.y {
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
                match &self.x {
                    Element::Regular(has) => {
                        self.x = Element::Regular(has + x);
                        return ReduceResult::Exploded((0, y));
                    }
                    Element::Pair(other_number) => {
                        let mut other_number = *other_number.clone();
                        let remaining = other_number.allocate_exploded_x(x);
                        self.x = Element::Pair(Box::new(other_number));
                        if remaining == 0 {
                            return ReduceResult::Exploded((0, y));
                        }
                        // Failed to allocate, pass up the chain
                        return ReduceResult::Exploded((x, y));
                    }
                }
            }
            let mut number = *number.clone();
            let reduced = number.explode(depth + 1);
            self.y = Element::Pair(Box::new(number));
            if reduced != ReduceResult::Reduced {
                if let ReduceResult::Exploded((x, y)) = reduced {
                    if x > 0 {
                        // Try to allocate x to the left
                        match &self.x {
                            Element::Regular(has) => {
                                self.x = Element::Regular(has + x);
                                return ReduceResult::Exploded((0, y));
                            }
                            Element::Pair(other_number) => {
                                let mut other_number = *other_number.clone();
                                let remaining = other_number.allocate_exploded_x(x);
                                if remaining == 0 {
                                    self.x = Element::Pair(Box::new(other_number));
                                    return ReduceResult::Exploded((0, y));
                                }
                                // Failed to allocate, pass up the chain
                                return ReduceResult::Exploded((x, y));
                            }
                        }
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
    fn split(value: u32) -> Self {
        let v1 = value / 2;
        let v2 = (value / 2) + (value % 2);
        Self::Pair(Box::new(SailFishNumber {
            x: Self::Regular(v1),
            y: Self::Regular(v2),
        }))
    }
}

/// Recursively parse a `SailFishNumber`
fn parse(iter: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> SailFishNumber {
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
    SailFishNumber { x, y }
}

#[aoc_generator(day18)]
fn gen(input: &str) -> VecDeque<SailFishNumber> {
    input
        .lines()
        .map(|line| parse(&mut line.chars().peekable()))
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &VecDeque<SailFishNumber>) -> u32 {
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
fn part2(input: &VecDeque<SailFishNumber>) -> u32 {
    // Find which permutation of two numbers results in the biggest magnitude
    input
        .iter()
        .permutations(2)
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
