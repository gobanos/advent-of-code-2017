extern crate aoc2017;
extern crate time;

use aoc2017::day3::*;

fn main() {
    let start_part1 = time::precise_time_s();

    println!("Part 1 : {}", part1(361_527));

    let start_part2 = time::precise_time_s();

    println!("Part 2 : {}", part2(361_527));

    let stop = time::precise_time_s();

    println!("\nREPORT:");
    println!("\t- Part 1: {:.9}s", start_part2 - start_part1);
    println!("\t- Part 2: {:.9}s", stop - start_part2);
}
