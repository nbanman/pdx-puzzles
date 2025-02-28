use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 15).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn hash_of(step: &str) -> usize {
    step.as_bytes()
        .iter()
        .fold(0, |acc, &c| (acc + c as usize) * 17 % 256)
}

fn parse_input(input: Input) -> impl Iterator<Item = &str> {
    input.trim_end().split(',')
}

fn part1(input: Input) -> Output {
    parse_input(input).map(hash_of).sum()
}

fn part2(input: Input) -> Output {
    let mut boxes: [HashMap<&str, (usize, usize)>; 256] = std::array::from_fn(|_| HashMap::new());
    parse_input(input).enumerate().for_each(|(index, step)| {
        let (label, operation) = step.split_once(|c: char| !c.is_ascii_alphabetic()).unwrap();
        let box_number = hash_of(label);
        if operation == "" {
            boxes[box_number].remove(label);
        } else {
            let digit = operation.parse().unwrap();
            let index = if let Some((i, _)) = boxes[box_number].get(label) {
                *i
            } else {
                index
            };
            boxes[box_number].insert(label, (index, digit));
        }
    });
    boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, lens_box)| {
            lens_box
                .values()
                .sorted_by(|(a, _), (b, _)| Ord::cmp(&a, &b))
                .enumerate()
                .fold(0, |acc, (lens_index, (_, focal_length))| {
                    acc + (box_index + 1) * (lens_index + 1) * focal_length
                })
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(23, 15).unwrap();
    assert_eq!(505427, part1(&input));
    assert_eq!(243747, part2(&input));
}

// Input parsed (21μs)
// 1. 505427 (68μs)
// 2. 243747 (244μs)
// Total: 336μs
