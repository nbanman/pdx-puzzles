use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Int = usize;
type PropagationRules<'a> = HashMap<&'a [u8], &'a [u8]>;
type ProteinPairs<'a> = HashMap<&'a [u8], Int>;
type EdgeProteins = Vec<u8>;
type Input<'a> = (PropagationRules<'a>, ProteinPairs<'a>, EdgeProteins);
type Output = Int;

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
    let mut protein_pairs = ProteinPairs::new();
    for i in 0..template.len() - 1 {
        protein_pairs.entry(&template[i ..= i + 1])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let mut edge_proteins = EdgeProteins::new();
    edge_proteins.push(template[0]);
    edge_proteins.push(template[template.len() - 1]);

    let mut rules = PropagationRules::new();
    rule_str.into_iter()
        .filter(|c| c.is_ascii_alphabetic())
        .tuple_windows()
    ()
}

fn part1(input: &Input) -> Output {
    todo!()
}

fn part2(input: &Input) -> Output {
    todo!()
}

#[test]
fn default() {
    let input = get_input(21, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(3555, part1(&input));
    assert_eq!(4439442043739, part2(&input));
}
