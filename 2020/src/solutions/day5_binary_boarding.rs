fn decode(boarding_pass: &str) -> (usize, usize, usize) {    
    let mut row_min = 0;
    let mut row_max = 127;
    let mut seat_min = 0;
    let mut seat_max = 7;
    for c in boarding_pass.chars() {
        match c {
            'F' => row_max = row_max - 1 - (row_max - row_min) / 2, // Lower half
            'B' => row_min = row_min + 1 + (row_max - row_min) / 2, // Upper half
            'L' => seat_max = seat_max - 1 - (seat_max - seat_min) / 2, // Left side
            'R' => seat_min = seat_min + 1 + (seat_max - seat_min) / 2, // Right side
            _ => {}
        }
    }
    let row = row_min;
    let seat = seat_min;
    let seat_id = (row * 8) + seat;
    return (seat_id, seat, row);
}


#[aoc_generator(day5)]
pub fn gen(input: &str) -> Vec<usize> {
    let mut seat_ids : Vec<usize> = Vec::new();    
    for boarding_pass in input.lines() {
        let (seat_id, _, _) = decode(boarding_pass);
        seat_ids.push(seat_id);
    }
    seat_ids.sort();
    return seat_ids;
}

#[aoc(day5, part1)]
fn part1(seat_ids: &Vec<usize>) -> Option<usize> {
    seat_ids.iter().max().copied()
}

#[aoc(day5, part2)]
fn part2(seat_ids: &Vec<usize>) -> usize {
    let mut last_id = 0;
    for id in seat_ids.iter() {
        if *id == last_id + 2 {
            // We have two IDs either side of a missing one
            return id - 1;
        }
        last_id = *id;
    }
    return 0;
}
