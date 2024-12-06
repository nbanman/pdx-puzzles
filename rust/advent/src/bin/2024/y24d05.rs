use std::{cmp::Ordering, collections::{HashMap, HashSet}};

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
        rules.entry(r).or_insert(HashSet::new()).insert(l);
        rules.entry(l).or_insert(HashSet::new());
    }
    (updates, rules)
}

fn part1(input: &Input) -> Output {
    let (updates, rules) = input;
    updates.iter()
        .filter(|update| { 
            update.iter().tuple_windows().all(|(a, b)| rules[b].contains(a))
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(input: &Input) -> Output {
    let (updates, rules) = input;
    updates.iter()
        .filter(|update| {
            update.iter().tuple_windows().any(|(a, b)| !rules[b].contains(a))
        })
        .map(|update| {
            update.iter()
                .sorted_by(|&a, &b| {
                    if rules[b].contains(a) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .take(update.len() / 2 + 1)
                .last()
                .unwrap()
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

// Input parsed (392μs)
// 1. 5129 (46μs)
// 2. 4077 (202μs)
// Total: 644μs

