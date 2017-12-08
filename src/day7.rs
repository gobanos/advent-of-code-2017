use std::collections::{HashMap, HashSet};
use day7_parser;

use petgraph::Graph;

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, weight: u32, children: Vec<&'a str>) -> Node<'a> {
        Node {
            name,
            weight,
            children,
        }
    }
}

pub fn part1(input: &str) -> &str {
    let input = day7_parser::parse(input, Node::new);

    find_root(&input)
}

pub fn part2(input: &str) -> u32 {
    let mut input = day7_parser::parse(input, Node::new);


    let root = find_root(&input);

    println!("{:#?}", input.remove(&root[..]).unwrap());

    unimplemented!()
}

fn find_root<'a, 'b>(nodes: &'a HashMap<&'b str, Node<'b>>) -> &'b str {
    let mut names = nodes.keys().collect::<HashSet<_>>();

    for (_, node) in nodes.iter() {
        for child in node.children.iter() {
            names.remove(child);
        }
    }

    assert_eq!(names.len(), 1);

    names.into_iter().next().unwrap()
}
