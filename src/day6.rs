use std::collections::HashSet;
use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut input = input
        .split_whitespace()
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut states = HashSet::new();

    for i in 1.. {
        reallocate(&mut input);

        if !states.insert(input.clone()) {
            return i;
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> u32 {
    let mut input = input
        .split_whitespace()
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut states = HashMap::new();

    for i in 1.. {
        reallocate(&mut input);

        if let Some(previous_index) = states.insert(input.clone(), i) {
            return i - previous_index;
        }
    }

    unreachable!()
}

fn reallocate(memory: &mut [usize]) {
    let (index, value) = find_max(memory);

    let base = value / memory.len();
    let remain = value % memory.len();

    memory[index] = 0;

    for offset in 0..memory.len() {
        let i = (index + offset + 1) % memory.len();
        memory[i] += if offset < remain { base + 1 } else { base };
    }
}

fn find_max(memory: &[usize]) -> (usize, usize) {
    let (k, &v) = memory
        .iter()
        .enumerate()
        .max_by_key(|&(i, &v)| (v, -(i as isize)))
        .expect("Failed to find max");

    (k, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// The banks start with 0, 2, 7, and 0 blocks. The third bank has the most blocks,
    /// so it is chosen for redistribution.
    fn part1_max_sample1() {
        assert_eq!(find_max(&[0, 2, 7, 0]), (2, 7));
    }

    #[test]
    /// Starting with the next bank (the fourth bank) and then continuing to the first bank,
    /// the second bank, and so on, the 7 blocks are spread out over the memory banks.
    /// The fourth, first, and second banks get two blocks each, and the third bank gets one back.
    /// The final result looks like this: 2 4 1 2.
    fn part1_reallocate_sample1() {
        let mut memory = vec![0, 2, 7, 0];
        reallocate(&mut memory);

        assert_eq!(memory, vec![2, 4, 1, 2]);
    }

    #[test]
    /// Next, the second bank is chosen because it contains the most blocks (four).
    /// Because there are four memory banks, each gets one block. The result is: 3 1 2 3.
    fn part1_max_sample2() {
        assert_eq!(find_max(&[2, 4, 1, 2]), (1, 4));
    }

    #[test]
    /// Next, the second bank is chosen because it contains the most blocks (four).
    /// Because there are four memory banks, each gets one block. The result is: 3 1 2 3.
    fn part1_reallocate_sample2() {
        let mut memory = vec![2, 4, 1, 2];
        reallocate(&mut memory);

        assert_eq!(memory, vec![3, 1, 2, 3]);
    }

    #[test]
    /// Now, there is a tie between the first and fourth memory banks,
    /// both of which have three blocks. The first bank wins the tie,
    /// and its three blocks are distributed evenly over the other three banks,
    /// leaving it with none: 0 2 3 4.
    fn part1_max_sample3() {
        assert_eq!(find_max(&[3, 1, 2, 3]), (0, 3));
    }

    #[test]
    /// Now, there is a tie between the first and fourth memory banks,
    /// both of which have three blocks. The first bank wins the tie,
    /// and its three blocks are distributed evenly over the other three banks,
    /// leaving it with none: 0 2 3 4.
    fn part1_reallocate_sample3() {
        let mut memory = vec![3, 1, 2, 3];
        reallocate(&mut memory);

        assert_eq!(memory, vec![0, 2, 3, 4]);
    }

    #[test]
    /// The fourth bank is chosen, and its four blocks are distributed such
    /// that each of the four banks receives one: 1 3 4 1.
    fn part1_max_sample4() {
        assert_eq!(find_max(&[0, 2, 3, 4]), (3, 4));
    }

    #[test]
    /// The fourth bank is chosen, and its four blocks are distributed such
    /// that each of the four banks receives one: 1 3 4 1.
    fn part1_reallocate_sample4() {
        let mut memory = vec![0, 2, 3, 4];
        reallocate(&mut memory);

        assert_eq!(memory, vec![1, 3, 4, 1]);
    }

    #[test]
    /// The third bank is chosen, and the same thing happens: 2 4 1 2.
    fn part1_max_sample5() {
        assert_eq!(find_max(&[1, 3, 4, 1]), (2, 4));
    }

    #[test]
    /// The third bank is chosen, and the same thing happens: 2 4 1 2.
    fn part1_reallocate_sample5() {
        let mut memory = vec![1, 3, 4, 1];
        reallocate(&mut memory);

        assert_eq!(memory, vec![2, 4, 1, 2]);
    }

    #[test]
    fn part1_sample1() {
        assert_eq!(part1("0 2 7 0"), 5);
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(part2("0 2 7 0"), 4);
    }
}
