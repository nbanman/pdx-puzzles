use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = (&'a str, &'a str, Vec<Rule<'a>>);
type Output = usize;

struct Rule<'a> {
    element: &'a str,
    replacement: &'a str,
}

impl<'a> Rule<'a> {
    fn len(&self) -> usize {
        self.replacement.chars()
            .filter(|c| c.is_ascii_uppercase())
            .count()
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input<'_> {
    let (rules, molecule) = input.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules.lines()
        .map(|line| {
            let (element, replacement) = line.split_once(" => ").unwrap();
            Rule { element, replacement }
        })
        .sorted_unstable_by_key(|it| it.len())
        .rev()
        .collect();
    (input, molecule, rules)
}

fn part1(input: &Input) -> Output {
    let (_, molecule, rules) = input;
    regex!("[A-Z][a-z]?")
        .find_iter(molecule)
        .flat_map(|mr| {
            rules.iter()
                .filter_map(move |rule| {
                    if rule.element == mr.as_str() {
                        let mut element = String::new();
                        element.push_str(&molecule[0..mr.start()]);
                        element.push_str(rule.replacement);
                        element.push_str(&molecule[mr.end()..]);
                        Some(element)
                    } else {
                        None
                    }
                })
        })
        .unique()
        .count()
}

fn part2(input: &Input) -> Output {
    let (input, molecule, rules) = input;
    let starts = input.lines()
        .filter(|line| line.starts_with("e => "))
        .map(|line| &line[5..])
        .collect_vec();
    let molecule = molecule.to_string();
    successors(Some(molecule), |it| {
        rules.iter()
            .find(|&rule| it.contains(rule.replacement))
            .map(|rule| it.replacen(rule.replacement, rule.element, 1))
    })
        .position(|molecule| starts.contains(&molecule.as_str()))
        .unwrap() + 1
}

#[test]
fn default() {
    let input = get_input(15, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(535, part1(&input));
    assert_eq!(212, part2(&input));
}

// Input parsed (35μs)
// 1. 535 (574μs)
// 2. 212 (145μs)
// Total: 757μs
