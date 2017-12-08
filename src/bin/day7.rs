#[macro_use]
extern crate aoc2017;

use aoc2017::day7;

fn main() {
    let input = load_input!("day7");
    let input = input.trim();

    println!("Part 1 : {}", day7::part1(input));
    println!("Part 2 : {}", day7::part2(input));
}
