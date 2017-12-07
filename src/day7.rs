use std::collections::HashMap;
use std::str;

use nom::{alphanumeric, space};

named!(name<&str>, map_res!(alphanumeric, str::from_utf8));

named!(weight<u32>, map_res!(
    map_res!(
        delimited!(
            char!('('),
            is_not!(")"),
            char!(')')
        ),
        str::from_utf8
    ),
    str::parse
));

named!(child_sep, complete!(tag!(" -> ")));

named!(children<Vec<&str>>, separated_list_complete!(tag!(", "), name));

named!(line<Node>, do_parse!(
    n: name         >>
    opt!(space)     >>
    w: weight       >>
    opt!(child_sep) >>
    c: children     >>

    (Node::new(n, w, c))
));

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: HashMap<&'a str, Option<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, weight: u32, children: Vec<&'a str>) -> Node<'a> {
        Node {
            name,
            weight,
            children: children.into_iter().map(|child| (child, None)).collect(),
        }
    }
}

#[cfg(test)]
mod tests_parser {
    use super::*;

    use nom::IResult::Done;

    #[test]
    fn name_sample() {
        assert_eq!(name(&b"pbga (66)"[..]), Done(&b" (66)"[..], "pbga"));
    }

    #[test]
    fn weight_sample() {
        assert_eq!(weight(&b"(66)"[..]), Done(&b""[..], 66u32));
    }

    #[test]
    fn children_sample() {
        assert_eq!(children(&b"ktlj, cntj, xhth"[..]), Done(&b""[..], vec!["ktlj", "cntj", "xhth"]));
    }

    #[test]
    fn no_children_sample() {
        assert_eq!(children(&b""[..]), Done(&b""[..], vec![]));
    }

    #[test]
    fn line_with_children() {
        assert_eq!(line(&b"fwft (72) -> ktlj, cntj, xhth"[..]), Done(&b""[..], Node::new("fwft", 72, vec!["ktlj", "cntj", "xhth"])));
    }

    #[test]
    fn line_without_children() {
        assert_eq!(line(&b"pbga (66)"[..]), Done(&b""[..], Node::new("pbga", 66, vec![])));
    }
}
