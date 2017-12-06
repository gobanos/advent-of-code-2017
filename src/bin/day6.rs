#[macro_use]
extern crate aoc2017;

use aoc2017::day6;

fn main() {
    let input = load_input!("day6");
    let input = input.trim();

    println!("Part 1 : {}", day6::part1(input));
    // println!("Part 2 : {}", day6::part2(input));
}
