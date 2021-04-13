use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::Infallible;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
pub struct Range {
    lower: RangeInclusive<usize>,
    upper: RangeInclusive<usize>,
}

impl Range {
    fn parse_sub_range(s: &str) -> RangeInclusive<usize> {
        let (start, end) = s
            .splitn(2, "-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        return RangeInclusive::new(start, end);
    }

    fn contains(&self, value: usize) -> bool {
        return self.lower.contains(&value) || self.upper.contains(&value);
    }
}

impl FromStr for Range {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, upper) = s
            .split(" or ")
            .map(|x| Range::parse_sub_range(x))
            .collect_tuple()
            .unwrap();
        return Ok(Range {
            lower: lower,
            upper: upper,
        });
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct TicketData {
    rules: HashMap<String, Range>,
    ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl TicketData {
    pub fn matches_any_rule(&self, value: usize) -> bool {
        for range in &mut self.rules.values() {
            if range.contains(value) {
                return true;
            }
        }
        return false;
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TicketDataSection {
    Rules,
    TicketHeader,
    Ticket,
    NearbyTicketsHeader,
    NearbyTickets,
}

impl TicketDataSection {
    pub fn advance(&mut self) {
        use TicketDataSection::*;
        *self = match self {
            Rules => TicketHeader,
            TicketHeader => Ticket,
            Ticket => NearbyTicketsHeader,
            NearbyTicketsHeader => NearbyTickets,
            NearbyTickets => panic!("Cannot advance {:?}", self),
        }
    }
}

#[aoc_generator(day16)]
pub fn gen(input: &str) -> TicketData {
    use TicketDataSection::*;
    let mut section = Rules;
    let mut rules = HashMap::new();
    let mut ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            // End of this section, move on to next
            section.advance();
            continue;
        }
        // Parse the input data depending on what section we are in
        match section {
            Rules => {
                let (name, ranges) = line.splitn(2, ": ").collect_tuple().unwrap();
                rules.insert(name.to_string(), ranges.parse().unwrap());
            }
            TicketHeader => {
                section.advance();
                continue;
            } // Skip the header
            Ticket => ticket = line.split(',').map(|x| x.parse().unwrap()).collect(),
            NearbyTicketsHeader => {
                section.advance();
                continue;
            } // Skip the header
            NearbyTickets => {
                nearby_tickets.push(line.split(',').map(|x| x.parse().unwrap()).collect())
            }
        }
    }
    return TicketData {
        rules: rules,
        ticket: ticket,
        nearby_tickets: nearby_tickets,
    };
}

#[aoc(day16, part1)]
fn part1(input: &TicketData) -> usize {
    // Find all invalid values
    let mut invalid: Vec<usize> = Vec::new();
    for ticket in input.nearby_tickets.iter() {
        for value in ticket.iter() {
            if !&input.matches_any_rule(*value) {
                invalid.push(*value);
            }
        }
    }
    return invalid.iter().sum();
}

#[aoc(day16, part2)]
fn part2(input: &TicketData) -> usize {
    // First discard out all invalid tickets
    let mut valid: Vec<Vec<usize>> = Vec::new();
    for ticket in input.nearby_tickets.iter() {
        let mut invalid = false;
        for value in ticket.iter() {
            if !input.matches_any_rule(*value) {
                invalid = true;
            }
        }
        if !invalid {
            valid.push(ticket.clone());
        }
    }
    // And add our ticket as we know that is valid
    valid.push(input.ticket.clone());
    // Start with every field being potentially valid for the value
    let mut possibilities = vec![input.rules.clone(); input.ticket.len()];
    // Then filter out what's not possible
    for ticket in &valid {
        for (index, value) in ticket.iter().enumerate() {
            &possibilities[index].retain(|_, range| range.contains(*value));
        }
    }

    // Now repeatedly pass over the options until each filed only has one possibility
    // (Find the fields with only one option and remove that option from others, and repeat...)
    let mut resolved_field_names: Vec<String> = vec!["?".to_string(); input.ticket.len()];
    loop {
        let mut single_options = HashSet::new();
        for (index, possible_fields) in possibilities.iter().enumerate() {
            if possible_fields.len() == 1 {
                // This field only has one option so we know it needs to be this
                let rule = possible_fields.keys().next().unwrap();
                resolved_field_names[index] = rule.to_string();
                single_options.insert(rule.to_string());
            }
        }
        if single_options.is_empty() {
            // Can't do anything more
            break;
        } else {
            // Remove the possible fields from ones with > 1
            for rule in single_options {
                for index in 0..input.ticket.len() {
                    &possibilities[index].retain(|name, _| *name != rule);
                }
            }
        }
    }
    // Now finally multiply all of the departure field values
    let mut departure_fields_multiplied = 1;
    for (index, name) in resolved_field_names.iter().enumerate() {
        if name.starts_with("departure") {
            departure_fields_multiplied *= &input.ticket[index];
        }
    }
    return departure_fields_multiplied;
}
