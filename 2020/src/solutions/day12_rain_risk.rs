use parse_display::{Display, FromStr};
use std::str::FromStr;
use std::convert::Infallible;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
pub enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}

#[derive(Display, PartialEq, Debug, Clone, Copy)]
#[display("{action}{value}")]
pub struct NavInstruction {
    action: Action,
    value: isize
}

impl FromStr for NavInstruction {    
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into first char (action) and remaining (value)
        let (a, v) = s.split_at(1);
        // Then parse into the right types
        return Ok(NavInstruction { action: a.parse().unwrap(), value: v.parse().unwrap()});
    }
}

#[derive(Display, PartialEq, Debug, Clone, Copy)]
#[display("{latitude},{longitude}: -> {direction}")]
pub struct Ship {
    latitude: isize,
    longitude: isize,
    direction: isize, // 0 = North, 90 = East, 180 = South, 270 = West
}

impl Ship {
    pub fn manhattan_distance(&self) -> isize {
        return self.latitude.abs() + self.longitude.abs();
    }
}

#[derive(Display, PartialEq, Debug, Clone, Copy)]
#[display("{latitude},{longitude}")]
pub struct Waypoint {
    latitude: isize,
    longitude: isize,
}

impl Waypoint {
    pub fn rotate(&mut self, angle: isize) {
        // From https://matthew-brett.github.io/teaching/rotation_2d.html
        let cos = (-angle as f64).to_radians().cos();
        let sin = (-angle as f64).to_radians().sin();
        let long = ((self.longitude as f64) * cos) - ((self.latitude as f64) * sin);
        let lat = ((self.longitude as f64) * sin) + ((self.latitude as f64) * cos);
        self.longitude = long.round() as isize;
        self.latitude = lat.round() as isize;
    }
}

pub fn move_ship(ship: &mut Ship, instr: &NavInstruction) {
    match instr.action {
        Action::N => ship.latitude += instr.value,
        Action::S => ship.latitude -= instr.value,
        Action::E => ship.longitude += instr.value,
        Action::W => ship.longitude -= instr.value,
        Action::L => ship.direction = (ship.direction - instr.value + 360) % 360,
        Action::R => ship.direction = (ship.direction + instr.value + 360) % 360,
        Action::F => {
            match ship.direction {
                0 => ship.latitude += instr.value,
                90 => ship.longitude += instr.value,
                180 => ship.latitude -= instr.value,
                270 => ship.longitude -= instr.value,
                _ => panic!("Don't know how to handle moving at an angle of {}", ship.direction)
            }
        }
    }
}

pub fn move_ship_and_waypoint(ship: &mut Ship, waypoint: &mut Waypoint, instr: &NavInstruction) {
    match instr.action {
        Action::N => waypoint.latitude += instr.value,
        Action::S => waypoint.latitude -= instr.value,
        Action::E => waypoint.longitude += instr.value,
        Action::W => waypoint.longitude -= instr.value,
        Action::L => waypoint.rotate(-instr.value),
        Action::R => waypoint.rotate(instr.value),
        Action::F => {
            ship.latitude += waypoint.latitude * instr.value;
            ship.longitude += waypoint.longitude * instr.value;
        }
    }
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> Vec<NavInstruction> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day12, part1)]
fn part1(input: &Vec<NavInstruction>) -> isize {
    let mut ship = Ship {latitude: 0, longitude:0, direction: 90};
    for instr in input {
        move_ship(&mut ship, &instr);
    }
    
    return ship.manhattan_distance();
}

#[aoc(day12, part2)]
fn part2(input: &Vec<NavInstruction>) -> isize {
    let mut ship = Ship {latitude: 0, longitude:0, direction: 90};
    let mut waypoint = Waypoint {latitude: 1, longitude: 10};
    for instr in input {
        move_ship_and_waypoint(&mut ship, &mut waypoint, &instr);
    }
    
    return ship.manhattan_distance();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_rotate() {
        let mut waypoint = Waypoint{latitude: -5, longitude: 5};
        waypoint.rotate(-90);
        assert_eq!(waypoint.latitude, 5);
        assert_eq!(waypoint.longitude, 5);

        let mut waypoint = Waypoint{latitude: 4, longitude: 10};
        waypoint.rotate(90);
        assert_eq!(waypoint.latitude, -10);
        assert_eq!(waypoint.longitude, 4);

        
        let mut waypoint = Waypoint{latitude: -5, longitude: 10};
        waypoint.rotate(90);
        waypoint.rotate(90);
        waypoint.rotate(90);
        assert_eq!(waypoint.latitude, 10);
        assert_eq!(waypoint.longitude, 5);
    }
}