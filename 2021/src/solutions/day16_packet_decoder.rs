use itertools::Itertools;

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    packets: Vec<Packet>,
}

impl Packet {
    const fn new(version: usize, type_id: usize) -> Self {
        Self {
            version,
            type_id,
            value: 0,
            packets: Vec::new(),
        }
    }
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<char> {
    input
        .chars()
        .flat_map(|c| {
            let value = u8::from_str_radix(&c.to_string(), 16).unwrap();
            let bin_str = format!("{:#06b}", value);
            bin_str.chars().skip(2).collect::<Vec<char>>()
        })
        .collect()
}

fn decode_value(binary: String) -> usize {
    usize::from_str_radix(&binary, 2).unwrap()
}

fn parse_packet<'a>(iter: &mut impl Iterator<Item = &'a char>) -> (Packet, usize) {
    // Expect 3 bits for the version and 3 for the type_id
    let version = decode_value(iter.take(3).collect::<String>());
    let type_id = decode_value(iter.take(3).collect::<String>());
    let mut bits_read = 6;
    let mut packet = Packet::new(version, type_id);
    if type_id == 4 {
        // Literal value packet
        let mut data: Vec<char> = Vec::new();
        loop {
            // Reach each group until we find one that starts with 0
            let final_group = *iter.next().unwrap() == '0';
            // Each group contains 4 values
            data.extend(iter.take(4));
            bits_read += 5;
            if final_group {
                // Read the complete value
                packet.value = decode_value(data.iter().collect::<String>());
                break;
            }
        }
    } else {
        // Operator packet with sub packets
        match *iter.next().unwrap() {
            '0' => {
                // Next 15 bits give the total length of sub packets
                let length = decode_value(iter.take(15).collect::<String>());
                bits_read += 1 + 15;
                let target = bits_read + length;
                while bits_read < target {
                    // parse sub packets until we have read the length bits
                    let (sub_packet, sub_bits_read) = parse_packet(iter);
                    packet.packets.push(sub_packet);
                    bits_read += sub_bits_read;
                }
            }
            '1' => {
                // Next 11 bits give the total number of immediately contained sub packets
                let count = decode_value(iter.take(11).collect::<String>());
                bits_read += 1 + 11;
                for _ in 0..count {
                    // parse count number of sub packets
                    let (sub_packet, sub_bits_read) = parse_packet(iter);
                    packet.packets.push(sub_packet);
                    bits_read += sub_bits_read;
                }
            }
            _ => {
                panic!();
            }
        }
    }

    (packet, bits_read)
}

fn sum_versions(packet: &Packet) -> usize {
    packet.version + packet.packets.iter().map(sum_versions).sum::<usize>()
}

fn eval_packet(packet: &Packet) -> usize {
    match packet.type_id {
        0 => packet.packets.iter().map(eval_packet).sum(),
        1 => packet.packets.iter().map(eval_packet).product(),
        2 => packet.packets.iter().map(eval_packet).min().unwrap(),
        3 => packet.packets.iter().map(eval_packet).max().unwrap(),
        4 => packet.value,
        5 => {
            let (a, b) = packet.packets.iter().map(eval_packet).next_tuple().unwrap();
            (a > b) as usize
        }
        6 => {
            let (a, b) = packet.packets.iter().map(eval_packet).next_tuple().unwrap();
            (a < b) as usize
        }
        7 => {
            let (a, b) = packet.packets.iter().map(eval_packet).next_tuple().unwrap();
            (a == b) as usize
        }
        _ => {
            panic!();
        }
    }
}

#[aoc(day16, part1)]
fn part1(input: &Vec<char>) -> usize {
    let (packet, _) = parse_packet(&mut input.iter());
    sum_versions(&packet)
}

#[aoc(day16, part2)]
fn part2(input: &Vec<char>) -> usize {
    let (packet, _) = parse_packet(&mut input.iter());
    eval_packet(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1(&gen("D2FE28")), 6);
        assert_eq!(part1(&gen("38006F45291200")), 9);
        assert_eq!(part1(&gen("EE00D40C823060")), 14);
        assert_eq!(part1(&gen("8A004A801A8002F478")), 16);
        assert_eq!(part1(&gen("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&gen("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&gen("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2(&gen("C200B40A82")), 3);
        assert_eq!(part2(&gen("04005AC33890")), 54);
        assert_eq!(part2(&gen("880086C3E88112")), 7);
        assert_eq!(part2(&gen("CE00C43D881120")), 9);
        assert_eq!(part2(&gen("D8005AC2A8F0")), 1);
        assert_eq!(part2(&gen("F600BC2D8F")), 0);
        assert_eq!(part2(&gen("9C005AC2F8F0")), 0);
        assert_eq!(part2(&gen("9C0141080250320F1802104A08")), 1);
    }
}
