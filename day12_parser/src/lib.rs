#[macro_use]
extern crate nom;

use std::str;
use std::collections::HashMap;

use nom::digit;

named!(
    number<u32>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    children<Vec<u32>>,
    separated_list_complete!(tag!(", "), number)
);

named!(
    line<(u32, Vec<u32>)>,
    do_parse!(n: number >> tag!(" <-> ") >> c: children >> (n, c))
);

pub fn parse(input: &str) -> HashMap<u32, Vec<u32>> {
    input
        .lines()
        .filter_map(|l| line(l.as_bytes()).to_result().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult::Done;

    const EMPTY: &[u8] = &[];

    #[test]
    fn line_sample1() {
        assert_eq!(line("0 <-> 2".as_bytes()), Done(EMPTY, (0, vec![2])))
    }

    #[test]
    fn line_sample2() {
        assert_eq!(
            line("2 <-> 0, 3, 4".as_bytes()),
            Done(EMPTY, (2, vec![0, 3, 4]))
        )
    }
}
