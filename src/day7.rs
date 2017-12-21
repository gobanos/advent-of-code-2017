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

pub fn part2(input: &str) -> i32 {
    let mut input = day7_parser::parse(input, Node::new);

    let root = find_root(&input);

    let (mut graph, root_index) = build_graph(&mut input, root);

    update_weight(&mut graph, root_index);

    let (diff, node) = find_wrong_weight(&graph, root_index).expect("unable to find wrong weight");

    graph[node].weight as i32 - diff
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

fn build_graph<'a, 'b>(
    input: &'b mut HashMap<&'a str, Node<'a>>,
    root: &'a str,
) -> (Graph<Node<'a>, ()>, NodeIndex) {
    let mut root_node = input.remove(root).expect("root not found");

    let mut graph = Graph::with_capacity(input.len() + 1, input.len());
    let mut nodes = HashMap::new();

    let children = root_node.children.take().expect("no children found");

    nodes.insert(root, graph.add_node(root_node));

    add_children(&mut graph, input, &mut nodes, root, children);

    (graph, nodes[root])
}

fn add_children<'a, 'b>(
    graph: &'b mut Graph<Node<'a>, ()>,
    input: &'b mut HashMap<&'a str, Node<'a>>,
    nodes: &'b mut HashMap<&'a str, NodeIndex>,
    root: &'a str,
    children: Vec<&'a str>,
) {
    for child in children {
        let mut child_node = input.remove(child).expect("root not found");

        let grand_children = child_node.children.take().expect("no children found");

        nodes.insert(child, graph.add_node(child_node));

        graph.add_edge(nodes[root], nodes[child], ());

        add_children(graph, input, nodes, child, grand_children);
    }
}

fn update_weight(graph: &mut Graph<Node, ()>, index: NodeIndex) -> u32 {
    let mut total = graph[index].weight;
    let mut children = graph.neighbors(index).detach();

    while let Some((_, child)) = children.next(graph) {
        total += update_weight(graph, child);
    }

    graph[index].total_weight = total;
    total
}

fn find_wrong_weight(graph: &Graph<Node, ()>, index: NodeIndex) -> Option<(i32, NodeIndex)> {
    let mut weight_a = None;
    let mut weight_b = None;

    for child in graph.neighbors(index) {
        let weight = graph[child].total_weight;

        weight_a = match weight_a.take() {
            None => Some((weight, 1, child)),
            Some((w, i, c)) if w == weight => Some((w, i + 1, c)),
            a => {
                weight_b = match weight_b.take() {
                    None => Some((weight, 1, child)),
                    Some((w, i, c)) if w == weight => Some((w, i + 1, c)),
                    _ => unreachable!(),
                };

                a
            }
        }
    }

    match (weight_a, weight_b) {
        (_, None) => None,
        (Some((wrong_weight, 1, wrong_node)), Some((correct_weight, _, _)))
        | (Some((correct_weight, _, _)), Some((wrong_weight, 1, wrong_node))) => {
            find_wrong_weight(graph, wrong_node)
                .or_else(|| Some((wrong_weight as i32 - correct_weight as i32, wrong_node)))
        }
        _ => unreachable!(),
    }
}
