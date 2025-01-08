use std::iter;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Vec<(String, Vec<usize>)>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug)]
struct SpringRow {
    conditions: String,
    damage_report: Vec<usize>,
    cache: FxHashMap<State, usize>,
}

impl SpringRow {
    fn arrangements(&mut self, s: State) -> usize {
        if let Some(value) = self.cache.get(&s) {
            *value
        } else {
            // do not consider conditions already handled in previous states
            // if state place exceeds the conditions length string, we are done and the block is blank
            let block = if s.conditions_index < self.conditions.len() {
                self.conditions[s.conditions_index..].to_string()
            } else {
                String::from("")
            };

            // the # of consecutive broken springs in the damage report that we try to place along the row
            let fulfillment = self.damage_report.get(s.damage_index).unwrap_or(&0).to_owned();

            // Base case. Takes states that have fulfilled the entire damage report and returns 1 if valid,
            // 0 if invalid. Valid states are those with no remaining '#' in the current or any future blocks,
            // and that have filled all the damaged spring requirements
            if fulfillment == 0usize {
                let value = if block.find('#').is_some() {
                    0
                } else {
                    1
                };
                self.cache.insert(s, value);
                return value;
            }

            // Otherwise, we go recursive by trying to fit the fulfillment in every place along the block
            // This starts as a sequence of indexes, from 0 until the length of the block minus the fulfillment size
            // (to account for the size of the fulfillment itself in the string).
            let value = if block.len() >= fulfillment {
                (0..=block.len() - fulfillment)
                    .take_while(|index| { *index == 0 || block.as_bytes()[index - 1] as char != '#' })
                    .filter(|index| {
                        // filter out invalid placements, in cascading fashion
                        // if the placement includes a '.', invalid b/c '.' means not broken
                        // if the placement has no part of the string after it, valid b/c nothing else to consider
                        // if the character following the placement is '#', invalid b/c that extra '#' would overfulfill
                        // otherwise valid
                        if let Some(_) = &block[*index..index + fulfillment].find('.') {
                            false
                        } else if index + fulfillment == block.len() {
                            true
                        } else if block.as_bytes()[index + fulfillment] == '#' as u8 {
                            false
                        } else {
                            true
                        }
                    })
                    .map(|index| {
                        let new_state = State {
                            conditions_index: s.conditions_index + index + fulfillment + 1,
                            damage_index: s.damage_index + 1,
                        };
                        self.arrangements(new_state)
                    })
                    .sum()
            } else {
                0
            };
            self.cache.insert(s, value);
            value
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    conditions_index: usize,
    damage_index: usize,
}

fn parse_input(input: &str) -> Input {
    input.lines()
    .map(|line| {
        let (conditions, damage_str) = line.split_once(' ').unwrap();
        let damage_report: Vec<usize> = damage_str.split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        (conditions.to_string(), damage_report)
    })
    .collect()
}

fn solve(spring_rows: Vec<SpringRow>) -> usize {
    spring_rows.into_iter()
        .map(|mut spring_row| {
            let result = spring_row.arrangements(
                State {
                    conditions_index: 0,
                    damage_index: 0,
                }
            );
            result
        }).sum()
}

fn part1(spring_reports: &Input) -> Output {
    let spring_rows: Vec<SpringRow> = spring_reports.iter()
        .map(|(conditions, damage_report)| {
            let cache: FxHashMap<State, usize> = FxHashMap::default();
            SpringRow { 
                conditions: conditions.clone(), 
                damage_report: damage_report.clone(), 
                cache 
            }
        })
        .collect();

    solve(spring_rows)
}


fn part2(spring_reports: &Input) -> Output {
    let spring_rows: Vec<SpringRow> = spring_reports.into_iter()
        .map(|(conditions, damage_report)| {
            let expanded_conditions = iter::repeat(conditions).take(5)
                .join("?");
            let expanded_damage_report = damage_report.repeat(5);
            let cache: FxHashMap<State, usize> = FxHashMap::default();
            SpringRow {
                conditions: expanded_conditions,
                damage_report: expanded_damage_report,
                cache,
            }
        })
        .collect();

    solve(spring_rows)
}

#[test]
fn default() {
    let input = get_input(23, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(7344, part1(&input));
    assert_eq!(1088006519007, part2(&input));
}

// Input parsed (174μs)
// 1. 7344 (873μs)
// 2. 1088006519007 (17ms)
// Total: 18ms