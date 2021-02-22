use modinverse::modinverse;

#[aoc_generator(day13)]
pub fn gen(input: &str) -> (i64, Vec<i64>) {
    let mut lines = input.lines();
    let earliest_depart_time = lines.next().unwrap().parse().unwrap();    
    let bus_ids = lines.next().unwrap().split(',').map(|id| {
        match id {
            "x" => 0,
            _ => id.parse().unwrap(),
        }        
    }).collect();
    return (earliest_depart_time, bus_ids);
}

#[aoc(day13, part1)]
fn part1(input: &(i64, Vec<i64>)) -> usize {
    let (edt, bus_ids) = input;
    let earliest_depart_time = *edt as f64;
    let mut min_wait = f64::MAX;
    let mut min_wait_id_mult = 0f64;
    for id in bus_ids {
        if *id == 0 {
            continue; // Skip out of service bus Ids
        }
        let bus_id = *id as f64;
        let cycles = earliest_depart_time / bus_id;
        let wait = ((cycles.ceil() - cycles) * bus_id).round();
        if wait < min_wait {
            min_wait = wait;
            min_wait_id_mult = wait * bus_id;
        }
    }
    return min_wait_id_mult.round() as usize;
}


/// Calculates the CRT
/// 
/// https://en.wikipedia.org/wiki/Chinese_remainder_theorem
fn chinese_remainder_theorem(input: &Vec<(i64, i64)>) -> i64 {
    let product = input.iter().map(|(_, m)| m).product::<i64>(); 
    let mut sum = 0; 
    for (remainder, modulus) in input.iter() {
        // Apply extended euclidean algorithm and sum up for all
        let p = product / modulus;
        sum += remainder * modinverse(p, *modulus).unwrap() * p
    }
    return sum % product;
}

/// Generates the offsets and periods needed for CRT from bus ids
/// (we map offsets => remainders and periods => moduli)
fn generate_offsets_periods(bus_ids: &Vec<i64>) -> Vec<(i64, i64)> {
    // The periods are simply the id's of the scheduled buses
    // The offsets are calculated by going through buses, decrementing a counter for each
    // but only emitting it if the bus is scheduled (id != 0)
    let mut offset = bus_ids.len() as i64; // Start the offset with the number of buses
    return bus_ids.iter().map(|id| {
        offset -= 1;
        (offset, *id)
    }).filter(|x| x.1 != 0).collect();
}

#[aoc(day13, part2)]
fn part2(input: &(i64, Vec<i64>)) -> i64 {
    // This part needs Chinese remainder theorem to calculate the first convergence point
    // We model it such that the convergence is equal to offset (mod period) for each bus
    // and then reverse stagger the buses so that they all converge at the same point.
    // We can then subtract back the max offset to get the time of the first bus

    // Use CRT to calculate the convergence point then subtract the 
    // the number of buses - 1 (== max offset)
    // to get the time for the first bus (with 0 offset)
    return chinese_remainder_theorem(&generate_offsets_periods(&input.1)) - (input.1.len() - 1) as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt() {
        assert_eq!(chinese_remainder_theorem(&[(2, 3), (3, 5), (2, 7)].to_vec()), 23);
    }

    #[test]
    fn test_generate_offsets_periods() {
        assert_eq!(generate_offsets_periods(&[17,0,13,19].to_vec()), [(3, 17), (1, 13), (0, 19)])
    }
}