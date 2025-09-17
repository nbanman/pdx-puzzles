use advent::utilities::get_input::get_input;
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::collections::HashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<Vec<FxHashSet<char>>>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 6).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    // Read input and split into separate groups.
    input.split("\n\n")
        .map(|it| it.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn solve<'a, F: Clone>(groups: &Input, f: F) -> Output
where F: Fn(HashSet<char, FxBuildHasher>, HashSet<char, FxBuildHasher>) -> HashSet<char, FxBuildHasher>
{
    // Both parts involve looking at each group separately, counting the answers in a particular
    // way, then returning the sum of those counts.
    groups.iter()
        .map(|group| group.iter().cloned().reduce(f.clone()).unwrap().len())
        .sum()
}

fn part1(groups: &Input) -> Output {
    // For each group, count the number of questions to which *anyone* answered "yes."
    solve(groups, |a, b| a.union(&b).copied().collect())
}

fn part2(groups: &Input) -> Output {
    // For each group, count the number of questions to which *everyone* answered "yes."
    solve(groups, |a, b| a.intersection(&b).copied().collect())
}

#[test]
fn default() {
    let input = get_input(20, 6).unwrap();
    let input = parse_input(&input);
    assert_eq!(6297, part1(&input));
    assert_eq!(3158, part2(&input));
}

// Input parsed (532μs)
// 1. 6297 (269μs)
// 2. 3158 (221μs)
// Total: 1ms