use utils::grid::{Direction, MapGrid, Pos};

/// Implements an integrator for allocating positions in
/// a spiral pattern like this:
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23---> ...
struct SpiralMemory {
    pos: Pos,
    layer: isize,
    dir: Direction,
}

impl Default for SpiralMemory {
    fn default() -> Self {
        Self {
            pos: Pos { x: -1, y: 0 },
            layer: 0,
            dir: Direction::Right,
        }
    }
}

impl Iterator for SpiralMemory {
    type Item = Pos;

    fn next(&mut self) -> Option<Pos> {
        // Walk outwards from the center in an anti-clockwise spiral
        self.pos = self.pos.next(self.dir);
        // Work out where we will go for the next value
        if self.dir == Direction::Right && self.pos.x == self.layer + 1 {
            self.dir = Direction::Up;
            // Move out to the next layer
            self.layer += 1;
        } else if self.dir == Direction::Up && self.pos.y == -self.layer {
            self.dir = Direction::Left;
        } else if self.dir == Direction::Left && self.pos.x == -self.layer {
            self.dir = Direction::Down;
        } else if self.dir == Direction::Down && self.pos.y == self.layer {
            self.dir = Direction::Right;
        }
        Some(self.pos)
    }
}

#[aoc_generator(day3)]
fn gen(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &usize) -> Option<usize> {
    let mut grid = MapGrid::new();
    grid.insert(Pos::new(0, 0), 1);
    // Build the grid until we have allocated input value positions
    for (index, pos) in SpiralMemory::default().enumerate() {
        let value = index + 1;
        grid.insert(pos, value);
        if value == *input {
            // Then return the manhattan distance
            return Some(pos.manhattan_distance_origin());
        }
    }
    None
}

#[aoc(day3, part2)]
fn part2(input: &usize) -> Option<usize> {
    // Init the grid with a 1 at 0,0
    let mut grid = MapGrid::new();
    grid.insert(Pos::new(0, 0), 1);
    // Build the grid by setting value to the sum of all neighbours
    for pos in SpiralMemory::default().skip(1) {
        let sum = pos.neighbours8().filter_map(|n| grid.get(&n)).sum();
        if sum > *input {
            // Return at the first sum that is bigger than the input
            return Some(sum);
        }
        grid.insert(pos, sum);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&1), Some(0));
        assert_eq!(part1(&9), Some(2));
        assert_eq!(part1(&12), Some(3));
        assert_eq!(part1(&23), Some(2));
        assert_eq!(part1(&1024), Some(31));
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&361_527), Some(326));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&361_527), Some(363_010));
    }
}
