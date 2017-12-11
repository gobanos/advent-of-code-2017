#[macro_use]
extern crate aoc2017;

use aoc2017::day10;

fn main() {
    let input = load_input!("day10");
    let input = input.trim();

    println!("Part 1 : {}", day10::part1(input));
    println!("Part 2 : {}", day10::part2(input));
}
