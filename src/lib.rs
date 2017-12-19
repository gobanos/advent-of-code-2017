extern crate day7_parser;
extern crate day8_parser;
extern crate day9_parser;
extern crate day12_parser;
extern crate day18_parser;

extern crate petgraph;
extern crate rayon;

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
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

pub mod knot_hasher;

#[macro_export]
macro_rules! load_input {
    ($file:expr) => ({ include_str!(concat!("../../resources/", $file, ".txt"))})
}
