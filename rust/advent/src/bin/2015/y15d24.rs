use std::ops::RangeInclusive;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<i64>;
type Output = i64;

#[derive(Debug, Clone)]
struct Package {
    weight: i64,
    range: RangeInclusive<i64>
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines()
        .map(|line| line.parse().unwrap())
        .sorted_unstable()
        .rev()
        .collect()
}

fn solve(weights: &Input, groups: i64) -> Output {
    let group_weight: i64 = weights.iter().sum::<i64>() / groups;
    let packages: Vec<Package> = weights.iter().enumerate()
        .map(|(i, &weight)| {
            let low_end = group_weight - &weights[i..].iter().sum();
            let high_end = group_weight - weight;
            Package { weight, range: low_end..=high_end }
        })
        .collect();

    let mut combos: Vec<Vec<i64>> = packages.iter()
        .filter(|package| package.range.contains(&0))
        .map(|package| vec![package.weight])
        .collect();
    let mut valid_combos = combos.iter()
        .filter(|&combo| combo.iter().sum::<i64>() == group_weight)
        .cloned()
        .collect_vec();

    while !combos.is_empty() {
        let mut new_combos = Vec::new();
        let last_first = *combos[0].last().unwrap();
        let mut i = 1 + packages.iter()
            .position(|package| package.weight == last_first)
            .unwrap();
        'middle:while i < packages.len() {
            for combo in combos.iter() {
                let package = packages.get(i).unwrap();
                if *combo.last().unwrap() <= package.weight {
                    i += 1;
                    continue 'middle;
                }
                if package.range.contains(&combo.iter().sum()) {
                    let latest_combo = combo.iter()
                        .copied()
                        .chain(std::iter::once(package.weight))
                        .collect_vec();
                    if latest_combo.iter().sum::<i64>() == group_weight {
                        valid_combos.push(latest_combo);
                    } else {
                        new_combos.push(latest_combo);
                    }
                }
            }
            i += 1;
        }
        std::mem::swap(&mut combos, &mut new_combos);
        if valid_combos.len() >= groups as usize - 1 {
            valid_combos.sort_unstable_by_key(|it| {
                it.iter().fold(1, |acc, &w| acc * w)
            });
            let taken: FxHashSet<i64> = valid_combos[0].iter().copied().collect();
            for combo in valid_combos.iter() {
                if combo.iter().all(|weight| !taken.contains(weight)) {
                    return taken.into_iter().fold(1, |acc, w| acc * w)
                }
            }
        }
    }
    unreachable!()
}

fn part1(input: &Input) -> Output {
    solve(input, 3)
}

fn part2(input: &Input) -> Output {
    solve(input, 4)
}

#[test]
fn default() {
    let input = get_input(15, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(11846773891, part1(&input));
    assert_eq!(80393059, part2(&input));
}

// Input parsed (17μs)
// 1. 11846773891 (23ms)
// 2. 80393059 (1ms)
// Total: 25ms