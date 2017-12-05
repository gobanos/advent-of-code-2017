#[macro_use]
extern crate aoc2017;

use aoc2017::day4;

fn main() {
    let input = load_input!("day4");
    let input = input.trim();

    println!("Part 1 : {}", day4::part1(input));
    println!("Part 2 : {}", day4::part2(input));
}
