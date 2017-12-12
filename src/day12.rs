use day12_parser::parse;

use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> u32 {
    let input = parse(input);

    let mut g = HashSet::new();

    group(0, &input, &mut g);

    g.len() as u32
}

fn group(item: u32, nodes: &HashMap<u32, Vec<u32>>, g: &mut HashSet<u32>) {
    for &child in &nodes[&item] {
        if g.insert(child) {
            group(child, nodes, g);
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let mut input = parse(input);

    let mut i = 0;
    while !input.is_empty() {
        let &key = input.keys().next().unwrap();
        let mut g = HashSet::new();
        group(key, &input, &mut g);

        for k in g {
            input.remove(&k);
        }

        i += 1;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(INPUT), 6)
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(INPUT), 2)
    }
}
