
#[aoc_generator(day13)]
pub fn gen(input: &str) -> (f64, Vec<f64>) {
    let mut lines = input.lines();
    let earliest_depart_time = lines.next().unwrap().parse().unwrap();    
    let bus_ids = lines.next().unwrap().split(',').filter(|&x| x != "x").map(|x| x.parse().unwrap()).collect();
    return (earliest_depart_time, bus_ids);
}

#[aoc(day13, part1)]
fn part1(input: &(f64, Vec<f64>)) -> usize {
    let (earliest_depart_time, bus_ids) = input;
    let mut min_wait = f64::MAX;
    let mut min_wait_id_mult = 0f64;
    for bus_id in bus_ids {
        let cycles = earliest_depart_time / bus_id;
        let wait = ((cycles.ceil() - cycles) * bus_id).round();
        if wait < min_wait {
            min_wait = wait;
            min_wait_id_mult = wait * bus_id;
        }
    }
    return min_wait_id_mult.round() as usize;
}

#[aoc(day13, part2)]
fn part2(input: &(f64, Vec<f64>)) -> usize {
    
    return 0;
}
