use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet, VecDeque};
use utils::grid::{Pos, VecGrid};

#[derive(Debug, Eq, PartialEq)]
enum Creature {
    Goblin,
    Elf,
}

impl Creature {
    /// Returns the enemy of this Creature
    const fn enemy(&self) -> Self {
        match self {
            Self::Goblin => Self::Elf,
            Self::Elf => Self::Goblin,
        }
    }

    /// Returns the char that is used to represent this Creature on a map
    const fn char(&self) -> char {
        match self {
            Self::Goblin => 'G',
            Self::Elf => 'E',
        }
    }
}

#[derive(Debug)]
struct Unit {
    creature: Creature,
    hp: usize,
}

impl Unit {
    /// Returns a new unit
    const fn new(creature: Creature) -> Self {
        Self { creature, hp: 200 }
    }
}

/// Chooses where to move to
///
/// Takes a set of target positions and chooses the nearest reachable position to `nearest_from`
/// If there is a tie, picks the first in reading order
fn chose_move(map: &VecGrid<char>, nearest_from: Pos, targets: &HashSet<Pos>) -> Option<Pos> {
    let mut targets = targets.clone();
    let mut visited = HashSet::new();
    let mut nearest_reachable = Vec::new();
    let mut queue = VecDeque::new();
    visited.insert(nearest_from);
    queue.push_back((nearest_from, 0));
    let mut nearest_distance = usize::MAX;
    // Perform a BFS from start to find the nearest reachable positions from our targets
    while !queue.is_empty() {
        let (pos, distance) = queue.pop_front().unwrap();
        if targets.contains(&pos) && distance <= nearest_distance {
            // Got to a place we want to without increasing distance
            nearest_reachable.push(pos);
            // Remove it from targets
            targets.remove(&pos);
            if targets.is_empty() {
                // Found all targets, stop search
                break;
            }
            nearest_distance = distance;
        }
        if distance >= nearest_distance {
            // Haven't found them all yet but no point searching further from here as we won't find anything nearer
            continue;
        }
        // Add open unvisited neighbours to queue
        for (_, neigh, cell) in map.neighbours_ex(pos) {
            if Some('.') == cell && visited.insert(neigh) {
                queue.push_back((neigh, distance + 1));
            }
        }
    }
    if nearest_reachable.is_empty() {
        // Can't reach any targets
        return None;
    }
    // We only have positions that are nearest to us
    // Get the one with the minimal position (to get the first in reading order)
    nearest_reachable.iter().min_by_key(|pos| *pos).copied()
}

/// Simulates a fight until only one side is left
///
/// Returns the outcome if the fight ends with just one side left, else None
///
/// `elf_attack`: Controls how much damage elves deal with each attack
/// `abort_on_elf_death`: If set causes the fight to be aborted if an elf dies
fn fight(input: &VecGrid<char>, elf_attack: usize, abort_on_elf_death: bool) -> Option<usize> {
    let mut map = input.clone();
    // Find all the elves and goblins (sorted by position)
    let mut units = map
        .into_iter()
        .filter_map(|(pos, value)| match value {
            'E' => Some((pos, Unit::new(Creature::Elf))),
            'G' => Some((pos, Unit::new(Creature::Goblin))),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    for round in 0.. {
        // For each round process each unit based on it's current position
        for mut unit_pos in units.keys().copied().collect::<Vec<_>>() {
            // Get the unit (may be none if it has been killed in this round)
            if let Some(unit) = units.remove(&unit_pos) {
                // Find remaining targets
                let enemy = unit.creature.enemy();
                let targets = units
                    .iter()
                    .filter(|(_, target)| target.creature == enemy)
                    .collect::<BTreeMap<_, _>>();
                if targets.is_empty() {
                    // No targets left, combat ends - return outcome
                    let remaining_unit_hp =
                        (units.iter().map(|(_, unit)| unit.hp).sum::<usize>() + unit.hp) as usize;
                    return Some(round * remaining_unit_hp);
                }
                // See if we are already in range of a target
                let mut attackable = unit_pos
                    .neighbours()
                    .filter(|pos| targets.contains_key(pos))
                    .collect::<Vec<_>>();
                if attackable.is_empty() {
                    // Not in range of anything yet, will need to move
                    // Find all open squares that are in range of each target
                    let in_range = targets
                        .keys()
                        .flat_map(|&target| target.neighbours())
                        .filter(|&neighbour| map.get(neighbour) == Some('.'))
                        .collect::<HashSet<Pos>>();
                    if !in_range.is_empty() {
                        // We have some targets with spaces next to them, see if we can actually move towards any of them
                        if let Some(chosen) = chose_move(&map, unit_pos, &in_range) {
                            // Have a valid target to move towards, move towards it
                            // Get all the places we can move to with one step
                            let open_neighbours = map
                                .neighbours_ex(unit_pos)
                                .iter()
                                .filter_map(|(_, neigh, value)| {
                                    if *value == Some('.') {
                                        Some(*neigh)
                                    } else {
                                        None
                                    }
                                })
                                .collect::<HashSet<Pos>>();
                            // Pick the one that gets us closest to the chosen target
                            let new_pos = chose_move(&map, chosen, &open_neighbours).unwrap();
                            // Update map and update unit pos
                            map.insert(unit_pos, '.');
                            map.insert(new_pos, unit.creature.char());
                            unit_pos = new_pos;

                            // See if we can now attack having moved
                            attackable.extend(
                                unit_pos
                                    .neighbours()
                                    .filter(|neigh| targets.contains_key(neigh)),
                            );
                        }
                    }
                }
                // Can now attack - select target with lowest hit points, if tie position in reading order
                if let Some(target) = attackable
                    .iter()
                    .map(|pos| (pos, units.get(pos).unwrap()))
                    .min_by(|(pos1, unit1), (pos2, unit2)| {
                        let hp_cmp = unit1.hp.cmp(&unit2.hp);
                        if hp_cmp == Ordering::Equal {
                            pos1.cmp(pos2)
                        } else {
                            hp_cmp
                        }
                    })
                    .map(|(pos, _)| pos)
                {
                    // Can attack target from where we are
                    // Get mutable ref to target unit
                    let mut target_unit = units.get_mut(target).unwrap();
                    // Attack the target based on this units attack strength
                    let attack = if unit.creature == Creature::Elf {
                        elf_attack
                    } else {
                        3 // Goblins always have an attack of 3
                    };
                    if target_unit.hp <= attack {
                        // Target will be killed, remove it and update map
                        if target_unit.creature == Creature::Elf && abort_on_elf_death {
                            // Abandon the fight if an elf has been killed and we want to find a fight where no elves die
                            return None;
                        }
                        units.remove(target);
                        map.insert(*target, '.');
                    } else {
                        // Target hurt but not killed, decrement it's hp
                        target_unit.hp -= attack;
                    }
                } // Else there weren't actually any in range to attack

                // Add current unit back as an active units (may now be at a new position)
                units.insert(unit_pos, unit);
            }
        }
    }
    None
}

#[aoc_generator(day15)]
fn gen(input: &str) -> VecGrid<char> {
    VecGrid::from(input.lines().map(|line| line.chars().collect()).collect())
}

#[aoc(day15, part1)]
fn part1(input: &VecGrid<char>) -> Option<usize> {
    // Fight normally and return the outcome
    fight(input, 3, false)
}

#[aoc(day15, part2)]
fn part2(input: &VecGrid<char>) -> Option<usize> {
    // Try with increasing elf attack until we get a fight that doesn't result in any elves dying
    (4..).find_map(|elf_attack| fight(input, elf_attack, true))
}
