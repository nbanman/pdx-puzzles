use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = HashMap<String, CelestialBody>;
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

struct CelestialBody {
    name: String,
    parent: Option<String>,
    orbits: Option<usize>,
}
fn parse_input(input: &str) -> Input {
    let com_name = "COM".to_string();
    let com = (com_name.clone(), CelestialBody { name: com_name, parent: None, orbits: Some(0)})
    input.lines()
        .map(|line| {
            let (name, parent) = line.split_once(')').unwrap()
            let name = name.to_string();
            let parent = Some(parent.to_string());
            (name.clone(), CelestialBody { name, parent, orbits: None })
        })
        .chain(std::iter::once(com))
        .collect()
}

fn part1(input: &Input) -> Output {

    todo!()
}

fn part2(input: &Input) -> Output {

    todo!()
}

#[test]
fn default() {
    let input = get_input(19, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(315757, part1(&input));
    // assert_eq!(481, part2(&input));
}

