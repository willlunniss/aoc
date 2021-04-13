use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn gen(input: &str) -> (HashMap<&str, usize>, HashMap<&str, Vec<HashSet<&str>>>) {
    let mut allergen_to_ingredients: HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();
    let mut all_ingredients_count: HashMap<&str, usize> = HashMap::new();
    for product in input.lines() {
        // abc def ghi (contains xyz, uvw)
        let (ingredients_str, allergens) =
            product.splitn(2, " (contains ").collect_tuple().unwrap();
        let ingredients = ingredients_str.split(' ').collect::<HashSet<&str>>();
        for ingredient in &ingredients {
            let count = all_ingredients_count.entry(ingredient).or_default();
            *count += 1;
        }
        for allergen in allergens[..allergens.len() - 1].split(", ") {
            allergen_to_ingredients
                .entry(allergen)
                .or_default()
                .push(ingredients.clone());
        }
    }
    (all_ingredients_count, allergen_to_ingredients)
}

/// Analyses the ingredients
///
/// returns a tuple with
/// * HashMap<ingredient, count>: The number of occurrences for each inert ingredient
/// * HashMap<allergen, ingredient>: The ingredient that contains each allergen
fn analyse_ingredients(input: &str) -> (HashMap<&str, usize>, HashMap<&str, &str>) {
    let (mut inert_ingredients, mut allergen_to_ingredients) = gen(input);

    let mut dangerous_ingredients: HashMap<&str, &str> = HashMap::new();
    loop {
        let mut ingredient = None;
        let mut to_remove = None;
        for (allergen, potential_ingredients) in &allergen_to_ingredients {
            let minimal: HashSet<_> = potential_ingredients[0]
                .iter()
                .filter(|k| potential_ingredients.iter().skip(1).all(|s| s.contains(*k)))
                .collect();
            if minimal.len() == 1 {
                ingredient = Some(*minimal.iter().next().unwrap().clone());
                to_remove = Some(allergen.clone());
                // Add to dangerous map
                dangerous_ingredients.insert(allergen, ingredient.unwrap());
            }
        }
        if ingredient.is_none() {
            // Can't reduce any more
            break;
        }
        // Remove this allergen
        allergen_to_ingredients.remove(to_remove.unwrap());
        // And remote the ingredient that contained it from the others so we can do another pass
        for ingredient_lists in allergen_to_ingredients.values_mut() {
            for ingredients in ingredient_lists.iter_mut() {
                ingredients.remove(ingredient.unwrap());
            }
        }
        // And finally remove it from our non-allergens
        inert_ingredients.remove(ingredient.unwrap());
    }

    (inert_ingredients, dangerous_ingredients)
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let (inert_ingredients, _) = analyse_ingredients(input);
    // Return the sum of the number of occurrences of each inert ingredient
    return inert_ingredients.values().sum();
}

#[aoc(day21, part2)]
fn part2(input: &str) -> String {
    let (_, dangerous_ingredients) = analyse_ingredients(input);

    // Build the canonical dangerous ingredient list by listing the ingredients
    // sorted by allergen
    let mut dangerous = Vec::new();
    for key in dangerous_ingredients.keys().sorted() {
        dangerous.push(*dangerous_ingredients.get(key).unwrap());
    }

    dangerous.join(",")
}
