#[macro_use]
extern crate aoc2017;
extern crate time;

use aoc2017::day25::*;

fn main() {
    let input = load_input!("day25");

    let start_part1 = time::precise_time_s();

    println!("Part 1 : {}", part1(input));

    let stop = time::precise_time_s();

    println!("\nREPORT:");
    println!("\t- Part 1: {:.9}s", stop - start_part1);
}
