#[macro_use]
extern crate aoc2017;

use aoc2017::day9;

fn main() {
    let input = load_input!("day9");
    let input = input.trim();

    println!("Part 1 : {}", day9::part1(input));
    println!("Part 2 : {}", day9::part2(input));
}
