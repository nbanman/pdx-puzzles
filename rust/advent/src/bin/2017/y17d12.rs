use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = IndexMap<usize, Vec<usize>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let mut numbers = line.get_numbers();
            let program = numbers.next().unwrap();
            let connections = numbers.collect_vec();
            (program, connections)
        })
        .collect()
}

fn all_links(start: usize, programs: &Input) -> IndexSet<usize> {
    let mut visited = IndexSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        visited.insert(current);
        for connect in programs.get(&current).unwrap() {
            if !visited.contains(connect) {
                queue.push_back(*connect);
            } 
        }
    }
    visited
}

fn part1(programs: &Input) -> Output {
    all_links(0, programs).len()
}

fn part2(programs: &Input) -> Output {
    let mut program_set: IndexSet<usize> = programs.keys().copied().collect();
    let mut count = 0;
    while !program_set.is_empty() {
        count += 1;
        let to_remove = all_links(*program_set.first().unwrap(), programs);
        program_set = program_set.difference(&to_remove).copied().collect();
    }
    count
}

#[test]
fn default() {
    let input = get_input(17, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(115, part1(&input));
    assert_eq!(221, part2(&input));
}

// Input parsed (546μs)
// 1. 115 (16μs)
// 2. 221 (3ms)
// Total: 3ms
