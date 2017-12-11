#[derive(Debug, Eq, PartialEq)]
struct KnotHasher {
    list: Vec<u8>,
    pos: usize,
    skip: usize,
}

impl KnotHasher {
    fn new(size: u32) -> KnotHasher {
        KnotHasher {
            list: (0..size).map(|n| n as u8).collect(),
            pos: 0,
            skip: 0,
        }
    }

    fn hash(&mut self, length: u8) {
        let length = length as usize;
        let len = self.list.len();

        let mut section = self.list
            .iter()
            .cloned()
            .cycle()
            .skip(self.pos)
            .take(length)
            .collect::<Vec<_>>();
        section.reverse();

        for i in 0..length {
            self.list[(self.pos + i) % len] = section[i];
        }

        self.pos = (self.pos + length + self.skip) % len;

        self.skip += 1;
    }

    fn dense_hash(&self) -> String {
        let mut hash = Vec::with_capacity(16);

        for i in 0..16 {
            let mut xor = None;
            for j in 0..16 {
                let val = self.list[i * 16 + j];
                xor = if let Some(xor) = xor.take() {
                    Some(xor ^ val)
                } else {
                    Some(val)
                }
            }

            hash.push(xor.expect("failed to compute XOR"));
        }

        hash.iter().map(|xor| format!("{:02x}", xor)).collect()
    }
}

pub fn part1(input: &str) -> u32 {
    let mut hasher = KnotHasher::new(256);

    input
        .split(',')
        .filter_map(|number| number.parse::<u8>().ok())
        .for_each(|len| hasher.hash(len));

    u32::from(hasher.list[0]) * u32::from(hasher.list[1])
}

pub fn part2(input: &str) -> String {
    let mut input = input.chars().map(|c| c as u8).collect::<Vec<_>>();
    input.push(17);
    input.push(31);
    input.push(73);
    input.push(47);
    input.push(23);

    let mut hasher = KnotHasher::new(256);

    for _ in 0..64 {
        for &length in &input {
            hasher.hash(length)
        }
    }

    hasher.dense_hash()
}

#[cfg(test)]
mod tests {
    use super::*;

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
