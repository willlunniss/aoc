use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    Mask {
        value: String,
    },
    Set {
        address: usize,
        value: usize
    }
}

struct AddressMask {
    set: usize,
    floating: usize,
}

struct ValueMask {
    set: usize,
    unset: usize,
}

impl AddressMask {
    /// Applies the mask to the specified value
    /// 
    /// Will generates multiple addresses (x number of floating bits set)
    /// 
    /// Returns a vector of addresses
    fn apply_to(&self, value: usize) -> Vec<usize> {
        let base = value | self.set;
        let mut current = Vec::new();
        current.push(base);
        let mut generated = Vec::new();
        for index in 0 .. 36 {
            let bit_value = 2i64.pow(index) as usize;
            if self.floating & bit_value != 0 {
                // This is a floating bit, create two new values for each address
                // One with it set and one with it not
                for addr in current.iter() {
                    generated.push(addr & !bit_value); // Unset the bit (AND with the bitwise inverse)
                    generated.push(addr | bit_value) // Set the bit (Or with the bit)
                }
                current = generated.clone();
                generated.clear();
            }
        }
        return current;
    }
}

impl ValueMask {
    /// Applies the mask to the specified value
    /// 
    /// Returns the masked value
    fn apply_to(&self, value: usize) -> usize {
        // Use OR to set the bits and AND to unset
        (value | self.set) & self.unset
    }
}

/// Parse the mask in reverse (from lsb to msb)
/// 
/// e.g.
/// mask: 000000000000000000000000000000X1001X 
/// return: [(1,X), (2,1), (4,0), (8,0), (16,1), (32,X), (64,0),...]
fn get_mask_bits_values(mask: &str) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    for (index, value) in mask.char_indices().rev() {
        // Calculate the value of the bit
        // As we get the chars in revese we then need to re-revese the index
        let bit_value = 2i64.pow((mask.len() - index - 1) as u32);
        result.push((bit_value as usize, value));
    }
    return result;
}

impl FromStr for AddressMask {
    type Err = Infallible;
    fn from_str(mask: &str) -> Result<Self, Self::Err> {
        // Parse the mask in reverse (from lsb to msb)
        //      src: 000000000000000000000000000000X1001X ->
        //      set: 000000000000000000000000000000010010 (used to OR)
        // floating: 000000000000000000000000000000100001 (used to create two values, one with it unset and another with it set)
        let mut set = 0;
        let mut floating = 0;
        for (bit_value, value) in get_mask_bits_values(mask) {
            match value {
                'X' => { floating += bit_value },
                '1' => { set += bit_value;},
                '0' => {}, // Nothing to do
                _ => { panic!("Unrecognised mask bit '{}'", value); }
            }
        }
        return Ok(AddressMask{set: set, floating: floating});
    }
}

impl FromStr for ValueMask {    
    type Err = Infallible;
    fn from_str(mask: &str) -> Result<Self, Self::Err> {
        // Parse the mask in reverse (from lsb to msb)
        //   src: XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X ->
        //   set: 000000000000000000000000000001000000 (used to OR)
        // unset: 111111111111111111111111111111111101 (used with AND)
        let mut set = 0;
        let mut unset = 0;
        for (bit_value, value) in get_mask_bits_values(mask) {
            match value {
                'X' => { unset += bit_value },
                '1' => { set += bit_value; unset += bit_value; },
                '0' => {}, // Nothing to do
                _ => { panic!("Unrecognised mask bit '{}'", value); }
            }
        }
        return Ok(ValueMask{set: set, unset: unset});
    }
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let (instr, value) = line.splitn(2, " = ").collect_tuple().unwrap();
        if instr == "mask" {
            commands.push(Command::Mask{value: value.to_string()});
        } else if instr.starts_with("mem") {
            // Extract address from mem command 
            // mem[8] 
            let addr = &instr[4 .. &instr.len() - 1];
            commands.push(Command::Set{address: addr.parse().unwrap(), value: value.parse().unwrap()});
        } else {
            panic!("Unrecognised instruction '{}'", line);
        }
    }
    return commands;
}

#[aoc(day14, part1)]
fn part1(input: &Vec<Command>) -> usize {
    let mut mask = ValueMask{set: 0, unset: 0};
    let mut mem : HashMap<usize, usize> = HashMap::new();
    for command in input {
        match command {
            Command::Mask { value } => { mask = value.parse().unwrap(); },
            Command::Set { address, value}  => { mem.insert(*address, mask.apply_to(*value)); }
        }
    }
    return mem.values().sum();
}

#[aoc(day14, part2)]
fn part2(input: &Vec<Command>) -> usize {
    let mut mask = AddressMask{set: 0, floating: 0};
    let mut mem : HashMap<usize, usize> = HashMap::new();
    for command in input {
        match command {
            Command::Mask { value } => { mask = value.parse().unwrap(); },
            Command::Set { address, value}  => { 
                // Set the value in all addresses produced by applying the address mask
                for generated_address in mask.apply_to(*address) {
                    mem.insert(generated_address, *value);
                }
            }
        }
    }
    return mem.values().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addressmask_apply() {
        let mask : AddressMask = "000000000000000000000000000000X1001X".parse().unwrap();
        assert_eq!(mask.apply_to(42), [26, 58, 27, 59]);
    }

    #[test]
    fn test_valuemask_apply() {
        let mask : ValueMask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();
        assert_eq!(mask.apply_to(11), 73);

        assert_eq!(mask.apply_to(101), 101);

        assert_eq!(mask.apply_to(0), 64);
    }
}