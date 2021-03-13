#![warn(clippy::all)]

extern crate aoc_runner;

extern crate digits_iterator;

#[macro_use]
extern crate aoc_runner_derive;

pub mod intcode;
pub mod solutions;
pub mod utils;

aoc_lib! { year = 2019 }
