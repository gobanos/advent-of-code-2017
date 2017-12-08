#[macro_use]
extern crate aoc2017;

use aoc2017::day8;

fn main() {
    let input = load_input!("day8");
    let input = input.trim();

    println!("Part 1 : {}", day8::part1(input));
    println!("Part 2 : {}", day8::part2(input));
}
