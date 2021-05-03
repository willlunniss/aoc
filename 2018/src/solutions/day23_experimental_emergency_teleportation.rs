use std::collections::HashMap;
use z3::{ast, ast::Int, Config, Context, Optimize};

type Pos3 = [i64; 3];

#[aoc_generator(day23)]
fn gen(input: &str) -> HashMap<Pos3, u64> {
    input
        .lines()
        .map(|line| {
            let parts = line
                .split(&['<', ' ', ',', '>', '='][..])
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>();
            (
                [
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                    parts[3].parse().unwrap(),
                ],
                parts[5].parse().unwrap(),
            )
        })
        .collect()
}

/// Calculates the Manhattan distance for two positions
fn manhattan_distance(a: &Pos3, b: &Pos3) -> u64 {
    (0..3).map(|index| (a[index] - b[index]).abs() as u64).sum()
}

#[aoc(day23, part1)]
fn part1(input: &HashMap<Pos3, u64>) -> usize {
    // Find the strongest nanobot as the one with the max range
    let (strongest, range) = input.iter().max_by_key(|(_, range)| *range).unwrap();
    // Count all the nanobots that are in range of the strongest
    input
        .keys()
        .filter(|pos| manhattan_distance(strongest, pos) <= *range)
        .count()
}

#[aoc(day23, part2)]
fn part2(input: &HashMap<Pos3, u64>) -> Option<i64> {
    // Solve part 2 using Z3
    // Largely based on <https://cprimozic.net/blog/a-rusty-aoc/#day-23-using-the-z3-smt-solver>
    // with some tweaks and updates for the latest Z3 crate API
    // TODO: This is pretty slow so want to look at alternatives (non Z3 ways)

    /// Computes the absolute value of `value`
    fn abs<'ctx, 'a>(ctx: &'ctx Context, value: &'a ast::Int<'ctx>) -> ast::Int<'ctx> {
        // If the value is less than 0, return value * -1, else return value
        value
            .lt(&Int::from_i64(ctx, 0))
            .ite(&Int::mul(ctx, &[&Int::from_i64(ctx, -1), value]), value)
    }

    // Setup Z3
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // Create 3 variable constraints to represent the position that we are trying to find
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    // Create a variable to count how many bots are in range
    let mut in_range = Int::from_i64(&ctx, 0);
    for (bot, range) in input {
        // Calculate the manhattan distance between the bot and the target position
        let dist = bot
            .iter() // For all axes
            .map(|&bot_| Int::from_i64(&ctx, bot_)) // Create a variable for the bot's position in this axis
            .zip([&x, &y, &z].iter())
            .map(|(bot_, pos_)| Int::sub(&ctx, &[&bot_, pos_])) // Calculate the distance between the bot and the target position
            .map(|dist_| abs(&ctx, &dist_)) // Convert to absolute
            .collect::<Vec<_>>();
        // Sum up the distances in each axis
        let distance_to_bot = Int::add(&ctx, &dist.iter().collect::<Vec<_>>());

        // Check to see if we are in range of the bot by checking that the distance is less than range + 1 (no lte)
        let bot_radius_plus_1 = Int::from_u64(&ctx, *range + 1);
        let is_in_range_of_bot = distance_to_bot.lt(&bot_radius_plus_1);

        // Update the number of bots in range
        // If in range of the bot then add 1, else add 0
        in_range = Int::add(
            &ctx,
            &[
                &in_range,
                &is_in_range_of_bot.ite(&Int::from_i64(&ctx, 1), &Int::from_i64(&ctx, 0)),
            ],
        );
    }

    // Configure the optimizer
    let optimizer = Optimize::new(&ctx);
    // Try to maximise the number of bots that are in range
    optimizer.maximize(&in_range);

    // Try to minimise the manhattan distance to the origin (if multiple points have the same number of bots in range)
    let distance_to_origin = Int::add(&ctx, &[&abs(&ctx, &x), &abs(&ctx, &y), &abs(&ctx, &z)]);
    optimizer.minimize(&distance_to_origin);

    // Evaluate the model
    optimizer.check(&[]);
    let model = optimizer.get_model().unwrap();
    let result = model.eval(&distance_to_origin).unwrap().as_i64();
    result
}
