extern crate aoc2017;

use aoc2017::day1;

fn main() {
    let input = aoc2017::load_input("day1").expect("Failed to load day1 input");
    let input = input.trim();

    println!("Part 1 : {}", day1::part1(input));
    println!("Part 2 : {}", day1::part2(input));
}
