use derive_more::Constructor;
use itertools::Itertools;
use lazy_static::lazy_static;
use reformation::Reformation;

#[derive(Debug, Constructor)]
struct Item {
    cost: u32,
    damage: i32,
    armor: i32,
}

lazy_static! {
    // Available items in the shop
    // Additional items with 0 stats for armor/rings are used to represent optional purchases
    static ref WEAPONS: Vec<Item> = {vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0)
    ]};

    static ref ARMOR:  Vec<Item> = {vec![
        Item::new(0, 0, 0),
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5)
    ]};

    static ref RINGS:  Vec<Item> = {vec![
        Item::new(0, 0, 0),
        Item::new(0, 0, 0),
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ]};
}

#[derive(Reformation, Debug, Default, Clone, Copy, Constructor)]
#[reformation(
    r"Hit Points: {hit_points}\nDamage: {damage}\nArmor: {armor}",
    fromstr = true
)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl Character {
    /// Creates a new character with items from the shop
    fn new_with_items(items: &[&&Item]) -> Self {
        Self {
            hit_points: 100,
            damage: items.iter().map(|item| item.damage).sum(),
            armor: items.iter().map(|item| item.armor).sum(),
        }
    }
}

/// Returns the number of rounds a defender could survive being attacked
const fn can_survive_rounds(defender: &Character, attacker: &Character) -> i32 {
    let damage_per_round = if attacker.damage > defender.armor {
        attacker.damage - defender.armor
    } else {
        1 // Always do at least 1 damage
    };
    if defender.hit_points % damage_per_round == 0 {
        defender.hit_points / damage_per_round
    } else {
        (defender.hit_points / damage_per_round) + 1
    }
}

/// Determines if in a fight the player wins (-> true) or the boss wins (-> false)
const fn fight(player: &Character, boss: &Character) -> bool {
    can_survive_rounds(boss, player) <= can_survive_rounds(player, boss)
}

/// Simulate all possible fights from different item purchases
///
/// Return all possible (outcome, cost of items)
fn simulate(boss: Character) -> Vec<(bool, u32)> {
    // Build all combinations of possible item purchases
    // Note: Additional items with 0 stats for armor/rings are included to account for optional purchases
    [
        WEAPONS.iter().combinations(1).collect::<Vec<_>>(),
        ARMOR.iter().combinations(1).collect::<Vec<_>>(),
        RINGS.iter().combinations(2).collect::<Vec<_>>(),
    ]
    .iter()
    .multi_cartesian_product()
    .map(|items| items.iter().flat_map(|&x| x).collect_vec())
    .map(|items| {
        // For each combination of items, see what the outcome and cost would be
        (
            fight(&Character::new_with_items(&items), &boss),
            items.iter().map(|item| item.cost).sum(),
        )
    })
    .collect_vec()
}

#[aoc_generator(day21)]
fn gen(input: &str) -> Character {
    input.parse().unwrap()
}

#[aoc(day21, part1)]
fn part1(input: &Character) -> u32 {
    // Find the lowest cost of items that allows the player to win
    simulate(*input)
        .iter()
        .filter(|(outcome, _)| *outcome)
        .map(|(_, cost)| *cost)
        .min()
        .unwrap()
}

#[aoc(day21, part2)]
fn part2(input: &Character) -> u32 {
    // Find the highest cost of items where the player still looses
    simulate(*input)
        .iter()
        .filter(|(outcome, _)| !*outcome)
        .map(|(_, cost)| *cost)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert!(fight(&Character::new(8, 5, 5), &Character::new(12, 7, 2)));
    }
}
