use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day4)]
fn gen(input: &str) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let mut lines = input.lines();

    let numbers = lines.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut boards = Vec::new();
    let mut board = Vec::new();


    for line in lines {
        if line.is_empty() {
            if !board.is_empty() {
                boards.push(board);
            }
            // Start a new board;
            board = Vec::new();
            continue;
        }
        board.push(line.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>());
    }

    (numbers, boards)
}

#[aoc(day4, part1)]
fn part1(input: &(Vec<usize>, Vec<Vec<Vec<usize>>>)) -> usize {
    let (numbers, boards) = input;
    let mut marked = Vec::new();
    for _ in boards {
        marked.push(vec![vec![false; 5]; 5]);
    }
    let mut map = Vec::new();
    for board in boards {
        let mut board_map = HashMap::new();
        for (x, row) in board.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                board_map.insert(col, (x, y));
            }
        }
        map.push(board_map);
    }
    for number in numbers {
        for (b_index, board) in map.iter().enumerate() {
            if let Some(&(x, y)) = board.get(number) {
                // Number is on the board - mark it
                marked[b_index][x][y] = true;
            }
        }        
        for (b_index, board) in marked.iter().enumerate() {
            for row in 0..5 {
                if (0..5).all(|y| board[row][y]) {
                    // Found a complete row
                    let mut unmarked = 0;
                    for (&n, &(x, y)) in &map[b_index] {
                        if !board[x][y] {
                            unmarked += *n;
                        }
                    }
                    return unmarked * number;
                }
            }
            for col in 0..5 {
                if (0..5).all(|x| board[x][col]) {
                    // Found a complete column
                    let mut unmarked = 0;
                    for (&n, &(x, y)) in &map[b_index] {
                        if !board[x][y] {
                            unmarked += *n;
                        }
                    }
                    return unmarked * number;
                }
            }
        }
    }
    0
}

#[aoc(day4, part2)]
fn part2(input: &(Vec<usize>, Vec<Vec<Vec<usize>>>)) -> usize {
    let (numbers, boards) = input;
    let mut marked = Vec::new();
    let mut loosing = HashSet::new();
    let mut index = 0;
    for _ in boards {
        marked.push(vec![vec![false; 5]; 5]);
        loosing.insert(index);
        index += 1;
    }
    let mut map = Vec::new();
    for board in boards {
        let mut board_map = HashMap::new();
        for (x, row) in board.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                board_map.insert(col, (x, y));
            }
        }
        map.push(board_map);
    }
    for number in numbers {
        for (b_index, board) in map.iter().enumerate() {
            if let Some(&(x, y)) = board.get(number) {
                // Number is on the board - mark it
                marked[b_index][x][y] = true;
            }
        }        
        for (b_index, board) in marked.iter().enumerate() {
            if !loosing.contains(&b_index) {
                continue;
            }
            for row in 0..5 {
                if (0..5).all(|y| board[row][y]) {
                    // Found a complete row
                    if loosing.len() == 1 {
                        let looser = *loosing.iter().next().unwrap();
                        let mut unmarked = 0;
                        for (&n, &(x, y)) in &map[b_index] {
                            if !board[x][y] {
                                unmarked += *n;
                            }
                        }
                        return unmarked * number;
                    }
                    loosing.remove(&b_index);
                }
            }
            for col in 0..5 {
                if (0..5).all(|x| board[x][col]) {
                    // Found a complete column
                    if loosing.len() == 1 {
                        let mut unmarked = 0;
                        for (&n, &(x, y)) in &map[b_index] {
                            if !board[x][y] {
                                unmarked += *n;
                            }
                        }
                        return unmarked * number;
                    }
                    loosing.remove(&b_index);
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 4512);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 0);
    }
}