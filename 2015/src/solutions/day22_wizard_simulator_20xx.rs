use derive_more::Constructor;
use reformation::Reformation;
use std::collections::{HashMap, VecDeque};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Reformation, Debug, Default, Constructor, Clone, Copy)]
#[reformation(r"Hit Points: {hit_points}\nDamage: {damage}", fromstr = true)]
struct Character {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: u32,
    spent_mana: u32,
}

type Turns = u32;

impl Character {
    /// Creates a new player
    const fn new_player(hit_points: i32, mana: u32) -> Self {
        Self {
            hit_points,
            damage: 0,
            armor: 0,
            mana,
            spent_mana: 0,
        }
    }

    /// Defend against an attack
    fn defend(&mut self, attacker: &Self) {
        self.hit_points -= if self.armor > attacker.damage {
            1 // Will always do at least 1 damage
        } else {
            attacker.damage - self.armor
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    /// Returns the number of turns the spell lasts for
    fn turns(self) -> Turns {
        match self {
            Self::Shield | Self::Poison => 6,
            Self::Recharge => 5,
            _ => {
                panic!("{self:?} is not an effect");
            }
        }
    }

    /// Returns the cost of casting the spell
    const fn cost(self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    /// Casts a spell, returns true if complete or false if it
    /// is an effect which applies over multiple turns
    fn cast(self, player: &mut Character, boss: &mut Character) -> bool {
        player.mana -= self.cost();
        player.spent_mana += self.cost();
        match self {
            Self::MagicMissile => {
                // Deal 4 damage
                boss.hit_points -= 4;
                return true;
            }
            Self::Drain => {
                // Deal 2 damage to boss and heal player 2
                boss.hit_points -= 2;
                player.hit_points += 2;
                return true;
            }
            Self::Shield => {
                // Gives 7 armor
                player.armor += 7;
            }
            _ => {}
        }
        false
    }

    /// Applies an active effect
    fn apply(self, player: &mut Character, boss: &mut Character) {
        match self {
            Self::Poison => {
                // Deals 3 damage to the boss
                boss.hit_points -= 3;
            }
            Self::Recharge => {
                // Recharge mana
                player.mana += 101;
            }
            _ => {}
        }
    }

    /// Ends and active effect
    fn end(self, player: &mut Character) {
        if self == Self::Shield {
            // Remove 7 armor
            player.armor -= 7;
        }
    }
}

#[derive(Debug, Clone)]
struct GameState {
    player: Character,
    boss: Character,
    effects: HashMap<Spell, Turns>,
}

impl GameState {
    /// Applies all active effects, removing them if they are used up
    fn apply_effects(&mut self) {
        let Self {
            player,
            boss,
            effects,
        } = self;
        for (spell, turns) in effects.iter_mut() {
            // Apply the effect
            spell.apply(player, boss);
            // Decrement remaining turns
            *turns -= 1;
            if *turns == 0 {
                // and end if no longer active
                spell.end(player);
            }
        }
        // Keep all that are still active
        effects.retain(|_, turns| *turns > 0);
    }

    /// Fight for one round, which includes one turn each
    ///
    /// Returns:
    /// None - no winner
    /// Some(true) - player wins
    /// Some(false) - boss wins
    fn fight(&mut self, spell: Spell, hard_mode: bool) -> Option<bool> {
        // Start of player turn
        if hard_mode {
            // Hard mode - player looses 1 HP at the start of their turn
            self.player.hit_points -= 1;
            if self.player.hit_points <= 0 {
                return Some(false); // And player is killed
            }
        }
        self.apply_effects();
        if self.boss.hit_points <= 0 {
            return Some(true); // And boss is killed
        }
        // Player casts a spell
        if !spell.cast(&mut self.player, &mut self.boss) {
            // Spell is an effect, will apply over multiple turns
            self.effects.insert(spell, spell.turns());
        }
        if self.boss.hit_points <= 0 {
            return Some(true); // And kills boss
        }

        // Start of boss turn
        // Apply all active effects
        self.apply_effects();
        if self.boss.hit_points <= 0 {
            return Some(true); // And boss is killed
        }
        // Boss attacks
        self.player.defend(&self.boss);
        if self.player.hit_points <= 0 {
            return Some(false); // And kills player
        }

        None // No winner
    }
}

/// Returns the cost for the play that results in a win with the minimal mana spent
fn find_optimal_play(player: Character, boss: Character, hard_mode: bool) -> u32 {
    let mut queue = VecDeque::new();
    let state = GameState {
        player,
        boss,
        effects: HashMap::new(),
    };
    queue.push_back(state);
    let mut winning_spend = u32::MAX;

    // Keep going until no more viable plays
    while let Some(state) = queue.pop_front() {
        // Consider all possible spells that could be cast from this state
        for spell in Spell::iter() {
            if spell.cost() > state.player.mana {
                // Cannot afford to cast this spell, skip
                continue;
            }
            if let Some(turns) = state.effects.get(&spell) {
                if *turns > 1 {
                    // Effect still active and won't run about before
                    // the player casts so cannot cast yet, skip
                    continue;
                }
            }
            // Fight using this spell to see what happens
            let mut state = state.clone();
            let outcome = state.fight(spell, hard_mode);
            if state.player.spent_mana < winning_spend {
                if outcome == Some(true) {
                    // Player won, update best spend
                    winning_spend = state.player.spent_mana;
                } else if outcome.is_none() {
                    // No one has won yet, add to the queue to fight again
                    queue.push_front(state);
                }
            } // Spent more than the best, not worth considering
        }
    }

    winning_spend
}

#[aoc_generator(day22)]
fn gen(input: &str) -> Character {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
fn part1(input: &Character) -> u32 {
    find_optimal_play(Character::new_player(50, 500), *input, false)
}

#[aoc(day22, part2)]
fn part2(input: &Character) -> u32 {
    find_optimal_play(Character::new_player(50, 500), *input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(
            find_optimal_play(
                Character::new_player(10, 250),
                Character::new(13, 8, 0, 0, 0),
                false
            ),
            226
        );
        assert_eq!(
            find_optimal_play(
                Character::new_player(10, 250),
                Character::new(14, 8, 0, 0, 0),
                false
            ),
            641
        );
    }
}
