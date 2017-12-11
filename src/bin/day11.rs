#[macro_use]
extern crate aoc2017;

use aoc2017::day11;

fn main() {
    let input = load_input!("day11");
    let input = input.trim();

    println!("Part 1 : {}", day11::part1(input));
    println!("Part 2 : {}", day11::part2(input));
}
