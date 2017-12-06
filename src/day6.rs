use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let mut input = input
        .split_whitespace()
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut states = HashSet::new();

    for i in 0.. {
        let (index, &value) = input.iter().enumerate().max_by_key(|&(_i, &v)| v).expect(
            "Failed to find max",
        );

        let base = value / input.len();
        let remain = value % input.len();

        input[index] = 0;

        for offset in 0..input.len() {
            let i = (index + offset + 1) % input.len();
            input[i] += base + if offset >= remain { 1 } else { 0 };
        }

        if !states.insert(input.clone()) {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(part1("0 2 7 0"), 5);
    }
}
