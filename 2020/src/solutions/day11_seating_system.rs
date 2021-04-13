use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Occupied => write!(f, "#"),
            State::Empty => write!(f, "L"),
            _ => write!(f, "."),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    ImmediateNeighbour,
    FirstVisibleSeat,
}

impl State {
    /// Create a new State from a char
    pub fn new(c: char) -> State {
        match c {
            'L' => State::Empty,
            '#' => State::Occupied,
            '.' => State::Floor,
            _ => {
                panic!("Unexpected state '{}'", c)
            }
        }
    }
}

#[aoc_generator(day11)]
pub fn gen(input: &str) -> Vec<Vec<State>> {
    let mut data: Vec<Vec<State>> = Vec::new();

    for line in input.lines() {
        let row = line.chars().map(|x| State::new(x)).collect::<Vec<State>>();
        data.push(row);
    }
    return data;
}

// TODO Do this statically
fn angles() -> Vec<(isize, isize)> {
    let mut angles = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            if (y, x) != (0, 0) {
                angles.push((y, x))
            }
        }
    }
    return angles;
}

fn get_pos(
    height: usize,
    width: usize,
    start: (usize, usize),
    angle: (isize, isize),
    range: isize,
) -> Option<(usize, usize)> {
    let y = start.0 as isize + (angle.0 * range);
    let x = start.1 as isize + (angle.1 * range);
    if y >= 0 && (y as usize) < height && x >= 0 && (x as usize) < width {
        return Some((y as usize, x as usize));
    }
    return None;
}

fn occupied_seats_from_pos(input: &Vec<Vec<State>>, pos: (usize, usize), mode: Mode) -> usize {
    let width = input.first().unwrap().len();
    let height = input.len();

    let mut count = 0;
    for angle in angles() {
        let mut range = 1;
        loop {
            let neighbour = get_pos(height, width, pos, angle, range);
            if neighbour == None {
                // Reached edge of grid, move to next angle
                break;
            } else {
                // See if there is a seat here
                let (y, x) = neighbour.unwrap();
                let state = input[y][x];
                if state == State::Floor && mode == Mode::FirstVisibleSeat {
                    // No seat and using first visible, increase range and try again
                    range += 1;
                    continue;
                }
                if state == State::Occupied {
                    count += 1;
                }
                // Found a seat
                break;
            }
        }
    }
    return count;
}

fn count_occupied(input: &Vec<Vec<State>>) -> usize {
    let width = input.first().unwrap().len();
    let height = input.len();
    let mut occupied_seats = 0;
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == State::Occupied {
                occupied_seats += 1;
            }
        }
    }
    return occupied_seats;
}

fn simulate_seating(input: &Vec<Vec<State>>, occupied_limit: usize, mode: Mode) -> Vec<Vec<State>> {
    // Create two copies as we need to apply state changes simultaneously to all positions
    let mut previous: Vec<Vec<State>> = input.clone();
    let mut next: Vec<Vec<State>> = input.clone();

    let width = previous.first().unwrap().len();
    let height = previous.len();
    let mut changed = true;

    while changed {
        changed = false;
        std::mem::swap(&mut next, &mut previous);
        for y in 0..height {
            for x in 0..width {
                let old = previous[y][x];
                let occupied_seats = occupied_seats_from_pos(&previous, (y, x), mode);
                if old == State::Occupied && occupied_seats >= occupied_limit {
                    changed = true;
                    next[y][x] = State::Empty;
                } else if old == State::Empty && occupied_seats == 0 {
                    changed = true;
                    next[y][x] = State::Occupied;
                } else {
                    next[y][x] = old; // No change - just copy it over
                }
            }
        }
    }

    return next;
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<State>>) -> usize {
    let layout = simulate_seating(input, 4, Mode::ImmediateNeighbour);
    return count_occupied(&layout);
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<State>>) -> usize {
    let layout = simulate_seating(input, 5, Mode::FirstVisibleSeat);
    return count_occupied(&layout);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pos() {
        assert_eq!(get_pos(5, 5, (0, 0), (-1, -1), 1), None);
        assert_eq!(get_pos(5, 5, (2, 2), (-1, -1), 1), Some((1, 1)));
        assert_eq!(get_pos(5, 5, (2, 2), (-1, -1), 2), Some((0, 0)));
    }
}
