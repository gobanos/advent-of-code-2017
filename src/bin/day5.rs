#[macro_use]
extern crate aoc2017;

use aoc2017::day5;

fn main() {
    let input = load_input!("day5");
    let input = input.trim();

    println!("Part 1 : {}", day5::part1(input));
    println!("Part 2 : {}", day5::part2(input));
}
