use crate::intcode::Intcode;
use itertools::Itertools;
use std::collections::HashMap;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
struct Room {
    name: String,
    doors: Vec<String>,
    item: Option<String>,
}

impl FromStr for Room {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Decodes ASCII output for a room e.g.
        //
        //
        //
        // == Kitchen ==
        //Everything's freeze-dried.
        //
        //Doors here lead:
        //- north
        //- west
        //
        //Items here:
        //- escape pod
        //
        //Command?
        
        // Skip the first 3 blank lines
        let mut lines = s.lines().skip(3);
        // Get the name of the room
        let name = lines.next().unwrap();
        // Skip to start of doors
        let mut lines = lines.skip(3);
        let mut doors = Vec::new();
        loop {
            // Get the names of the doors 
            let mut chars = lines.next().unwrap().chars();
            if chars.next() == Some('-') {
                doors.push(chars.skip(1).collect());
            } else {
                break;
            }
        }
        // See if there is an item here (only expect 1 unless we have dropped something)
        let item = if lines.next().unwrap() == "Items here:" {
            Some(lines.next().unwrap().chars().skip(2).collect())
        } else {
            None
        };
        Ok(Self {
            name: name.to_owned(),
            doors,
            item,
        })
    }
}

struct Droid {
    controller: Intcode,
}

impl Droid {
    fn new(program: &str) -> Self {
        Self {
            controller: Intcode::from_with(program, 1024 * 1024),
        }
    }

    /// Returns the name of the door to go backwards
    fn backwards(door: Option<&String>) -> Option<String> {
        door.and_then(|door| match &door[..] {
            "north" => Some("south".to_owned()),
            "east" => Some("west".to_owned()),
            "south" => Some("north".to_owned()),
             "west" => Some("east".to_owned()),
            _ => None
        })
    }

    /// Checks if an item is safe to take
    fn safe_to_take(item: &str) -> bool {
        !matches!(
            item,
            "photons" | "molten lava" | "giant electromagnet" | "escape pod" | "infinite loop"
        )
    }

    /// Performs the specified action on the supplied items (e.g. drop/take)
    fn act_on_items(&mut self, items: &[&String], action: &str) {
        for &item in items {
            let mut command = "".to_owned();
            command.push_str(action);
            command.push(' ');
            command.push_str(item);
            self.controller.inputln(&command);
        }
        self.controller.run();
        self.controller.outputs().clear();
    }

    /// Tries to verify identify at a security checkpoint
    /// 
    /// Moves through the specified door holding different combinations of items to
    /// try to achieve the correct weight in order to be let through the pressure sensitive floor
    /// Shouldn't be called until all items from the ship have been collected
    fn try_verify_identity(&mut self, door :&str) -> usize {
        // Find out what we are holding (check inventory)
        let mut inventory: Vec<String> = Vec::new();
        self.controller.inputln("inv");
        self.controller.run();
        for line in self.controller.outputs_as_ascii().lines().skip(1) {
            let mut chars = line.chars();
            if chars.next() == Some('-') {
                inventory.push(chars.skip(1).collect());
            }
        }
        // Drop everything to start
        self.act_on_items(&inventory.iter().collect::<Vec<_>>(), "drop");

        // Pick up every combination of 3 or more items and try to move through the door
        for num_items in 3..inventory.len() {
            for combination in inventory.iter().combinations(num_items) {
                // Pick up the items that we are going to try with
                self.act_on_items(&combination, "take");
                // Try to move through the door
                self.controller.inputln(door);
                self.controller.run();
                // Parse output
                let output = self.controller.outputs_as_ascii();
                // Interested in the last line
                let last = output.lines().last().unwrap();
                if last != "Command?" {
                    // Got through the door!, extract the password (series of numbers)
                    return last
                        .chars()
                        .filter(|c| c.is_digit(10))
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                }
                // Didn't work (have been pushed back to the checkpoint), drop what we were carrying and try again
                self.act_on_items(&combination, "drop");
            }
        }
        // Nothing worked, must not have picked up all items from the ship...
        0
    }

    /// Explores the ship and returns the password for the main airlock
    /// 
    /// Goes through every door picking up items that are safe until the ship
    /// has been fully explored.
    /// Once fully explored the droid heads to the security checkpoint and attempts
    /// to work out what combination of items are required to get past the pressure
    /// sensitive floor.
    /// Once through get the password from Santa
    fn find_password(&mut self) -> usize {
        // Map of rooms to unexplored doors
        let mut unexplored_doors: HashMap<String, Vec<String>> = HashMap::new();
        // The current direct route from the start point
        let mut route: Vec<String> = Vec::new();
        // The route to the security checkpoint
        let mut security_checkpoint_route: Vec<String> = Vec::new();
        loop {
            // Process last input and get back info about the room
            self.controller.run();
            let output = self.controller.outputs_as_ascii();
            let room = output.parse::<Room>().unwrap();
            // Get/record which doors we still need to go through
            let doors = unexplored_doors.entry(room.name.clone()).or_insert_with(||{
                // Found a new room
                // If the room has an item that is safe to take then pick it up
                if let Some(item) = room.item {
                    if Self::safe_to_take(&item) {
                        let mut command = "take ".to_owned();
                        command.push_str(&item);
                        // Pick up the item and discard output message about what we just picked up
                        self.controller.inputln(&command);
                        self.controller.run();
                        self.controller.outputs().clear();
                    }
                }
                if room.name == "== Security Checkpoint ==" {
                    // Found the security checkpoint
                    // Make a note of how we got here
                    security_checkpoint_route = route.clone();
                    // Can't go further until we have fully explored the ship though so record 0 doors
                    return Vec::new();
                }
                // Record which doors are present, excluding the one that takes up back to where we came
                if let Some(backwards) = Self::backwards(route.last()) {
                    room.doors.iter().filter_map(|door| if *door != backwards { Some(door.clone())} else { None }).collect()
                } else {
                    room.doors.clone()
                }
            });
            // Now work out where to go next
            if doors.is_empty() {
                // Fully explored this room, go back
                if route.is_empty() {
                    // At the start so must have explored everywhere!
                    // Go to back to the checkpoint
                    security_checkpoint_route.iter().for_each(|door| self.controller.inputln(door));
                    self.controller.run();
                    self.controller.outputs().clear();
                    // Now try to get through, returning the password that we should find on the other side
                    return self.try_verify_identity(security_checkpoint_route.last().unwrap());
                }
                // Go backwards through the last door we came from
                self.controller.inputln(&Self::backwards(route.last()).unwrap());
                route.pop();
            } else {
                // Have a door that we haven't explored yet, let's go there and see what's in that room
                let next = doors.pop().unwrap();
                self.controller.inputln(&next);
                route.push(next);
            }
        }
    }
}

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let mut droid = Droid::new(input);

    droid.find_password()
}
