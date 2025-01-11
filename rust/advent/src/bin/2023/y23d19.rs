use std::ops::RangeInclusive;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use lazy_regex::regex;
use rustc_hash::FxHashMap;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Rules = FxHashMap<String, Vec<Rule>>;
type Input = (Rules, Vec<Vec<usize>>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
struct Rule {
    category: usize,
    amount: usize,
    comparison: Comparison,
    destination: String,
}

#[derive(Debug, PartialEq)]
enum Comparison {
    Greater,
    Less,
    Pass,
}

fn parse_input(input: &str) -> Input {
    let (work_stanza, part_stanza) = input.split_once("\n\n").unwrap();

    let rx = regex!(r"(?<name>[A-z]+)\{(?<conditionals>.*),(?<last>[A-z]+)\}");
    let rx_conditionals = 
        regex!(r"(?<category>[A-z]+)(?<comparison><|>)(?<amount>\d+):(?<destination>[A-z]+)");
    
    let workflows: Rules = work_stanza
        .lines()
        .map(|line| {
            let captures = rx.captures(line).unwrap();
            let name = captures.name("name").unwrap().as_str().to_string();
            let mut conditionals: Vec<Rule> = rx_conditionals
                .captures_iter(captures.name("conditionals").unwrap().as_str())
                .map(|caps| {
                    let category = match caps.name("category").unwrap().as_str() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        c => panic!("{c} not a valid category!"),
                    };
                    let amount: usize = caps.name("amount").unwrap().as_str().parse().unwrap();
                    let comparison = match caps.name("comparison").unwrap().as_str() {
                        "<" => Comparison::Less,
                        ">" => Comparison::Greater,
                        c => panic!("{c} not a valid comparison!"),
                    };
                    let destination = caps.name("destination").unwrap().as_str().to_string();
                    Rule { category, amount, comparison, destination }
                })
                .collect();
            let last = Rule { 
                category: 0, 
                amount: 0, 
                comparison: Comparison::Pass, 
                destination: captures.name("last").unwrap().as_str().to_string(), 
            };
            conditionals.push(last);
            (name, conditionals)
        })
        .collect();

    let parts = part_stanza
        .get_numbers::<usize>()
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();

    (workflows, parts)
}

fn sort(name: &str, part: &Vec<usize>, workflows: &Rules ) -> String {
    let workflow = workflows.get(name).unwrap();
    for rule in workflow {
        let result = match rule.comparison {
            Comparison::Greater => {
                if part[rule.category] > rule.amount {
                    Some(&rule.destination)
                } else {
                    None
                }
            },
            Comparison::Less => {
                if part[rule.category] < rule.amount {
                    Some(&rule.destination)
                } else {
                    None
                }
            },
            Comparison::Pass => Some(&rule.destination),
        }; 

        if let Some(result) = result {
            return if result == "A" || result == "R" {
                result.clone()
            } else {
                sort(result, part, workflows)    
            }
        } 
    }
    unreachable!()
}

fn part1(input: &Input) -> Output {
    let (workflows, parts) = input;
    parts.iter()
        .filter(|&part| &sort("in", part, workflows) == "A")
        .map(|part| part.iter().sum::<usize>())
        .sum()
}

#[derive(Clone, Debug)]
struct PartRanges([RangeInclusive<usize>; 4]);

impl PartRanges {
    fn new() -> Self {
        Self(std::array::from_fn(|_| 1..=4000))
    }
    
    fn permutations(&self) -> usize {
        self.0.iter().fold(1, |acc, range| {
            acc * (1 + range.end() - range.start())
        })
    }

    fn split(&self, rule: &Rule) -> [PartRanges; 2] {
        match rule.comparison {
            Comparison::Greater => {
                let breakpoint = rule.amount + 1;
                let pass = breakpoint..=*self.0[rule.category].end();
                let fail = *self.0[rule.category].start()..=breakpoint - 1;
                self.make_splits(rule.category, pass, fail)
            },
            Comparison::Less => {
                let breakpoint = rule.amount;
                let pass = *self.0[rule.category].start()..=breakpoint - 1;
                let fail = breakpoint..=*self.0[rule.category].end();
                self.make_splits(rule.category, pass, fail)
            },
            Comparison::Pass => panic!("Non-comparisons should not be passed to split function."),
        }
    }
    
    fn make_splits(&self, 
        category: usize, 
        pass: RangeInclusive<usize>, 
        fail: RangeInclusive<usize>,
    ) -> [PartRanges; 2] {
        [
            PartRanges(std::array::from_fn(|i| {
                if i == category { pass.clone() } else { self.0[i].clone() }})
            ),
            PartRanges(std::array::from_fn(|i| {
                if i == category { fail.clone() } else { self.0[i].clone() }})
            ),
        ]
    }
}

fn route(workflow: &[Rule], part_ranges: PartRanges) -> Vec<(String, PartRanges)> {
    let mut routes = Vec::new();
    let mut remaining = Some(part_ranges);
    for rule in workflow {
        if let Some(range) = remaining {
            remaining = if rule.comparison != Comparison::Pass {
                let [pass, fail] = range.split(rule);
                routes.push((rule.destination.clone(), pass));
                Some(fail)
            } else {
                routes.push((rule.destination.clone(), range));
                None
            }
        } else {
            break;
        }
    }
    routes
}

fn part2(input: &Input) -> Output {
    let (workflows, _) = input;
    let mut accepted: Vec<PartRanges> = Vec::new();
    let mut remaining = Vec::new();
    remaining.push(("in".to_string(), PartRanges::new()));
    while !remaining.is_empty() {
        let next: Vec<_> = remaining.iter()
            .flat_map(|(name, part_ranges)| {
                route(workflows.get(name).unwrap(), part_ranges.clone())
            })
            .collect();
        remaining = next.into_iter()
            .filter(|(name, part_ranges)| {
                match name.as_str() {
                    "R" => false,
                    "A" => { 
                        accepted.push(part_ranges.clone());
                        false 
                    },
                    _ => true,
                }
            })
            .collect();
    }
    accepted.iter().map(|v| v.permutations()).sum()
}

#[test]
fn default() {
    let input = get_input(23, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!(449531, part1(&input));
    assert_eq!(122756210763577, part2(&input));
}

// Input parsed (1ms)
// 1. 449531 (35μs)
// 2. 122756210763577 (247μs)
// Total: 1ms