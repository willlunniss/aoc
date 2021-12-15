use num_integer::Integer;
use std::collections::{HashMap, HashSet};
use utils::grid::Pos;
use utils::grid::VecGrid;

#[aoc_generator(day15)]
fn gen(input: &str) -> VecGrid<u8> {
    // Read in the grid, treating each char as a u8
    VecGrid::from(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| char::to_digit(c, 10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

/// Find the 'lowest risk' path from top left to bottom right corner based
fn dijkstra(map: &VecGrid<u8>) -> Option<usize> {
    let target = Pos::from((map.width() - 1, map.height() - 1));

    // Initialise to max risk for each point
    let mut risks = VecGrid::new_sized(usize::MAX, map.width(), map.height());
    // Not having visited anywhere
    let mut unvisisted = map.indexes().collect::<HashSet<_>>();

    // Places to consider exploring next, starting at (0,0)
    let mut candidates: HashMap<Pos, usize> = HashMap::new();
    candidates.insert(Pos::new(0, 0), 0);
    while !candidates.is_empty() {
        // Explore the candidate with the lowest risk next
        let (&pos, &total_risk) = candidates.iter().min_by_key(|(_, &risk)| risk).unwrap();
        candidates.remove(&pos);

        if pos == target {
            // Done!
            return Some(total_risk);
        }
        for (_, next, risk) in map.neighbours_ex(pos) {
            if let Some(risk) = risk {
                if unvisisted.contains(&next) {
                    // Not explored yet
                    let new_risk = total_risk + risk as usize;
                    if risks[next] > new_risk {
                        // Got here with a better risk level
                        risks[next] = new_risk;
                        candidates.insert(next, new_risk);
                    }
                }
            }
        }
        // Finished visiting this position
        unvisisted.remove(&pos);
    }
    None
}

#[aoc(day15, part1)]
fn part1(input: &VecGrid<u8>) -> usize {
    // Find the lowest risk path
    dijkstra(input).unwrap()
}

#[aoc(day15, part2)]
fn part2(input: &VecGrid<u8>) -> usize {
    // Build the full map from our sub segments
    let mut map = VecGrid::new_sized(0, input.width() * 5, input.height() * 5);
    for row in 0..5_u8 {
        for col in 0..5_u8 {
            let shift = (row as usize * input.width(), col as usize * input.height());
            for (pos, value) in input {
                // Increase the value, wrapping back around to 1 when it gets above 9
                let (div, rem) = (value + row + col).div_mod_floor(&10);
                map.insert(pos + shift, div + rem);
            }
        }
    }

    // Find the lowest risk path
    dijkstra(&map).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 40);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 315);
    }
}
