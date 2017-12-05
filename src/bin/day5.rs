extern crate aoc2017;

use aoc2017::day5;

fn main() {
    let input = aoc2017::load_input("day5").expect("Failed to load day5 input");
    let input = input.trim();

    println!("Part 1 : {}", day5::part1(input));
    println!("Part 2 : {}", day5::part2(input));
}
