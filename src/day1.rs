fn uncaptcha(input: &str, gap: usize) -> u32 {
    input
        .chars()
        .zip(input.chars().cycle().skip(gap))
        .filter_map(|(a, b)| if a == b { a.to_digit(10) } else { None })
        .sum()
}

pub fn part1(input: &str) -> u32 {
    uncaptcha(input, 1)
}

pub fn part2(input: &str) -> u32 {
    uncaptcha(input, input.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the
    /// second digit and the third digit (2) matches the fourth digit.
    fn part1_sample1() {
        assert_eq!(part1("1122"), 3);
    }

    #[test]
    /// 1111 produces 4 because each digit (all 1) matches the next.
    fn part1_sample2() {
        assert_eq!(part1("1111"), 4);
    }

    #[test]
    /// 1234 produces 0 because no digit matches the next.
    fn part1_sample3() {
        assert_eq!(part1("1234"), 0);
    }

    #[test]
    /// 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
    fn part1_sample4() {
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    /// 1212 produces 6: the list contains 4 items, and all four digits
    /// match the digit 2 items ahead.
    fn part2_sample1() {
        assert_eq!(part2("1212"), 6);
    }

    #[test]
    /// 1221 produces 0, because every comparison is between a 1 and a 2.
    fn part2_sample2() {
        assert_eq!(part2("1221"), 0);
    }

    #[test]
    /// 123425 produces 4, because both 2s match each other, but no other digit has a match.
    fn part2_sample3() {
        assert_eq!(part2("123425"), 4);
    }

    #[test]
    /// 123123 produces 12.
    fn part2_sample4() {
        assert_eq!(part2("123123"), 12);
    }

    #[test]
    /// 12131415 produces 4.
    fn part2_sample5() {
        assert_eq!(part2("12131415"), 4);
    }
}