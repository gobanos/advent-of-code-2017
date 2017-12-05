#[macro_use]
extern crate aoc2017;

use aoc2017::day1;

fn main() {
    let input = load_input!("day1");
    let input = input.trim();

    println!("Part 1 : {}", day1::part1(input));
    println!("Part 2 : {}", day1::part2(input));
}
