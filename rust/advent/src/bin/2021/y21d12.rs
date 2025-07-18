use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = FxHashMap<&'a str, Vec<&'a str>>;
type Output = usize;

struct State<'a> {
    visits: Vec<&'a str>,
    visited_twice: bool,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input_str = get_input(21, 12).unwrap();
    let input = parse_input(&input_str);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut edges = Input::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let v = edges.entry(a).or_insert(Vec::new());
        v.push(b);
        if a != "start" && b != "end" {
            let v = edges.entry(b).or_insert(Vec::new());
            v.push(a);
        }
    }
    edges
}

fn find_paths(edges: &Input, allow_one_extra: bool) -> Output {
    let mut count = 0;
    let mut q = VecDeque::new();
    q.push_back(State { visits: vec!["start"], visited_twice: false });
    while let Some(v) = q.pop_front() {
        if *v.visits.last().unwrap() == "end" {
            count += 1;
        } else {
            edges[v.visits.last().unwrap()]
                .iter()
                .filter_map(|edge| {
                    if edge.chars().next().unwrap().is_uppercase() || !v.visits.contains(edge) {
                        let mut new_visits = v.visits.clone();
                        new_visits.push(*edge);
                        Some(State { visits: new_visits, visited_twice: v.visited_twice })
                    } else if allow_one_extra && !v.visited_twice {  
                        let mut new_visits = v.visits.clone();
                        new_visits.push(*edge);
                        Some(State { visits: new_visits, visited_twice: true })
                    } else {
                        None
                    }
                })
                .for_each(|state| q.push_back(state));
        }
    }
    count
}

fn part1(edges: &Input) -> Output {
    find_paths(edges, false)
}

fn part2(edges: &Input) -> Output {
    find_paths(edges, true)
}

#[test]
fn default() {
    let input = get_input(21, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(4104, part1(&input));
    assert_eq!(119760, part2(&input));
}

// Input parsed (26μs)
// 1. 4104 (2ms)
// 2. 119760 (65ms)
// Total: 67ms
