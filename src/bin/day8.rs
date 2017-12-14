#[macro_use]
extern crate aoc2017;
extern crate time;

use aoc2017::day8::*;

fn main() {
    let input = load_input!("day8");
    let input = input.trim();

    let start_part1 = time::precise_time_s();

    println!("Part 1 : {}", part1(input));

    let start_part2 = time::precise_time_s();

    println!("Part 2 : {}", part2(input));

    let stop = time::precise_time_s();

    println!("\nREPORT:");
    println!("\t- Part 1: {:.9}s", start_part2 - start_part1);
    println!("\t- Part 2: {:.9}s", stop - start_part2);
}
