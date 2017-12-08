use std::collections::{HashMap, HashSet};
use day7_parser;

use petgraph::Graph;
use petgraph::graph::NodeIndex;

#[derive(Debug, Eq, PartialEq)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    total_weight: u32,
    children: Option<Vec<&'a str>>,
}

impl<'a> Node<'a> {
    fn new(name: &'a str, weight: u32, children: Vec<&'a str>) -> Node<'a> {
        Node {
            name,
            weight,
            total_weight: weight,
            children: Some(children),
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
    let mut root_node = input.remove(root).expect("root not found");

    let mut graph = Graph::with_capacity(input.len() + 1, input.len());
    let mut nodes = HashMap::new();

    let children = root_node.children.take().expect("no children found");

    nodes.insert(root, graph.add_node(root_node));

    add_children(&mut graph, &mut input, &mut nodes, root, children);

    println!("{:#?}", graph);
    println!("{:#?}", input);

    unimplemented!()
}

fn add_children<'a, 'b>(graph: &'b mut Graph<Node<'a>, ()>, input: &'b mut HashMap<&'a str, Node<'a>>, nodes: &'b mut HashMap<&'a str, NodeIndex>, root: &'a str, children: Vec<&'a str>) {
    for child in children {
        let mut child_node = input.remove(child).expect("root not found");

        let grand_children = child_node.children.take().expect("no children found");

        nodes.insert(child, graph.add_node(child_node));

        graph.add_edge(nodes[root], nodes[child], ());

        add_children(graph, input, nodes, child, grand_children);
    }
}

fn find_root<'a, 'b>(nodes: &'a HashMap<&'b str, Node<'b>>) -> &'b str {
    let mut names = nodes.keys().collect::<HashSet<_>>();

    for (_, node) in nodes.iter() {
        for child in node.children.as_ref().unwrap() {
            names.remove(child);
        }
    }

    assert_eq!(names.len(), 1);

    names.into_iter().next().unwrap()
}
