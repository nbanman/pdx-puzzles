use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 23).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &'_ str) -> Input<'_> {
    let mut lan = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        lan.entry(a).or_insert(HashSet::new()).insert(b);
        lan.entry(b).or_insert(HashSet::new()).insert(a);
    }
    lan
}

fn part1(lan: &Input) -> usize {
    lan.iter()
        .filter(|(a, _)| a.starts_with('t'))
        .flat_map(|(&a, bs)| {
            let mut triads: Vec<Vec<&str>> = Vec::new();
            let bs: Vec<_> = bs.iter().collect();
            for b_idx in 0..bs.len() - 1 {
                let b = *bs[b_idx];
                for &&c in bs[b_idx + 1..].iter() {
                    if lan[&b].contains(&c) {
                        let triad: Vec<_> = vec![a, b, c].into_iter().sorted_unstable().collect();
                        triads.push(triad);
                    }
                }
            }
            triads
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part2(lan: &Input) -> String {
    lan.keys()
        .map(|&pc| {
            let mut connections = lan[&pc].clone();
            connections.insert(pc);
            connections.iter()
                .map(|&next_pc| {
                    let intersect: HashSet<&str> = lan[&next_pc]
                        .intersection(&connections)
                        .chain(std::iter::once(&next_pc))
                        .cloned()
                        .collect();
                    (next_pc, intersect)
                })
                .fold(connections.clone(), |mut acc, (next_pc, intersect)| {
                    let trial: HashSet<_> = acc.intersection(&intersect).cloned().collect();
                    if trial.len() >= 13 {
                        trial
                    } else {
                        acc.remove(&next_pc);
                        acc
                    }
                })
        })
        .max_by_key(|it| it.len())
        .expect("Not empty")
        .into_iter()
        .sorted_unstable()
        .join(",")
}

#[test]
fn default() {
    let input = get_input(24, 23).unwrap();
    let input = parse_input(&input);
    assert_eq!(1253, part1(&input));
    assert_eq!("ag,bt,cq,da,hp,hs,mi,pa,qd,qe,qi,ri,uq".to_string(), part2(&input));
}

// Input parsed (591μs)
// 1. 1253 (326μs)
// 2. ag,bt,cq,da,hp,hs,mi,pa,qd,qe,qi,ri,uq (8ms)
// Total: 9ms