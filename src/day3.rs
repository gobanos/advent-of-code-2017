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

    generator.find(|r| r.contains(number)).expect(&format!(
        "Failed to find ring for number {}",
        number
    ))
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
    if a > b { a - b } else { b - a }
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
}
