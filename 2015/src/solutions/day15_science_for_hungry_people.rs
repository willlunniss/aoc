use derive_more::Add;
use itertools::Itertools;
use reformation::Reformation;

#[derive(Reformation, Debug, Default, Clone, Copy, Add)]
#[reformation(r"[a-z,A-Z]*: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}", fromstr = true)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    // Calculates the score for an ingredient
    fn score(&self, calorie_goal: Option<i64>) -> i64 {
        if let Some(goal) = calorie_goal {
            if goal != self.calories {
                // Have a calorie goal but haven't met it, 0 score
                return 0;
            }
        }
        let parts = [self.capacity, self.durability, self.flavor, self.texture];
        if parts.iter().any(|v| v <= &0) {
            return 0; // Any zero or negative properties result in 0 score
        }
        parts.iter().product()
    }
}

#[aoc_generator(day15)]
fn gen(input: &str) -> Vec<Ingredient> {
    input.lines().flat_map(str::parse).collect()
}

// Calculates the recipe that gives the highest score taking into account an optional target number of calories
fn best_recipe(input: &[Ingredient], calorie_goal: Option<i64>) -> i64 {
    input
        .iter()
        .combinations_with_replacement(100) // For all combinations of 100 ingredients
        .map(|recipe| {
            recipe
                .iter() // Add all ingredients together
                .fold(Ingredient::default(), |mixing_bowl, &&ingredient| {
                    mixing_bowl + ingredient
                })
                .score(calorie_goal) // And calculate the overall score
        })
        .max() // Return the best score
        .unwrap()
}

#[aoc(day15, part1)]
fn part1(input: &[Ingredient]) -> i64 {
    best_recipe(input, None)
}

#[aoc(day15, part2)]
fn part2(input: &[Ingredient]) -> i64 {
    best_recipe(input, Some(500))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_INPUT: &str = indoc! {"
    Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"};

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&gen(EXAMPLE_INPUT)), 62_842_880);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&gen(EXAMPLE_INPUT)), 57_600_000);
    }
}
