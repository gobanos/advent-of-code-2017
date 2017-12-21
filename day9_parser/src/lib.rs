#[macro_use]
extern crate nom;

use std::str;

use nom::anychar;

pub use Content::{Garbage, Group};

#[derive(Debug, Eq, PartialEq)]
pub enum Content {
    Garbage(u32),
    Group(Vec<Content>),
}

named!(
    garbage<Content>,
    map!(
        delimited!(
            tag!("<"),
            fold_many0!(garbage_bit, 0, |sum, nb| sum + nb),
            tag!(">")
        ),
        Garbage
    )
);

named!(
    garbage_bit<u32>,
    alt!(value!(0, tuple!(tag!("!"), anychar)) | value!(1, tuple!(not!(char!('>')), anychar)))
);

named!(
    group<Content>,
    map!(
        delimited!(
            tag!("{"),
            separated_list_complete!(tag!(","), content),
            tag!("}")
        ),
        Group
    )
);

named!(content<Content>, alt!(group | garbage));

pub fn parse(input: &str) -> Result<Content, nom::ErrorKind> {
    content(input.as_bytes()).to_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::IResult::Done;

    const EMPTY: &[u8] = &[];

    #[test]
    /// <>, empty garbage.
    /// <>, 0 characters.
    fn garbage_sample1() {
        assert_eq!(content("<>".as_bytes()), Done(EMPTY, Garbage(0)));
    }

    #[test]
    /// <random characters>, garbage containing random characters.
    /// <random characters>, 17 characters.
    fn garbage_sample2() {
        assert_eq!(
            content("<random characters>".as_bytes()),
            Done(EMPTY, Garbage(17))
        );
    }

    #[test]
    /// <<<<>, because the extra < are ignored.
    /// <<<<>, 3 characters.
    fn garbage_sample3() {
        assert_eq!(content("<<<<>".as_bytes()), Done(EMPTY, Garbage(3)));
    }

    #[test]
    /// <{!>}>, because the first > is canceled.
    /// <{!>}>, 2 characters.
    fn garbage_sample4() {
        assert_eq!(content("<{!>}>".as_bytes()), Done(EMPTY, Garbage(2)));
    }

    #[test]
    /// <!!>, because the second ! is canceled, allowing the > to terminate the garbage.
    /// <!!>, 0 characters.
    fn garbage_sample5() {
        assert_eq!(content("<!!>".as_bytes()), Done(EMPTY, Garbage(0)));
    }

    #[test]
    /// <!!!>>, because the second ! and the first > are canceled.
    /// <!!!>>, 0 characters.
    fn garbage_sample6() {
        assert_eq!(content("<!!!>>".as_bytes()), Done(EMPTY, Garbage(0)));
    }

    #[test]
    /// <{o"i!a,<{i<a>, which ends at the first >.
    /// <{o"i!a,<{i<a>, 10 characters.
    fn garbage_sample7() {
        assert_eq!(
            content("<{o\"i!a,<{i<a>".as_bytes()),
            Done(EMPTY, Garbage(10))
        );
    }

    #[test]
    /// {}, 1 group.
    fn group_sample1() {
        assert_eq!(content("{}".as_bytes()), Done(EMPTY, Group(vec![])));
    }

    #[test]
    /// {{{}}}, 3 groups.
    fn group_sample2() {
        assert_eq!(
            content("{{{}}}".as_bytes()),
            Done(EMPTY, Group(vec![Group(vec![Group(vec![])])]),)
        );
    }

    #[test]
    /// {{},{}}, also 3 groups.
    fn group_sample3() {
        assert_eq!(
            content("{{},{}}".as_bytes()),
            Done(EMPTY, Group(vec![Group(vec![]), Group(vec![])]))
        );
    }

    #[test]
    /// {{{},{},{{}}}}, 6 groups.
    fn group_sample4() {
        assert_eq!(
            content("{{{},{},{{}}}}".as_bytes()),
            Done(
                EMPTY,
                Group(vec![
                    Group(vec![
                        Group(vec![]),
                        Group(vec![]),
                        Group(vec![Group(vec![])]),
                    ]),
                ])
            )
        );
    }

    #[test]
    /// {<{},{},{{}}>}, 1 group (which itself contains garbage).
    fn group_sample5() {
        assert_eq!(
            content("{<{},{},{{}}>}".as_bytes()),
            Done(EMPTY, Group(vec![Garbage(10)]))
        );
    }

    #[test]
    /// {<a>,<a>,<a>,<a>}, 1 group.
    fn group_sample6() {
        assert_eq!(
            content("{<a>,<a>,<a>,<a>}".as_bytes()),
            Done(
                EMPTY,
                Group(vec![Garbage(1), Garbage(1), Garbage(1), Garbage(1)])
            )
        );
    }

    #[test]
    /// {{<a>},{<a>},{<a>},{<a>}}, 5 groups.
    fn group_sample7() {
        assert_eq!(
            content("{{<a>},{<a>},{<a>},{<a>}}".as_bytes()),
            Done(
                EMPTY,
                Group(vec![
                    Group(vec![Garbage(1)]),
                    Group(vec![Garbage(1)]),
                    Group(vec![Garbage(1)]),
                    Group(vec![Garbage(1)]),
                ])
            )
        );
    }

    #[test]
    /// {{<!>},{<!>},{<!>},{<a>}}, 2 groups (since all but the last > are canceled).
    fn group_sample8() {
        assert_eq!(
            content("{{<!>},{<!>},{<!>},{<a>}}".as_bytes()),
            Done(EMPTY, Group(vec![Group(vec![Garbage(13)])]))
        );
    }
}
