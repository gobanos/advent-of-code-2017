extern crate day7_parser;
extern crate day8_parser;
extern crate day9_parser;
extern crate day12_parser;

extern crate petgraph;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;

#[macro_export]
macro_rules! load_input {
    ($file:expr) => ({ include_str!(concat!("../../resources/", $file, ".txt"))})
}
