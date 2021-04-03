use chrono::format::ParseError;
use chrono::{NaiveDateTime, Timelike};
use itertools::Itertools;
use std::collections::HashMap;
use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
enum Action {
    GuardBeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

impl FromStr for Action {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => Ok(Self::FallsAsleep),
            "wakes up" => Ok(Self::WakesUp),
            _ => Ok(Self::GuardBeginsShift(
                s.splitn(3, ' ').collect::<Vec<&str>>()[1][1..]
                    .parse()
                    .unwrap(),
            )),
        }
    }
}

#[derive(Debug)]
struct Entry {
    date: NaiveDateTime,
    action: Action,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("] ").collect::<Vec<_>>();
        Ok(Self {
            date: NaiveDateTime::parse_from_str(parts[0], "[%Y-%m-%d %H:%M")?,
            action: parts[1].parse().unwrap(),
        })
    }
}

#[aoc_generator(day4)]
fn gen(input: &str) -> HashMap<usize, Vec<usize>> {
    // Convert input into a sorted list of Entries
    let entries = input
        .lines()
        .sorted()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Entry>>();
    // Go through all entries building up a record of which minutes each guard was asleep
    let mut guard = 0;
    let mut records: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut fell_asleep = 0;
    for entry in entries {
        match entry.action {
            Action::GuardBeginsShift(id) => guard = id,
            Action::FallsAsleep => fell_asleep = entry.date.minute(),
            Action::WakesUp => {
                // When a guard wakes up, get the record for that guard
                let record = records.entry(guard).or_insert_with(|| vec![0; 60]);
                // and increment a counter for each individual minute that they were sleeping
                (fell_asleep..entry.date.minute()).for_each(|minute| record[minute as usize] += 1);
            }
        }
    }
    records
}

#[aoc(day4, part1)]
fn part1(input: &HashMap<usize, Vec<usize>>) -> usize {
    // Find the guard that slept the most
    let (id, record) = input
        .iter()
        .max_by_key(|(_, record)| record.iter().sum::<usize>())
        .unwrap();
    // Return the minute that they were asleep the most multiplied by their id
    let (minute, _) = record
        .iter()
        .enumerate()
        .max_by_key(|&(_, asleep)| asleep)
        .unwrap();
    id * minute
}

#[aoc(day4, part2)]
fn part2(input: &HashMap<usize, Vec<usize>>) -> usize {
    // Find the guard which is most frequently asleep on the same minute
    let (id, record) = input
        .iter()
        .max_by_key(|(_, record)| record.iter().max().unwrap())
        .unwrap();
    // Return the minute that they were asleep the most multiplied by their id
    let (minute, _) = record
        .iter()
        .enumerate()
        .max_by_key(|&(_, asleep)| asleep)
        .unwrap();
    id * minute
}
