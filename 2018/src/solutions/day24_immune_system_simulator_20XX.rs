use itertools::Itertools;
use regex::Regex;
use std::borrow::ToOwned;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;
use strum_macros::EnumString;

type GroupId = usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, EnumString)]
enum Army {
    #[strum(serialize = "Immune System:")]
    ImmuneSystem,
    #[strum(serialize = "Infection:")]
    Infection,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Group {
    id: GroupId,
    army: Army,
    units: usize,
    hit_points: usize,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    attack_type: String,
    attack_damage: usize,
    initiative: usize,
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.effective_power().cmp(&other.effective_power());
        if cmp == Ordering::Equal {
            self.initiative.cmp(&other.initiative)
        } else {
            cmp
        }
    }
}

impl Group {
    fn new(id: GroupId, army: Army, s: &str) -> Result<Self, Box<dyn Error>> {
        // Parse Group description using Regex. Broke it into two to to make it slightly more readable
        lazy_static! {
            static ref GROUP_RE: Regex = Regex::new(r"^(?P<units>\d+)\D+(?P<hit_points>\d+)[a-z\s]*(?P<attributes>\([a-z\s,;]+\))?\D+\s(?P<attack_damage>\d+)\s(?P<attack_type>\S+)\D+(?P<initiative>\d+)$").unwrap();
            static ref ATTRIBUTES_RE: Regex = Regex::new(r"((?P<attribute>(weak|immune)) to (?P<values>((\w+)(,\s\w+)*))+)").unwrap();
        }
        let captured = GROUP_RE.captures(s).unwrap();
        let units = captured.name("units").unwrap().as_str().parse().unwrap();
        let hit_points = captured
            .name("hit_points")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let initiative = captured
            .name("initiative")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let attack_type = captured.name("attack_type").unwrap().as_str().to_owned();
        let attack_damage = captured
            .name("attack_damage")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let mut weaknesses = HashSet::new();
        let mut immunities = HashSet::new();
        // Expect to get 0, 1 or 2 attribute types in any order, each of which will have 1 or more values
        if let Some(attributes) = captured.name("attributes") {
            for captured_attrs in ATTRIBUTES_RE.captures_iter(attributes.as_str()) {
                let attribute = captured_attrs.name("attribute").unwrap().as_str();
                let values = captured_attrs
                    .name("values")
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(ToOwned::to_owned);
                match attribute {
                    "weak" => weaknesses.extend(values),
                    "immune" => immunities.extend(values),
                    _ => return Err(format!("Unexpected attribute {}", attribute).into()),
                }
            }
        }
        Ok(Self {
            id,
            army,
            units,
            hit_points,
            weaknesses,
            immunities,
            attack_type,
            attack_damage,
            initiative,
        })
    }

    /// Returns the effective power of this group
    const fn effective_power(&self) -> usize {
        self.units * self.attack_damage
    }

    /// Returns the damage that would be dealt if this group attacked the `defending` group
    fn damage_dealt(&self, defending: &Self) -> usize {
        if defending.immunities.contains(&self.attack_type) {
            0 // Immune to the attack, zero damage
        } else if defending.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2 // Weak to the attack, double damage
        } else {
            self.effective_power() // Normal damage
        }
    }

    /// Attacks the group with `damage` and returns the number of units killed
    fn attack(&mut self, damage: usize) -> usize {
        // Work out how many whole units will be killed
        let units = self.units;
        let units_killed = damage / self.hit_points;
        if units_killed >= units {
            // Group has been wiped out
            self.units = 0;
            units
        } else {
            self.units -= units_killed;
            units_killed
        }
    }
}

fn gen(input: &str) -> HashMap<GroupId, Group> {
    let mut lines = input.lines();
    let mut groups = HashMap::new();
    let mut id = 0;
    while let Some(line) = lines.next() {
        // Read in the input of form
        // Army:
        // group 1
        // group 2
        // ..
        // <blank line>
        let army = Army::from_str(line).unwrap();
        #[allow(clippy::while_let_on_iterator)]
        while let Some(line) = lines.next() {
            if line.is_empty() {
                // Skip to next army
                break;
            }
            // Create a new group
            groups.insert(id, Group::new(id, army, line).unwrap());
            id += 1;
        }
    }
    groups
}

