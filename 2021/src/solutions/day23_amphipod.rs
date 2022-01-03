use itertools::Either;
use pathfinding::prelude::*;
use std::ops::Sub;
use utils::grid::{Pos, VecGrid};

fn abs_diff<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn requires_energy(amphipod: char) -> usize {
    match amphipod {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

fn room(amphipod: char) -> usize {
    match amphipod {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => unreachable!(),
    }
}

fn owner(room: usize) -> char {
    match room {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => unreachable!(),
    }
}

fn room_entrance(amphipod: char) -> usize {
    match amphipod {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => unreachable!(),
    }
}

const fn is_entrance(index: usize) -> bool {
    matches!(index, 2 | 4 | 6 | 8)
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State<const SLOTS: usize> {
    hallway: [char; 11],
    rooms: [[char; SLOTS]; 4],
}

impl<const SLOTS: usize> State<SLOTS> {
    fn new(input: &VecGrid<char>) -> Self {
        let mut rooms = [['?'; SLOTS]; 4];
        let mut hallway = ['?'; 11];
        for idx in 0..hallway.len() {
            hallway[idx] = input[Pos::new(1 + idx, 1)];
        }
        for room in 0..rooms.len() {
            for slot in 0..SLOTS {
                rooms[room][slot] = input[Pos::new(3 + (room * 2), 2 + slot)];
            }
        }
        Self { hallway, rooms }
    }

    fn organised(&self) -> bool {
        self.rooms.iter().enumerate().all(|(room, contents)| {
            let owner = owner(room);
            contents.iter().all(|&x| x == owner)
        })
    }

    #[allow(dead_code)]
    fn debug(&self, energy: usize) {
        let mut grid = VecGrid::new_sized('#', 14, 3 + SLOTS);
        for idx in 0..self.hallway.len() {
            grid[Pos::new(1 + idx, 1)] = self.hallway[idx];
        }
        for room in 0..4 {
            for slot in 0..SLOTS {
                grid[Pos::new(3 + (room * 2), 2 + slot)] = self.rooms[room][slot];
            }
        }
        println!("Energy: {}", energy);
        grid.print();
    }
}

fn organise_amphipods<const SLOTS: usize>(input: &VecGrid<char>) -> usize {
    let initial: State<SLOTS> = State::new(input);
    dijkstra(
        &initial,
        |state| {
            // From this state, consider what next states there could be
            let mut next = Vec::new();
            // First consider all amphipods in hallway
            'HallwayLoop: for (idx, amphipod) in
                state.hallway.iter().enumerate().filter(|(_, &x)| x != '.')
            {
                // Can only move directly to their room from the hallway
                // Check if there is space and if there are any amphipods already there, that they are of the right type
                let room = room(*amphipod);
                if state.rooms[room]
                    .iter()
                    .filter(|&x| *x != '.')
                    .all(|x| x == amphipod)
                {
                    // OK to go into the room
                    // See if we can actually get there (i.e. hallway isn't blocked)
                    // Check from here to the entrance to the room
                    let entrance = room_entrance(*amphipod);
                    let (h1, h2) = if entrance > idx {
                        (idx + 1, entrance)
                    } else {
                        (entrance, idx - 1)
                    };
                    for contains in (h1..=h2).map(|i| state.hallway[i]) {
                        if contains != '.' {
                            // Hallway isn't clear - can't move
                            // Check this isn't a deadlock state
                            let blockers_entrance = room_entrance(contains);
                            if if idx > entrance {
                                blockers_entrance > idx
                            } else {
                                blockers_entrance < idx
                            } {
                                // Two amphipods are stuck in the hallways trying to get to rooms either side of each other
                                // This state can never complete - don't attempt to generate any new ones from it
                                return Vec::new();
                            }
                            // No deadlock, but can't move this time round
                            continue 'HallwayLoop;
                        }
                    }
                    // Hallway is clear - find the last empty slot
                    let (slot, _) = state.rooms[room]
                        .iter()
                        .enumerate()
                        .filter(|(_, &x)| x == '.')
                        .last()
                        .unwrap();
                    // Work out how far we need to move
                    let moves = abs_diff(entrance, idx) + slot + 1;
                    let energy = moves * requires_energy(*amphipod);
                    // Create new state with the amphipod moved
                    let mut state = state.clone();
                    std::mem::swap(&mut state.hallway[idx], &mut state.rooms[room][slot]);
                    next.push((state, energy));
                }
            }

            // Then consider all amphipods in rooms
            for room in 0..state.rooms.len() {
                let owner = owner(room);
                let mut first = SLOTS;
                for (slot, contains) in (0..SLOTS)
                    .map(|slot| (slot, state.rooms[room][slot]))
                    .filter(|(_, x)| *x != '.')
                {
                    if slot < first {
                        // Find the first taken slot
                        first = slot;
                    }
                    if contains != owner {
                        // Found an amphipod in this room that shouldn't be here
                        // Move the first one out (may or may not be this one)
                        let amphipod = state.rooms[room][first];
                        let exit = room_entrance(owner);
                        // Create states for all valid hallway positions that we can move into from the room exit
                        // Will consider two sets of states, moving left and moving right
                        let ranges = [
                            Either::Left((0..exit).rev()),
                            Either::Right(exit + 1..state.hallway.len()),
                        ];
                        for range in ranges {
                            for idx in range.filter(|&i| !is_entrance(i)) {
                                if state.hallway[idx] == '.' {
                                    // Can move here - work out how far we need to move
                                    let moves = first + 1 + abs_diff(idx, exit);
                                    let energy = moves * requires_energy(amphipod);
                                    // Create new state with the amphipod moved
                                    let mut state = state.clone();
                                    std::mem::swap(
                                        &mut state.hallway[idx],
                                        &mut state.rooms[room][first],
                                    );
                                    next.push((state, energy));
                                } else {
                                    // Blocked - stop
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            next
        },
        State::organised,
    )
    .unwrap()
    .1
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    organise_amphipods::<2>(&input.parse().unwrap())
}

static P2_INSERT: [&str; 2] = ["  #D#C#B#A#", "  #D#B#A#C#"];

#[aoc(day23, part2)]
fn part2(input: &str) -> usize {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.splice(3..3, P2_INSERT);
    organise_amphipods::<4>(&lines.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    #############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 12521);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 44169);
    }
}
