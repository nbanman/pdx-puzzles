use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = HashMap<String, Vec<String>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut children: HashMap<String, Vec<String>> = HashMap::new();
    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (parent, name) = line.split(')')
            .map(|s| s.to_string())
            .collect_tuple()
            .unwrap();
        children.entry(parent.clone())
            .or_insert(Vec::new())
            .push(name.clone());
    }
    develop_paths("COM", Vec::new(), &mut paths, &children);
    paths
}


fn develop_paths(
    planet: &str,
    prev: Vec<String>,
    paths: &mut HashMap<String, Vec<String>>,
    register: &HashMap<String, Vec<String>>,
) {
    let mut next = prev.clone();
    let planet_string = planet.to_string();
    next.push(planet_string.clone());
    paths.insert(planet_string, prev);
    let Some(children) = register.get(planet) else { return; };
    for child in children {
        develop_paths(child, next.clone(), paths, register);
    }
}

fn part1(paths: &Input) -> Output {
    paths.values().map(|x| x.len()).sum()
}

fn part2(paths: &Input) -> Output {
    let me = paths.get("YOU").unwrap();
    let santa = paths.get("SAN").unwrap();
    let shared_size = (0..me.len()).find(|&i| me[i] != santa[i]).unwrap();
    me.len() + santa.len() - shared_size * 2
}

#[test]
fn default() {
    let input = get_input(19, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(315757, part1(&input));
    assert_eq!(481, part2(&input));
}

// Input parsed (21ms)
// 1. 315757 (10μs)
// 2. 481 (2μs)
// Total: 21ms