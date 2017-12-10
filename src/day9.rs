use day9_parser::{parse, Content, Garbage, Group};

pub fn part1(input: &str) -> u32 {
    let content = parse(input).expect("Failed to parse input");

    group_score(&content, 0)
}
pub fn part2(input: &str) -> u32 {
    let content = parse(input).expect("Failed to parse input");

    garbage_length(&content)
}

fn group_score(content: &Content, depth: u32) -> u32 {
    match *content {
        Group(ref sub_content) => {
            sub_content
                .iter()
                .map(|c| group_score(c, depth + 1))
                .sum::<u32>() + depth + 1
        }
        Garbage(_) => 0,
    }
}

fn garbage_length(content: &Content) -> u32 {
    match *content {
        Group(ref sub_content) => sub_content.iter().map(|c| garbage_length(c)).sum(),
        Garbage(len) => len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// {}, score of 1.
    fn part1_sample1() {
        assert_eq!(part1("{}"), 1);
    }

    #[test]
    /// {{{}}}, score of 1 + 2 + 3 = 6.
    fn part1_sample2() {
        assert_eq!(part1("{{{}}}"), 6);
    }

    #[test]
    /// {{},{}}, score of 1 + 2 + 2 = 5.
    fn part1_sample3() {
        assert_eq!(part1("{{},{}}"), 5);
    }

    #[test]
    /// {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
    fn part1_sample4() {
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
    }

    #[test]
    /// {<a>,<a>,<a>,<a>}, score of 1.
    fn part1_sample5() {
        assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
    }

    #[test]
    /// {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    fn part1_sample6() {
        assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }

    #[test]
    /// {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    fn part1_sample7() {
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }

    #[test]
    /// {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
    fn part1_sample8() {
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }


}
