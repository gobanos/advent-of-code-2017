#[derive(Debug)]
struct Generator {
    factor: u64,
    previous_value: u64,
}

impl Generator {
    fn new(factor: u64, initial: u64) -> Generator {
        Generator {
            factor,
            previous_value: initial,
        }
    }
}

impl Iterator for Generator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous_value = self.previous_value * self.factor % 2_147_483_647;

        Some(self.previous_value as u16)
    }
}

pub fn part1(a: u64, b: u64) -> usize {
    let gen_a = Generator::new(16_807, a);
    let gen_b = Generator::new(48_271, b);

    gen_a
        .zip(gen_b)
        .take(40_000_000)
        .filter(|&(a, b)| a == b)
        .count()
}

pub fn part2(a: u64, b: u64) -> usize {
    let gen_a = Generator::new(16_807, a);
    let gen_b = Generator::new(48_271, b);

    gen_a
        .filter(|&a| a % 4 == 0)
        .zip(gen_b.filter(|&b| b % 8 == 0))
        .take(5_000_000)
        .filter(|&(a, b)| a == b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(65, 8_921), 588);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(65, 8_921), 309);
    }
}
