use crate::intcode::Intcode;
use std::collections::HashMap;

/// Builds NICs
fn create_nics(input: &str, count: isize) -> Vec<Intcode> {
    let program = input
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<isize>>();
    (0..count)
        .map(|address| {
            // Create a new NIC and configure with an address
            let mut nic = Intcode::new_with(&program, 10240);
            nic.inputs().push_back(address);
            nic
        })
        .collect::<Vec<_>>()
}

/// Runs the supplied NIC program on 50 adapters
fn run(program: &str, stop_on_first_nat: bool) -> isize {
    let mut nics = create_nics(program, 50);
    let mut queues: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
    let mut last_nat_y = -1;
    loop {
        let mut idle = true;
        for (address, nic) in nics.iter_mut().enumerate() {
            // Receive any queued packets as input
            let queue = queues.entry(address).or_default();
            if queue.is_empty() {
                nic.inputs().push_back(-1);
            } else {
                idle = false;
                for packet in queue.drain(0..) {
                    // Input all queued packets
                    let (x, y) = (packet.0, packet.1);
                    nic.inputs().push_back(x);
                    nic.inputs().push_back(y);
                }
            }
            // Run until we need more input
            nic.run();
            // Store any outputs as packets in a queue to be received later
            if !nic.outputs().is_empty() {
                for packet in nic.outputs().drain(0..).collect::<Vec<_>>().chunks(3) {
                    let (target, x, y) = (packet[0], packet[1], packet[2]);
                    let queue = queues.entry(target as usize).or_default();
                    if target == 255 {
                        if stop_on_first_nat {
                            return y; // part 1
                        }
                        // NAT at address 255 only keeps the last packet
                        queue.clear();
                    }
                    queue.push((x, y));
                }
            }
        }
        if idle {
            // Network is idle, send NAT packet to address 0
            let queue = queues.entry(255).or_default();
            let (x, y) = queue.pop().unwrap();
            nics[0].inputs().push_back(x);
            nics[0].inputs().push_back(y);
            if y == last_nat_y {
                return y; // part 2
            }
            last_nat_y = y;
        }
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> isize {
    run(input, true)
}

#[aoc(day23, part2)]
fn part2(input: &str) -> isize {
    run(input, false)
}
