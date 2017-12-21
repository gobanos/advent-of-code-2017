struct SpiralGenerator {
    index: u32,
    previous_last: u32,
}

impl SpiralGenerator {
    fn new() -> SpiralGenerator {
        SpiralGenerator {
            index: 0,
            previous_last: 0,
        }
    }
}

impl Iterator for SpiralGenerator {
    type Item = Ring;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        let first = self.previous_last + 1;
        let last = u32::max(first, self.previous_last + 8 * index);

        self.previous_last = last;
        self.index += 1;

        Some(Ring { index, first, last })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Ring {
    pub index: u32,
    pub first: u32,
    pub last: u32,
}

impl Ring {
    fn contains(&self, number: u32) -> bool {
        number >= self.first && number <= self.last
    }

    fn group(&self, number: u32) -> u32 {
        assert!(self.contains(number));
        assert!(self.index > 0);

        let number = number - self.first;

        number / (2 * self.index)
    }

    fn pivot(&self, number: u32) -> u32 {
        let group = self.group(number);

        self.first + (self.index - 1) + 2 * self.index * group
    }
}

fn find_ring(number: u32) -> Ring {
    assert!(number > 0);

    let mut generator = SpiralGenerator::new();

    generator
        .find(|r| r.contains(number))
        .expect(&format!("Failed to find ring for number {}", number))
}

pub fn part1(input: u32) -> u32 {
    assert!(input > 0);

    if input == 1 {
        0
    } else {
        let ring = find_ring(input);

        let pivot = ring.pivot(input);

        ring.index + diff(input, pivot)
    }
}

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

struct SumRing {
    previous: Option<Box<SumRing>>,
    ring: Ring,
    values: Vec<u32>,
}

impl SumRing {
    fn new(mut previous: SumRing, ring: Ring) -> SumRing {
        previous.previous = None;

        let capacity = 8 * ring.index as usize;

        SumRing {
            previous: Some(Box::new(previous)),
            ring,
            values: vec![0; capacity],
        }
    }

    fn get(&self, group: usize, index: usize) -> Option<u32> {
        assert!(group < 4);

        let ring_index = self.ring.index as usize;

        if index < 2 * ring_index {
            let group_index = 2 * ring_index * group;

            Some(self.values[group_index + index])
        } else {
            None
        }
    }

    fn get_last(&self, group: usize) -> u32 {
        assert!(group < 4);

        let next_group_index = 2 * self.ring.index as usize * (group + 1);

        self.values[next_group_index - 1]
    }

    fn compute(&mut self, x: usize) -> u32 {
        let mut sum = 0;

        let n = self.ring.index as usize;
        let group_len = 2 * n;

        let group = x / group_len;
        let group_index = x % group_len;

        // add previous value
        if let Some(prev) = x.checked_sub(1) {
            sum += self.values[prev];
        }

        // if it's the first value of a group,
        // add a value before
        if group_index == 0 {
            if let Some(prev) = x.checked_sub(2) {
                sum += self.values[prev];
            }
        }

        // if it's one of the two last values of a ring,
        // add the first value from the ring
        if 8 * n - x <= 2 {
            sum += self.values[0];
        }

        if let Some(ref previous) = self.previous {
            for offset in (0..3).rev() {
                if let Some(i) = group_index.checked_sub(offset) {
                    if let Some(val) = previous.get(group, i) {
                        sum += val;
                    }
                }
            }

            if group_index < 2 {
                let previous_group = group.checked_sub(1).unwrap_or(3);

                sum += previous.get_last(previous_group);
            }
        }

        self.values[x] = sum;

        sum
    }

    fn capacity(&self) -> usize {
        8 * self.ring.index as usize
    }
}

fn first_sum() -> SumRing {
    SumRing {
        previous: None,
        ring: Ring {
            index: 1,
            first: 2,
            last: 9,
        },
        values: vec![1, 2, 4, 5, 10, 11, 23, 25],
    }
}

pub fn part2(input: u32) -> u32 {
    assert!(input > 25);

    let generator = SpiralGenerator::new();

    let mut previous_sum = Some(first_sum());

    for ring in generator.skip(2) {
        let sum = {
            let mut new_sum = SumRing::new(previous_sum.take().unwrap(), ring);

            for x in 0..new_sum.capacity() {
                let value = new_sum.compute(x);
                if value > input {
                    return value;
                }
            }

            new_sum
        };

        previous_sum = Some(sum);
    }

    panic!("No value found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_RING: Ring = Ring {
        index: 0,
        first: 1,
        last: 1,
    };

    const SECOND_RING: Ring = Ring {
        index: 1,
        first: 2,
        last: 9,
    };

    const THIRD_RING: Ring = Ring {
        index: 2,
        first: 10,
        last: 25,
    };

    #[test]
    fn first_rings() {
        let mut generator = SpiralGenerator::new();

        // First ring: only 1
        assert_eq!(generator.next(), Some(FIRST_RING));

        // Second ring: 2 - 9
        assert_eq!(generator.next(), Some(SECOND_RING));

        // Third ring: 10 - 25
        assert_eq!(generator.next(), Some(THIRD_RING));
    }

    #[test]
    fn find_rings() {
        assert_eq!(find_ring(1), FIRST_RING);
        assert_eq!(find_ring(12), THIRD_RING);
        assert_eq!(find_ring(23), THIRD_RING);
    }

    #[test]
    fn groups() {
        assert_eq!(THIRD_RING.group(12), 0);
        assert_eq!(THIRD_RING.group(23), 3);
    }

    #[test]
    fn pivots() {
        assert_eq!(THIRD_RING.pivot(12), 11);
        assert_eq!(THIRD_RING.pivot(23), 23);
    }

    #[test]
    /// Data from square 1 is carried 0 steps, since it's at the access port.
    fn part1_sample1() {
        assert_eq!(part1(1), 0);
    }

    #[test]
    /// Data from square 12 is carried 3 steps, such as: down, left, left.
    fn part1_sample2() {
        assert_eq!(part1(12), 3);
    }

    #[test]
    /// Data from square 23 is carried only 2 steps: up twice.
    fn part1_sample3() {
        assert_eq!(part1(23), 2);
    }

    #[test]
    /// Data from square 1024 must be carried 31 steps.
    fn part1_sample4() {
        assert_eq!(part1(1024), 31);
    }

    #[test]
    fn second_sum() {
        let first_sum = first_sum();

        let mut second_sum = SumRing::new(first_sum, THIRD_RING);

        assert_eq!(second_sum.compute(0), 26);
        assert_eq!(second_sum.compute(1), 54);
        assert_eq!(second_sum.compute(2), 57);
        assert_eq!(second_sum.compute(3), 59);
        assert_eq!(second_sum.compute(4), 122);
        assert_eq!(second_sum.compute(5), 133);
        assert_eq!(second_sum.compute(6), 142);
        assert_eq!(second_sum.compute(7), 147);
        assert_eq!(second_sum.compute(8), 304);
        assert_eq!(second_sum.compute(9), 330);
        assert_eq!(second_sum.compute(10), 351);
        assert_eq!(second_sum.compute(11), 362);
        assert_eq!(second_sum.compute(12), 747);
        assert_eq!(second_sum.compute(13), 806);
    }
}
