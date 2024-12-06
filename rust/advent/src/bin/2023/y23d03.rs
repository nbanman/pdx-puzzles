use std::{collections::HashSet, ops::Range};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 3).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn numbers_adjacent_to_symbol<F>(schematic: &str, width: usize, symbol: F) -> Vec<HashSet<Range<Output>>>
    where
        F: Fn(&char) -> bool
{
    schematic.chars().enumerate()
        .filter(|(_, c)| symbol(c))
        .map(|(index, _)| {
            let mut set_of_ranges: HashSet<Range<usize>> = HashSet::new();
            for y in -1isize..=1 {
                for x in -1isize..=1 {
                    let new_index = index as isize + y * width as isize + x;
                    if let Some(int_range) = get_number(schematic, new_index) {
                        set_of_ranges.insert(int_range);
                    }
                }
            };
            set_of_ranges
        }).collect()
}

fn get_number(schematic: &str, index: isize) -> Option<Range<Output>> {
    if index < 0 || index >= schematic.len() as isize { return None; };
    if !schematic.as_bytes()[index as usize].is_ascii_digit() { return None; };
    let mut left_index = index as usize;
    let mut right_index = index as usize;
    while let Some(x) = left_index.checked_sub(1) {
        if schematic.as_bytes()[x].is_ascii_digit() {
            left_index = x;
        } else {
            break;
        }
    };
    while schematic.as_bytes()[right_index + 1].is_ascii_digit() {
        right_index += 1;
    };
    Some(left_index..right_index)
}


fn part1(schematic: Input) -> Output {
    let width = schematic.find('\n').unwrap() + 1;
    let symbol = |c: &char| {
        *c != '\n' && *c != '.' && !c.is_ascii_digit()
    };
    numbers_adjacent_to_symbol(schematic, width, symbol)
        .iter()
        .flatten()
        .unique()
        .map(|range| {
            let start = range.start;
            let end = range.end;
            &schematic[start..=end]
        })
        .filter_map(|substring| substring.parse::<usize>().ok())
        .sum()
}

fn part2(schematic: Input) -> Output {
    let width = schematic.find('\n').unwrap() + 1;
    let symbol = |c: &char| {
        *c == '*'
    };
    numbers_adjacent_to_symbol(schematic, width, symbol)
        .iter()
        .filter(|set| set.len() == 2)
        .map(|set| {
            set
                .iter()
                .map(|range| {
                    let start = range.start;
                    let end = range.end;
                    &schematic[start..=end]
                })
                .filter_map(|substring| substring.parse::<usize>().ok())
                .product::<usize>()
        }).sum()
}

#[test]
fn default() {
    let input = get_input(23, 3).unwrap();
    assert_eq!(525911, part1(&input));
    assert_eq!(75805607, part2(&input));
}
