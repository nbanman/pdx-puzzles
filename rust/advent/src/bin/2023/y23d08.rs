use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use utilities::{math::formulae::lcm, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = (&'a str, HashMap<String, (String, String)>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (directions, net_str) = input.split_once("\n\n").unwrap();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for line in net_str.lines() {
        let (node, to) = line.split_once(" = (").unwrap();
        let (left, to) = to.split_once(", ").unwrap();
        let (right, _) = to.split_once(')').unwrap();
        network.insert(node.to_string(), (left.to_string(), right.to_string()));
    }
    (directions, network)
}

fn traverse<F>(
    directions: &str,
    network: &HashMap<String, (String, String)>,
    start_node: &str,
    end_condition: F) -> usize
    where F: Fn(&str) -> bool
{
    directions
        .chars()
        .cycle()
        .scan(start_node, |node, dir| {
            let new_node = network
                .get(&node.to_string())
                .map(|(left, right)| if dir == 'L' {
                    left.as_str()
                } else {
                    right.as_str()
                })
                .unwrap();
            *node = new_node;
            Some(new_node)
        })
        .enumerate()
        .find(|(_, node)| {
            end_condition(node)
        })
        .unwrap()
        .0 + 1
}

fn part1(input: &Input) -> Output {
    let (directions, network) = input;
    let end_condition = |end: &str| end == "ZZZ";
    traverse(directions, network, "AAA", end_condition)
}

fn part2(input: &Input) -> Output {
    let (directions, network) = input;
    let end_condition = |end: &str| end.ends_with('Z');
    network.keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| traverse(directions, network, node, end_condition))
        .reduce(|acc, cycle_length| lcm(acc as i64, cycle_length as i64) as usize)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(23, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(19241, part1(&input));
    assert_eq!(9606140307013, part2(&input));
}
