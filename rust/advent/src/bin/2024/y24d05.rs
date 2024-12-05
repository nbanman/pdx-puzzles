use std::collections::{HashMap, HashSet};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::get_numbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Rules = HashMap<usize, HashSet<usize>>;
type Input = (Vec<Vec<usize>>, Rules);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (rules_build, updates) = input.split("\n\n")
        .map(|stanza| {
            stanza.lines().map(|line| get_numbers::<usize>(line)).collect::<Vec<_>>() 
        })
        .collect_tuple()
        .unwrap();
    let mut rules: Rules = Rules::new();
    for rule in rules_build {
        let [l, r] = rule[..2] else { panic!("Invalid rule") };
        if !rules.contains_key(&r) {
            rules.insert(r, HashSet::new());
        }
        rules.entry(r)
            .and_modify(|it| { it.insert(l); } );
        rules.entry(l).or_insert(HashSet::new());
    }
    (updates, rules)
}

fn sort_update(update: &HashSet<usize>, rules: &Rules) -> Vec<usize> {
    rules.iter()
        .filter_map(|(page, on_left)| {
            if update.contains(page) {
                Some((*page, on_left.intersection(update).count()))
            } else {
                None
            }
        })
        .sorted_by_key(|(_, pages_on_left)| *pages_on_left)
        .map(|(page, _)| page)
        .collect()
}

fn part1(input: &Input) -> Output {
    let (updates, rules) = input;
    updates.iter()
        .map(|update| {
            let update_set: HashSet<usize> = update.iter().copied().collect();
            let sorted = sort_update(&update_set, rules);
            if *update == sorted {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &Input) -> Output {
    let (updates, rules) = input;
    updates.iter()
        .map(|update| {
            let update_set: HashSet<usize> = update.iter().copied().collect();
            let sorted = sort_update(&update_set, rules);
            if *update != sorted {
                sorted[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(24, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(5129, part1(&input));
    assert_eq!(4077, part2(&input));
}

// Input parsed (384μs)
// 1. 5129 (982μs)
// 2. 4077 (952μs)
// Total: 2ms
