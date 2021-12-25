use utils::grid::{Pos, VecGrid};

#[aoc_generator(day25)]
fn gen(input: &str) -> VecGrid<char> {
    input.parse().unwrap()
}

#[aoc(day25, part1)]
fn part1(input: &VecGrid<char>) -> usize {
    let mut grid = input.clone();
    let width = grid.width();
    let height = grid.height();
    for step in 1.. {
        let mut moves = 0;
        for phase in ['>', 'v'] {
            let mut next = VecGrid::new_sized('.', width, height);
            for (pos, &cucumber) in grid.into_iter().filter(|(_, &x)| x != '.') {
                if cucumber == phase {
                    // Work out where this cucumber would like to move to next
                    let target = if cucumber == '>' {
                        Pos::from(((pos.x + 1) % width as isize, pos.y))
                    } else {
                        Pos::from((pos.x, (pos.y + 1) % height as isize))
                    };
                    if grid.get(target) == Some('.') {
                        // target is free - move
                        next[pos] = '.';
                        next[target] = cucumber;
                        moves += 1;
                    } else {
                        // not free - stay in the same position
                        next[pos] = cucumber;
                    }
                } else {
                    next[pos] = cucumber;
                }
            }
            std::mem::swap(&mut next, &mut grid);
        }
        if moves == 0 {
            // Sea cucumbers have stopped moving, return step number as the result
            return step;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 58);
    }
}
