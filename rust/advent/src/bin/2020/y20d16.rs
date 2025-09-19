use std::{collections::HashMap, ops::RangeInclusive};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::stopwatch::{ReportDuration, Stopwatch},
};
use utilities::structs::grid::Grid2;

type Input<'a> = (Vec<Rule<'a>>, Vec<Vec<usize>>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 16).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Rule<'a> {
    name: &'a str,
    low: RangeInclusive<usize>,
    high: RangeInclusive<usize>,
}

impl<'a> Rule<'a> {
    fn valid_for(&self, values: &Vec<&usize>) -> bool
    {
        values.into_iter().all(|&it| self.low.contains(it) || self.high.contains(it))
    }
}
fn parse_input(input: &'_ str) -> Input<'_> {
    let (rules, tickets) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|rule| {
            let (name, ranges) = rule.split_once(": ").unwrap();
            let (lolo, lohi, hilo, hihi) = ranges.get_numbers().collect_tuple().unwrap();
            Rule {
                name,
                low: lolo..=lohi,
                high: hilo..=hihi,
            }
        })
        .collect();
    let tickets: Vec<Vec<usize>> = tickets
        .lines()
        .map(|ticket| ticket.get_numbers().collect())
        .filter(|ticket: &Vec<usize>| !ticket.is_empty())
        .collect();
    (rules, tickets)
}

fn part1(input: &Input) -> Output {
    let (rules, tickets) = input;
    tickets.iter().flatten()
        .filter(|&value| {
            rules.iter().all(|rule| !rule.low.contains(value) && !rule.high.contains(value))
        })
        .sum() 
}

fn part2(input: &Input) -> Output {
    let (rules, tickets) = input;
    let valid_tickets: Grid2<usize> = tickets.iter()
        .filter(|ticket| ticket.iter().all(|value| {
            rules.iter().any(|rule| rule.low.contains(value) || rule.high.contains(value))
        }))
        .cloned()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    // a list of a tuple. the first value is the field column, the second is all the possible
    // fields the column could correspond to. Gradually eliminate invalid ones, then when there is
    // only one possibility, assign it to the column and take it out of consideration for future
    // field columns.
    let mut sorter: Vec<(usize, Vec<&Rule>)> = valid_tickets
        .columns()
        .enumerate()
        .map(|(index, values)| {
            let filtered: Vec<_> = rules.iter().filter(|rule| rule.valid_for(&values)).collect();
            (index, filtered)
        })
        .collect();
    let mut register: HashMap<&Rule, usize> = HashMap::new();
    while !sorter.is_empty() {
        let (singles, multiples): (Vec<(usize, Vec<&Rule>)>, Vec<(usize, Vec<&Rule>)>) = sorter
            .into_iter()
            .partition(|(_, potential_rules)| potential_rules.len() == 1);
        sorter = multiples;
        for (col_idx, single_vec) in singles {
            for (_, potential_rules) in sorter.iter_mut() {
                potential_rules.retain(|rule| *rule != single_vec[0])
            }
            register.insert(single_vec[0], col_idx);
        }
    }
    rules.iter()
        .filter(|rule| rule.name.starts_with("departure"))
        .map(|rule| valid_tickets.get([register[&rule], 0]).unwrap())
        .fold(1, |acc, i| acc * i)
}

#[test]
fn default() {
    let input = get_input(20, 16).unwrap();
    let input = parse_input(&input);
    assert_eq!(29878, part1(&input));
    assert_eq!(855438643439, part2(&input));
}

// Input parsed (311μs)
// 1. 29878 (20μs)
// 2. 855438643439 (223μs)
// Total: 557μs