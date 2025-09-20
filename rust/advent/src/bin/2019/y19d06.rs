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
    // intermediate collection, mapping planet names with a Vec of child planet names
    let mut child_registry: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (parent, name) = line.split_once(')').unwrap();
        child_registry.entry(&parent)
            .or_insert(Vec::new())
            .push(name);
    }

    // from child_registry map, recursively develop the paths map as input
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();
    develop_paths("COM", Vec::new(), &mut paths, &child_registry);
    paths
}


fn develop_paths<'a>(
    planet: &'a str,
    mut current_path: Vec<&'a str>,
    paths: &mut HashMap<&'a str, Vec<&'a str>>,
    child_registry: &HashMap<&'a str, Vec<&'a str>>,
) {
    // populates paths map with the path for the particular planet
    paths.insert(planet, current_path.clone());

    // base case, stop if the child_registry shows no children
    if let Some(children) = child_registry.get(planet) {
        // otherwise update the path with planet and recurse with the children
        current_path.push(planet);
        for child in children {
            develop_paths(child, current_path.clone(), paths, child_registry);
        }
    };
}

fn part1(paths: &Input) -> Output {
    paths.values().map(|path| path.len()).sum()
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