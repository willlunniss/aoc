use std::convert::TryFrom;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
struct Point {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Point {
    /// Create a new point that is advanced forward by it's velocity
    const fn advance(&self) -> Self {
        Self {
            position: (
                self.position.0 + self.velocity.0,
                self.position.1 + self.velocity.1,
            ),
            velocity: self.velocity,
        }
    }
}

impl FromStr for Point {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split into parts and parse as numbers to form a Point
        // e.g. position=< 50769, -40375> velocity=<-5,  4>
        //
        // to   position: (50769, -40375)
        //      velocity: (-5, 4)
        let parts = s
            .split(&['<', ' ', ',', '>'][..])
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        Ok(Self {
            position: (parts[1].parse().unwrap(), parts[2].parse().unwrap()),
            velocity: (parts[4].parse().unwrap(), parts[5].parse().unwrap()),
        })
    }
}

/// Aligns the points and returns the number of seconds it would take to align along with the aligned points
fn align(points: &Vec<Point>) -> (usize, Vec<Point>) {
    // Keep advancing until points 'align' which we can detect as one of the axis max/min inverting e.g. when y max is at a minimum
    let mut prev_y_max = isize::MAX;
    let mut current = points.clone();
    let mut seconds = 0;
    loop {
        // Work out where the points would be next
        let next = current.iter().map(Point::advance).collect::<Vec<Point>>();
        // Check to see if we were already aligned
        let y_max = next.iter().map(|point| point.position.1).max().unwrap();
        if y_max > prev_y_max {
            // Gone past the point of alignment, stop before as current is what we want
            break;
        }
        // Not past alignment yet, advance
        prev_y_max = y_max;
        seconds += 1;
        current = next;
    }
    (seconds, current)
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<Point> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &Vec<Point>) -> String {
    // Align the points
    let (_, aligned) = align(input);
    // Find min/max x/y
    let min_x = aligned.iter().map(|point| point.position.0).min().unwrap();
    let min_y = aligned.iter().map(|point| point.position.1).min().unwrap();
    let max_x = aligned.iter().map(|point| point.position.0).max().unwrap();
    let max_y = aligned.iter().map(|point| point.position.1).max().unwrap();
    // Build a nested vec to render the points offset so that the top left of the image is at 0, 0
    let y_size = usize::try_from(max_y - min_y).unwrap() + 1;
    let x_size = usize::try_from(max_x - min_x).unwrap() + 1;
    let mut image = vec![vec![' '; x_size]; y_size];
    for point in &aligned {
        let y = usize::try_from(point.position.1 - min_y).unwrap();
        let x = usize::try_from(point.position.0 - min_x).unwrap();
        image[y][x] = '█';
    }
    // Print to the console
    for row in image {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
    "↑ Check the printed image ↑".to_owned()
}

#[aoc(day10, part2)]
fn part2(input: &Vec<Point>) -> usize {
    // Align image and return how long it took
    let (duration, _) = align(input);
    duration
}
