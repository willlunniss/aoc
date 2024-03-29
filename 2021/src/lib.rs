#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

extern crate aoc_runner;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate cached;

#[macro_use]
extern crate aoc_runner_derive;

pub mod solutions;

aoc_lib! { year = 2021 }
