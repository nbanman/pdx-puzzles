use std::iter::successors;

use itertools::Itertools;
use rustc_hash::FxHashMap;
use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type PropagationRules = [u16; 256];
type ProteinPairs = [u64; 256];
type EdgeProteins = (u8, u8);
type Input = (PropagationRules, ProteinPairs, EdgeProteins);
type Output = u64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (template, rule_str) = input.split_once("\n\n").unwrap();
    let template = template.as_bytes();
    let rule_str = rule_str.as_bytes();

    let mut store = FxHashMap::default();
    
    let mut protein_pairs = [0u64; 256];

    template.iter()
        .map(|&b| {
            let next = store.len() as u8;
            *store.entry(b).or_insert(next)
        })
        .tuple_windows()
        .for_each(|(a, b)| { protein_pairs[((a as usize) << 4) | b as usize] += 1; });

    let edge_proteins = {
        let next = store.len() as u8;
        let a = *store.entry(template[0]).or_insert(next);
        let next = store.len() as u8;
        let b = *store.entry(*template.last().unwrap()).or_insert(next);
        (a, b)
    };

    let mut rules = [0u16; 256];

    rule_str.iter()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|&b| {
            let next = store.len() as u8;
            *store.entry(b).or_insert(next)
        })
        .tuples()
        .for_each(|(a, b, c)| {
            let aa = (a as u16) << 4;
            let ac = aa | (c as u16);
            let cb = ((c as u16) << 4) | b as u16;
            rules[aa as usize | b as usize] = (ac << 8) | cb;
        });

    (rules, protein_pairs, edge_proteins)
}

fn solve(input: &Input, steps: usize) -> Output {
    let (rules, protein_pairs, edge_proteins) = input;
    let polymerized = successors(Some(protein_pairs.clone()), |prev| {
        let mut next = [0u64; 256];
        for (proteins, &amt) in prev.iter().enumerate() {
            let code = rules[proteins];
            let a = (code >> 8) as usize;
            let b = (code & 255) as usize;
            next[a] += amt;
            next[b] += amt;
        }
        Some(next)
    })
        .take(steps + 1)
        .last()
        .unwrap();
    let protein_count: [u64; 10] = count_proteins(&polymerized, edge_proteins);
    let (min, max) = protein_count.into_iter().minmax().into_option().unwrap();
    max - min
}

fn count_proteins(polymerized: &[u64], edge_proteins: &(u8, u8)) -> [u64; 10] {
    let mut counts = [0u64; 10];
    for (proteins, &amt) in polymerized.iter().enumerate() {
        if amt > 0 {
            let a = proteins >> 4;
            let b = proteins & 15;
            counts[a] += amt;
            counts[b] += amt;
        }
    }
    // Proteins at the very beginning and end are not double-counted, so bump the count for these to make
    // it consistent.
    counts[edge_proteins.0 as usize] += 1;
    counts[edge_proteins.1 as usize] += 1;
    for count in counts.iter_mut() {
        *count /= 2;
    }
    counts
}

fn part1(input: &Input) -> Output {
    solve(input, 10)
}

fn part2(input: &Input) -> Output {
    solve(input, 40)
}

#[test]
fn default() {
    let input = get_input(21, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(3555, part1(&input));
    assert_eq!(4439442043739, part2(&input));
}

// Input parsed (22μs)
// 1. 3555 (12μs)
// 2. 4439442043739 (19μs)
// Total: 56μs