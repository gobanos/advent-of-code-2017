#[macro_use]
extern crate aoc2017;

use aoc2017::day2;

fn main() {
    let input = load_input!("day2");
    let input = input.trim();

    println!("Part 1 : {}", day2::part1(input));
    println!("Part 2 : {}", day2::part2(input));
}
