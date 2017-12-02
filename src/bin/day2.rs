extern crate aoc2017;

use aoc2017::day2;

fn main() {
    let input = aoc2017::load_input("day2").expect("Failed to load day2 input");
    let input = input.trim();

    println!("Part 1 : {}", day2::part1(input));
    println!("Part 2 : {}", day2::part2(input));
}
