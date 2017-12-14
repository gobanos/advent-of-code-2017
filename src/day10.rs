use knot_hasher::KnotHasher;

pub fn part1(input: &str) -> u32 {
    let mut hasher = KnotHasher::new(256);

    input
        .split(',')
        .filter_map(|number| number.parse::<u8>().ok())
        .for_each(|len| hasher.hash(len));

    u32::from(hasher.list[0]) * u32::from(hasher.list[1])
}

pub fn part2(input: &str) -> String {
    KnotHasher::dense_hash_from_key(input)
        .iter()
        .map(|xor| format!("{:02x}", xor))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use knot_hasher::KnotHasher;

    #[test]
    /// Suppose we instead only had a circular list containing five elements, 0, 1, 2, 3, 4,
    /// and were given input lengths of 3, 4, 1, 5.
    ///
    /// - The list begins as [0] 1 2 3 4 (where square brackets indicate the current position).
    /// - The first length, 3, selects ([0] 1 2) 3 4
    ///   (where parentheses indicate the sublist to be reversed).
    /// - After reversing that section (0 1 2 into 2 1 0), we get ([2] 1 0) 3 4.
    /// - Then, the current position moves forward by the length, 3, plus the skip size,
    ///   0: 2 1 0 [3] 4. Finally, the skip size increases to 1.
    fn sample_step1() {
        let mut hasher = step0();
        hasher.hash(3);
        assert_eq!(hasher, step1())
    }

    #[test]
    /// - The second length, 4, selects a section which wraps: 2 1) 0 ([3] 4.
    /// - The sublist 3 4 2 1 is reversed to form 1 2 4 3: 4 3) 0 ([1] 2.
    /// - The current position moves forward by the length plus the skip size, a total of 5,
    ///   causing it not to move because it wraps around: 4 3 0 [1] 2. The skip size increases to 2.
    fn sample_step2() {
        let mut hasher = step1();
        hasher.hash(4);
        assert_eq!(hasher, step2())
    }

    #[test]
    /// - The third length, 1, selects a sublist of a single element,
    ///   and so reversing it has no effect.
    /// - The current position moves forward by the length (1) plus the skip size (2):
    ///   4 [3] 0 1 2. The skip size increases to 3.
    fn sample_step3() {
        let mut hasher = step2();
        hasher.hash(1);
        assert_eq!(hasher, step3())
    }

    #[test]
    /// - The fourth length, 5, selects every element starting with the second: 4) ([3] 0 1 2.
    ///   Reversing this sublist (3 0 1 2 4 into 4 2 1 0 3) produces: 3) ([4] 2 1 0.
    /// - Finally, the current position moves forward by 8: 3 4 2 1 [0].
    ///   The skip size increases to 4.
    fn sample_step4() {
        let mut hasher = step3();
        hasher.hash(5);
        assert_eq!(hasher, step4())
    }

    #[test]
    /// The empty string becomes a2582a3a0e66e6e86e3812dcb672a272.
    fn part2_sample1() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272".to_owned())
    }

    #[test]
    /// AoC 2017 becomes 33efeb34ea91902bb2f59c9920caa6cd.
    fn part2_sample2() {
        assert_eq!(
            part2("AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd".to_owned()
        )
    }

    #[test]
    /// 1,2,3 becomes 3efbe78a8d82f29979031a4aa0b16a9d.
    fn part2_sample3() {
        assert_eq!(
            part2("1,2,3"),
            "3efbe78a8d82f29979031a4aa0b16a9d".to_owned()
        )
    }

    #[test]
    /// 1,2,4 becomes 63960835bcdc130f0b66d7ff4f6a5a8e.
    fn part2_sample4() {
        assert_eq!(
            part2("1,2,4"),
            "63960835bcdc130f0b66d7ff4f6a5a8e".to_owned()
        )
    }

    fn step0() -> KnotHasher {
        KnotHasher {
            list: vec![0, 1, 2, 3, 4],
            pos: 0,
            skip: 0,
        }
    }

    fn step1() -> KnotHasher {
        KnotHasher {
            list: vec![2, 1, 0, 3, 4],
            pos: 3,
            skip: 1,
        }
    }

    fn step2() -> KnotHasher {
        KnotHasher {
            list: vec![4, 3, 0, 1, 2],
            pos: 3,
            skip: 2,
        }
    }

    fn step3() -> KnotHasher {
        KnotHasher {
            list: vec![4, 3, 0, 1, 2],
            pos: 1,
            skip: 3,
        }
    }

    fn step4() -> KnotHasher {
        KnotHasher {
            list: vec![3, 4, 2, 1, 0],
            pos: 4,
            skip: 4,
        }
    }
}