/// Simulates the armies fighting until either one side is eliminated or there is a stalemate
///
/// Returns Some(winning army, remaining units) or None for a stalemate
fn fight(groups: HashMap<GroupId, Group>) -> Option<(Army, usize)> {
    let mut groups = groups;
    for _fight in 0.. {
        // Target selection phase:
        // Create a set of available targets based on remaining groups
        let mut available_targets: HashSet<GroupId> = groups.iter().map(|(id, _)| *id).collect();
        // Map to store who will attack who
        let mut targets: HashMap<GroupId, GroupId> = HashMap::new();
        // Consider each attacker based decreasing effective power (tied using initiative)
        for attacker in groups.values().sorted().rev() {
            // Find a target
            let target = available_targets
                .iter()
                .filter_map(|id| {
                    // Only consider groups that belong to the other army that we could do some damage too
                    let target = groups.get(id).unwrap();
                    if target.army == attacker.army {
                        None
                    } else {
                        let damage = attacker.damage_dealt(target);
                        if damage > 0 {
                            Some((target, damage))
                        } else {
                            None
                        }
                    }
                })
                .max_by(|(target_x, damage_x), (target_y, damage_y)| {
                    // Work out which one we would do the most damage against
                    let cmp = damage_x.cmp(damage_y);
                    if cmp == Ordering::Equal {
                        // If a tie, pick the highest effective power, then initiative
                        target_x.cmp(target_y)
                    } else {
                        cmp
                    }
                });
            if let Some((target, _)) = target {
                // Found a suitable target to fight against, record it for later and remove from available
                targets.insert(attacker.id, target.id);
                available_targets.remove(&target.id);
            }
        }
        if targets.is_empty() {
            // Only one side left - combat ends
            break;
        }
        // Attacking phase:
        let mut units_killed = 0;
        // Now we have selected targets we can start the fight
        // Attack in decreasing order of initiative
        let mut attackers = targets
            .keys()
            .map(|id| (*id, groups.get(id).unwrap().initiative))
            .collect::<Vec<_>>();
        attackers.sort_by_key(|x| x.1);
        for (attacker_id, _) in attackers.iter().rev() {
            // Get the attacker, if they are still alive
            if let Some(attacker) = groups.get(attacker_id) {
                // Work out how much damage we will do
                // Have to re-calculate during attacking phase as we may have been attacked and lost units since target selection
                let target_id = targets.get(attacker_id).unwrap();
                let target = groups.get(target_id).unwrap();
                let damage = attacker.damage_dealt(target);
                // Then attack
                let target = groups.get_mut(target_id).unwrap();
                let killed = target.attack(damage);
                units_killed += killed;
                if target.units == 0 {
                    // Target has been wiped out
                    groups.remove(target_id);
                }
            } // else attacker has since been wiped out
        }
        if units_killed == 0 {
            // No one was killed this fight - stalemate
            return None;
        }
    }
    // One side won, return the Army and number of remaining units
    Some((
        groups.values().next().unwrap().army,
        groups.iter().map(|(_, group)| group.units).sum(),
    ))
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
    // Fight and return the number of units left for the winning side
    let (_, units) = fight(gen(input)).unwrap();
    units
}

#[aoc(day24, part2)]
fn part2(input: &str) -> usize {
    let initial = gen(input);
    for boost in 0.. {
        // Keep boosting the immune system's attack until they win
        let mut groups = initial.clone();
        for (_, group) in groups
            .iter_mut()
            .filter(|(_, group)| group.army == Army::ImmuneSystem)
        {
            group.attack_damage += boost;
        }
        // Fight
        if let Some((winner, units)) = fight(groups) {
            if winner == Army::ImmuneSystem {
                return units;
            }
        } // else stalemate
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parse_sample() {
        let group = Group::new(1, Army::Infection, "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10").unwrap();
        println!("{:?}", group);
        assert_eq!(group.units, 18);
        assert_eq!(group.hit_points, 729);
        assert_eq!(group.weaknesses.contains("fire"), true);
        assert_eq!(group.immunities.contains("cold"), true);
        assert_eq!(group.immunities.contains("slashing"), true);
        assert_eq!(group.attack_type, "radiation".to_owned());
        assert_eq!(group.attack_damage, 8);
        assert_eq!(group.initiative, 10);
    }

    #[test]
    fn test_group_parse_immune_weak() {
        let group = Group::new(1, Army::Infection, "18 units each with 729 hit points (immune to cold, slashing; weak to fire) with an attack that does 8 radiation damage at initiative 10").unwrap();
        assert_eq!(group.weaknesses.len(), 1);
        assert_eq!(group.immunities.len(), 2);
    }

    #[test]
    fn test_group_parse_weak_immune() {
        let group = Group::new(1, Army::Infection, "18 units each with 729 hit points (weak to fire; immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10").unwrap();
        assert_eq!(group.weaknesses.len(), 1);
        assert_eq!(group.immunities.len(), 2);
    }

    #[test]
    fn test_group_parse_weak() {
        let group = Group::new(1, Army::Infection, "18 units each with 729 hit points (weak to fire) with an attack that does 8 radiation damage at initiative 10").unwrap();
        assert_eq!(group.weaknesses.len(), 1);
        assert_eq!(group.immunities.len(), 0);
    }

    #[test]
    fn test_group_parse_immune() {
        let group = Group::new(1, Army::Infection, "18 units each with 729 hit points (immune to cold, slashing) with an attack that does 8 radiation damage at initiative 10").unwrap();
        assert_eq!(group.weaknesses.len(), 0);
        assert_eq!(group.immunities.len(), 2);
    }
}
