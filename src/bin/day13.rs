#[macro_use]
extern crate aoc2017;

use aoc2017::day13;

fn main() {
    let input = load_input!("day13");
    let input = input.trim();

    println!("Part 1 : {}", day13::part1(input));
    println!("Part 2 : {}", day13::part2(input));
}
