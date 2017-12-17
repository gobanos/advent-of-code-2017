extern crate aoc2017;
extern crate time;

use aoc2017::day17::*;

fn main() {
    let start_part1 = time::precise_time_s();

    println!("Part 1 : {}", part1(366));

    let start_part2 = time::precise_time_s();

    println!("Part 2 : {}", part2(366));

    let stop = time::precise_time_s();

    println!("\nREPORT:");
    println!("\t- Part 1: {:.9}s", start_part2 - start_part1);
    println!("\t- Part 2: {:.9}s", stop - start_part2);
}
