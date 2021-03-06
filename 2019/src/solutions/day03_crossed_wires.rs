use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Path {
    waypoints: Vec<Point>,
}

impl Path {
    /// Follows the path returning every point that is visited
    /// and the distance travelled to get there
    fn follow(&self) -> Vec<(Point, usize)> {
        let mut points = Vec::new();
        // Start at the origin
        let mut pos = Point { x: 0, y: 0 };
        let mut distance = 0;
        for waypoint in &self.waypoints {
            // For each waypoint, move from the current pos to it creating points along the way
            // We expect to only ever move in one axis per waypoint
            // Work out if we are moving +1/-1 or 0 in each axis
            let x_step = if waypoint.x == 0 { 0 } else { waypoint.x / isize::abs(waypoint.x) } ;
            let y_step = if waypoint.y == 0 { 0 } else { waypoint.y / isize::abs(waypoint.y) } ;
            for _ in 0..waypoint.manhattan_distance() {
                // Move one step towards the waypoint creating points as we go
                pos.x += x_step;
                pos.y += y_step;
                distance += 1;
                points.push((pos.clone(), distance));
            }
        }
        return points;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance_str) = s.split_at(1);
        let distance = distance_str.parse::<isize>().unwrap();
        return Ok(match direction {
            "U" => Point { x: 0, y: distance },
            "D" => Point { x: 0, y: -distance },
            "L" => Point { x: -distance, y: 0 },
            "R" => Point { x: distance, y: 0 },
            _ => panic!("Unexpected direction {}", direction),
        });
    }
}

impl Point {
    fn manhattan_distance(&self) -> isize {
        return isize::abs(self.x) + isize::abs(self.y);
    }
}

#[aoc_generator(day3)]
fn gen(input: &str) -> Vec<Path> {
    input
        .lines()
        .map(|line| Path {
            waypoints: line.split(",").map(|p| p.parse().unwrap()).collect(),
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Vec<Path>) -> isize {
    let (path1, path2) = input.iter().collect_tuple().unwrap();
    // Get the points along both paths in a set
    let points1: HashSet<Point> = path1.follow().into_iter().map(|(point, _)| point).collect();
    let points2: HashSet<Point> = path2.follow().into_iter().map(|(point, _)| point).collect();

    // Calculate the intersections between the two paths (cross over points)
    let intersections: HashSet<_> = points1.intersection(&points2).collect();
    // Find the intersection point with the smallest manhattan distance
    return intersections
        .iter()
        .map(|point| point.manhattan_distance())
        .min()
        .unwrap();
}

#[aoc(day3, part2)]
fn part2(input: &Vec<Path>) -> usize {
    let (path1, path2) = input.iter().collect_tuple().unwrap();
    // Get the points along both paths and associated distance as hashmap
    let points1: HashMap<Point, usize> = path1.follow().into_iter().collect();
    let points2: HashMap<Point, usize> = path2.follow().into_iter().collect();

    // For all points that exist on both paths (an intersection)
    // Calculate combined the distance travelled for each path to get to that point
    // Return the smallest
    return points1
        .iter()
        .filter(|(point, _)| points2.contains_key(point))
        .map(|(point, distance)| distance + points2.get(point).unwrap())
        .min()
        .unwrap();
}
