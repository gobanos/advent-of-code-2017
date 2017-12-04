use std::collections::HashSet;

fn is_valid_passphrase(phrase: &str) -> bool {
    let mut set = HashSet::new();

    for word in phrase.split_whitespace() {
        if !set.insert(word) {
            return false;
        }
    }

    true
}

pub fn part1(input: &str) -> u32 {
    input.lines().filter(|&l| is_valid_passphrase(l)).count() as u32
}

fn is_super_valid_passphrase(phrase: &str) -> bool {
    let mut set = HashSet::new();

    for word in phrase.split_whitespace() {
        let mut chars = word.chars().collect::<Vec<_>>();
        chars.sort();

        if !set.insert(chars) {
            return false;
        }
    }

    true
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|&l| is_super_valid_passphrase(l))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// aa bb cc dd ee is valid.
    fn part1_sample1() {
        assert!(is_valid_passphrase("aa bb cc dd ee"));
    }

    #[test]
    /// aa bb cc dd aa is not valid - the word aa appears more than once.
    fn part1_sample2() {
        assert!(!is_valid_passphrase("aa bb cc dd aa"));
    }

    #[test]
    /// aa bb cc dd aaa is valid - aa and aaa count as different words.
    fn part1_sample3() {
        assert!(is_valid_passphrase("aa bb cc dd aaa"));
    }

    #[test]
    /// abcde fghij is a valid passphrase.
    fn part2_sample1() {
        assert!(is_super_valid_passphrase("abcde fghij"));
    }

    #[test]
    /// abcde xyz ecdab is not valid - the letters from the third word
    /// can be rearranged to form the first word.
    fn part2_sample2() {
        assert!(!is_super_valid_passphrase("abcde xyz ecdab"));
    }

    #[test]
    /// a ab abc abd abf abj is a valid passphrase, because all letters need
    /// to be used when forming another word.
    fn part2_sample3() {
        assert!(is_super_valid_passphrase("a ab abc abd abf abj"));
    }

    #[test]
    /// iiii oiii ooii oooi oooo is valid.
    fn part2_sample4() {
        assert!(is_super_valid_passphrase("iiii oiii ooii oooi oooo"));
    }

    #[test]
    /// oiii ioii iioi iiio is not valid - any of these words can be rearranged
    /// to form any other word.
    fn part2_sample5() {
        assert!(!is_super_valid_passphrase("oiii ioii iioi iiio"));
    }
}
