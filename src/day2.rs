fn line_min_max(line: &str) -> Option<(u32, u32)> {
    line.split_whitespace()
        .filter_map(|number| number.parse::<u32>().ok())
        .fold(None, |min_max, number| {
            if let Some((min, max)) = min_max {
                Some((u32::min(min, number), u32::max(max, number)))
            } else {
                Some((number, number))
            }
        })
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(line_min_max)
        .map(|(min, max)| max - min)
        .sum()
}

fn line_evenly_divisors(line: &str) -> Option<(u32, u32)> {
    let mut line: Vec<u32> = line.split_whitespace()
        .filter_map(|number| number.parse::<u32>().ok())
        .collect();

    line.sort();

    for (i, &divisor) in line.iter().enumerate() {
        for &number in line[i + 1..].iter() {
            if number % divisor == 0 {
                return Some((divisor, number));
            }
        }
    }

    None
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(line_evenly_divisors)
        .map(|(divisor, dividend)| dividend / divisor)
        .sum()
}

#[cfg(test)]
mod tests_part1 {
    //! For example, given the following spreadsheet:
    //! 5 1 9 5
    //! 7 5 3
    //! 2 4 6 8

    use super::*;

    #[test]
    /// The first row's largest and smallest values are 9 and 1, and their difference is 8.
    fn line_min_max_sample1() {
        assert_eq!(line_min_max("5 1 9 5"), Some((1, 9)));
    }

    #[test]
    /// The second row's largest and smallest values are 7 and 3, and their difference is 4.
    fn line_min_max_sample2() {
        assert_eq!(line_min_max("7 5 3"), Some((3, 7)));
    }

    #[test]
    /// The third row's difference is 6.
    fn line_min_max_sample3() {
        assert_eq!(line_min_max("2 4 6 8"), Some((2, 8)));
    }

    #[test]
    /// In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
    fn part1_sample() {
        assert_eq!(part1("5 1 9 5\n7 5 3\n2 4 6 8"), 18);
    }
}

#[cfg(test)]
mod tests_part2 {
    //! For example, given the following spreadsheet:
    //! 5 9 2 8
    //! 9 4 7 3
    //! 3 8 6 5

    use super::*;

    #[test]
    /// In the first row, the only two numbers that evenly divide are 8 and 2;
    /// the result of this division is 4.
    fn line_evenly_divisors_sample1() {
        assert_eq!(line_evenly_divisors("5 9 2 8"), Some((2, 8)));
    }

    #[test]
    /// In the second row, the two numbers are 9 and 3; the result is 3.
    fn line_evenly_divisors_sample2() {
        assert_eq!(line_evenly_divisors("9 4 7 3"), Some((3, 9)));
    }

    #[test]
    /// In the third row, the result is 2.
    fn line_evenly_divisors_sample3() {
        assert_eq!(line_evenly_divisors("3 8 6 5"), Some((3, 6)));
    }

    #[test]
    /// In this example, the sum of the results would be 4 + 3 + 2 = 9.
    fn part2_sample() {
        assert_eq!(part2("5 9 2 8\n9 4 7 3\n3 8 6 5"), 9);
    }
}
