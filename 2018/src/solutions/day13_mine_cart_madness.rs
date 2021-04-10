use std::collections::BTreeMap;
use utils::grid::{Direction, Pos, VecGrid};

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Cart {
    direction: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(c: char) -> Self {
        Self {
            direction: match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                '<' => Direction::Left,
                'v' => Direction::Down,
                _ => panic!("Unexpected cart direction {}", c),
            },
            next_turn: Turn::Left,
        }
    }

    /// Turns the cart at an intersection by cycling through the options
    fn turn(&mut self) {
        self.direction = match self.next_turn {
            Turn::Left => {
                self.next_turn = Turn::Straight;
                self.direction.rotate_left()
            }
            Turn::Straight => {
                self.next_turn = Turn::Right;
                self.direction
            }
            Turn::Right => {
                self.next_turn = Turn::Left;
                self.direction.rotate_right()
            }
        };
    }
}

/// Simulates the carts moving on the track
///
/// If `remove_on_crash` then carts will be removed as they crash and eventually the position
/// of the one remaining cart will be returned
/// Otherwise the position of the first crash will be returned
fn simulate(carts: &BTreeMap<Pos, Cart>, tracks: &VecGrid<char>, remove_on_crash: bool) -> Pos {
    let mut carts = carts.clone();
    loop {
        // Process each cart ordered by position from the top left to bottom right
        for position in &carts.keys().copied().collect::<Vec<_>>() {
            // Get the cart (may not exist if it's been crashed into)
            if let Some(mut cart) = carts.remove(position) {
                // Move the cart forward and check for collisions
                let next = position.next(cart.direction);
                if carts.contains_key(&next) {
                    // Crashed into another cart!
                    if remove_on_crash {
                        // Remove the cart we crashed into and then continue
                        // without adding back the current cart (effectively removing it too)
                        carts.remove(&next);
                        continue;
                    }
                    // Return the position of the first crash
                    return next;
                }
                // No collision, handle intersections/corners
                match tracks.get(next).unwrap() {
                    '+' => {
                        // Intersection - turn the cart
                        cart.turn();
                    }
                    '\\' => {
                        // Corner track - rotate the cart
                        cart.direction = match cart.direction {
                            Direction::Down => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                            Direction::Up => Direction::Left,
                        };
                    }
                    '/' => {
                        // Corner track - rotate the cart
                        cart.direction = match cart.direction {
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            Direction::Up => Direction::Right,
                        };
                    }
                    _ => {} // Normal straight track - no change
                }
                // Add updated cart back in at the new position
                carts.insert(next, cart);
            }
        }
        if carts.len() == 1 {
            // Only one cart left, return where it ended up
            return *carts.keys().next().unwrap();
        }
    }
}

#[aoc_generator(day13)]
fn gen(input: &str) -> (BTreeMap<Pos, Cart>, VecGrid<char>) {
    // Load the input into a map
    let mut tracks = VecGrid::from(input.lines().map(|line| line.chars().collect()).collect());
    // Find all the carts and store in a map sorted by position
    let carts = tracks
        .into_iter()
        .filter_map(|(pos, c)| match *c {
            '^' | '>' | '<' | 'v' => Some((pos, Cart::new(*c))),
            _ => None,
        })
        .collect::<BTreeMap<Pos, Cart>>();
    // Replace the carts with plain track in the map
    for (position, cart) in &carts {
        let track = match cart.direction {
            Direction::Up | Direction::Down => '|',
            Direction::Left | Direction::Right => '-',
        };
        tracks.insert(*position, track);
    }

    (carts, tracks)
}

#[aoc(day13, part1)]
fn part1(input: &(BTreeMap<Pos, Cart>, VecGrid<char>)) -> String {
    let (carts, tracks) = input;
    let pos = simulate(carts, tracks, false);
    format!("{},{}", pos.x, pos.y)
}

#[aoc(day13, part2)]
fn part2(input: &(BTreeMap<Pos, Cart>, VecGrid<char>)) -> String {
    let (carts, tracks) = input;
    let pos = simulate(carts, tracks, true);
    format!("{},{}", pos.x, pos.y)
}
