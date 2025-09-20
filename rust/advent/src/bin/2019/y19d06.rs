use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = HashMap<&'a str, Vec<&'a str>>;
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

fn parse_input(input: &'_ str) -> Input<'_> {
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (parent, name) = line.split_once(')').unwrap();
        children.entry(&parent)
            .or_insert(Vec::new())
            .push(name);
    }
    let &com = children.keys().find(|&&k| k == "COM").unwrap();
    develop_paths(com, Vec::new(), &mut paths, &children);
    paths
}


fn develop_paths<'a>(
    planet: &'a str,
    prev: Vec<&'a str>,
    paths: &mut HashMap<&'a str, Vec<&'a str>>,
    register: &HashMap<&'a str, Vec<&'a str>>,
) {
    let mut next = prev.clone();
    next.push(planet);
    paths.insert(planet, prev);
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

// Input parsed (3ms)
// 1. 315757 (9μs)
// 2. 481 (4μs)
// Total: 3ms