use itertools::Itertools;

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    type_id: usize,
    payload: Payload,
}

impl Packet {
    const fn new_value(version: usize, type_id: usize, value: usize) -> Self {
        Self {
            version,
            type_id,
            payload: Payload::Value(value),
        }
    }

    const fn new_operator(version: usize, type_id: usize, packets: Vec<Self>) -> Self {
        Self {
            version,
            type_id,
            payload: Payload::Operator(packets),
        }
    }

    fn sum_versions(&self) -> usize {
        self.version
            + match &self.payload {
                Payload::Value(_) => 0,
                Payload::Operator(packets) => packets.iter().map(Self::sum_versions).sum(),
            }
    }

    fn eval(&self) -> usize {
        match &self.payload {
            Payload::Value(value) => *value,
            Payload::Operator(packets) => match self.type_id {
                0 => packets.iter().map(Self::eval).sum(),
                1 => packets.iter().map(Self::eval).product(),
                2 => packets.iter().map(Self::eval).min().unwrap(),
                3 => packets.iter().map(Self::eval).max().unwrap(),
                5 => {
                    let (a, b) = packets.iter().map(Self::eval).next_tuple().unwrap();
                    (a > b) as usize
                }
                6 => {
                    let (a, b) = packets.iter().map(Self::eval).next_tuple().unwrap();
                    (a < b) as usize
                }
                7 => {
                    let (a, b) = packets.iter().map(Self::eval).next_tuple().unwrap();
                    (a == b) as usize
                }
                _ => {
                    panic!();
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Payload {
    Value(usize),
    Operator(Vec<Packet>),
}

#[aoc_generator(day16)]
fn gen(input: &str) -> Vec<u8> {
    input
        .chars()
        .flat_map(|c| {
            // Read in each char as a hex value
            let hex = u8::from_str_radix(&c.to_string(), 16).unwrap();
            // Split into 4 bits (MSB -> LSB)
            (0..=3_u8).rev().map(move |b| (hex & (1 << b)) >> b)
        })
        .collect()
}

fn decode_value<'a>(iter: &mut impl Iterator<Item = &'a u8>, bits: usize) -> usize {
    // Decodes a variable length value MSB -> LSB
    iter.take(bits).fold(0, |acc, b| (acc << 1) + *b as usize)
}

#[allow(clippy::redundant_else)]
fn parse_packet<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> (Packet, usize) {
    // Expect 3 bits for the version and 3 for the type_id
    let version = decode_value(iter, 3);
    let type_id = decode_value(iter, 3);
    let mut bits_read = 6;
    if type_id == 4 {
        // Literal value packet
        let mut data: Vec<u8> = Vec::new();
        loop {
            // Read each group of 1 + 4 bits until we find the final group (starts with 0)
            let final_group = *iter.next().unwrap() == 0;
            // Each group contains 4 values
            data.extend(iter.take(4));
            bits_read += 5;
            if final_group {
                break;
            }
        }
        // Read the complete value
        let value = decode_value(&mut data.iter(), data.len());
        // Return the complete packet and bits read
        (Packet::new_value(version, type_id, value), bits_read)
    } else {
        // Operator packet with sub packets
        let mut packets = Vec::new();
        match iter.next().unwrap() {
            0 => {
                // Next 15 bits give the total length of sub packets
                let length = decode_value(iter, 15);
                bits_read += 1 + 15;
                let target = bits_read + length;
                while bits_read < target {
                    // parse sub packets until we have read the length bits
                    let (sub_packet, sub_bits_read) = parse_packet(iter);
                    packets.push(sub_packet);
                    bits_read += sub_bits_read;
                }
            }
            1 => {
                // Next 11 bits give the total number of immediately contained sub packets
                let count = decode_value(iter, 11);
                bits_read += 1 + 11;
                for _ in 0..count {
                    // parse count number of sub packets
                    let (sub_packet, sub_bits_read) = parse_packet(iter);
                    packets.push(sub_packet);
                    bits_read += sub_bits_read;
                }
            }
            _ => {
                panic!();
            }
        }
        // Return the complete packet and bits read
        (Packet::new_operator(version, type_id, packets), bits_read)
    }
}

#[aoc(day16, part1)]
fn part1(input: &Vec<u8>) -> usize {
    let (packet, _) = parse_packet(&mut input.iter());
    packet.sum_versions()
}

#[aoc(day16, part2)]
fn part2(input: &Vec<u8>) -> usize {
    let (packet, _) = parse_packet(&mut input.iter());
    packet.eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_bin() {
        assert_eq!(
            gen("D2FE28"),
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

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
