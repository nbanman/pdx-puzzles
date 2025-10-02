use advent::utilities::get_input::get_input;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Node>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Debug)]
struct Node {
    children: Vec<usize>,
    metadata: Vec<usize>,
    metadata_sum: usize,
}

fn parse_input(input: &str) -> Input {
    let mut nodes = Vec::new();
    let mut numbers = input.get_numbers();
    make_nodes(&mut numbers, &mut nodes);
    nodes
}

fn make_nodes(
    numbers: &mut utilities::parsing::get_numbers::NumberIterator<'_, usize>,
    nodes: &mut Vec<Node>,
) -> usize
{
    let child_quantity = numbers.next().unwrap();
    let metadata_quantity = numbers.next().unwrap();
    let children: Vec<usize> = (0..child_quantity)
        .map(|_| make_nodes(numbers, nodes))
        .collect();
    let metadata: Vec<usize> = (0..metadata_quantity)
        .map(|_| numbers.next().unwrap())
        .collect();
    let metadata_sum = metadata.iter().sum();
    let node = Node { children, metadata, metadata_sum };
    nodes.push(node);
    nodes.len() - 1
}

fn get_total_metadata(nodes: &[Node], id: usize) -> usize {
    let node = &nodes[id];
    let mut total_metadata: usize = node.metadata_sum;
    for &child_id in node.children.iter() {
        total_metadata += get_total_metadata(nodes, child_id);
    }
    total_metadata
}

fn get_value(nodes: &[Node], id: usize) -> usize {
    let node = &nodes[id];
    if node.children.is_empty() {
        return node.metadata_sum;
    }
    let mut value = 0;
    for &metadata_index in node.metadata.iter() {
        if (1..=node.children.len()).contains(&metadata_index) {
            value += get_value(nodes, node.children[metadata_index - 1]);
        }
    }
    value
}

fn part1(nodes: &Input) -> Output {
    get_total_metadata(nodes, nodes.len() - 1)
}

fn part2(nodes: &Input) -> Output {
    get_value(nodes, nodes.len() - 1)
}

#[test]
fn default() {
    let input = get_input(18, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(36027, part1(&input));
    assert_eq!(23960, part2(&input));
}

// Input parsed (242μs)
// 1. 36027 (9μs)
// 2. 23960 (9μs)
// Total: 263μs
