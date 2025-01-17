use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Int = usize;
type Input = (Vec<VecDeque<char>>, Vec<(Int, Int, Int)>);
type Output = String;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 5).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (stacks_str, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = vec![VecDeque::new(); 10];
    
    for line in stacks_str.lines().dropping_back(1) {
        for (idx, c) in line.chars().enumerate().filter(|(_, c)| c.is_alphabetic()) {
            stacks[idx / 4].push_back(c);
        }
    }
    
    let instructions = instructions
        .trim_ascii_end()
        .lines()
        .map(|line| {
            let (quantity, from, to) = line.get_numbers().tuples().next().unwrap();
            (quantity, from - 1, to - 1)
        })
        .collect();

    (stacks, instructions)
}

fn top(stacks: &[VecDeque<char>]) -> Output {
    stacks.iter()
        .filter_map(|stack| stack.front())
        .collect()
}

fn part1(input: Input) -> Output {
    let (mut stacks, instructions) = input;
    for (quantity, from, to) in instructions {
        for _ in 0..quantity {
            if let Some(crayt) = stacks[from].pop_front() {
                stacks[to].push_front(crayt);
            }
        }
    }
    top(&stacks)
}

fn part2(input: Input) -> Output {
    let (mut stacks, instructions) = input;
    let mut temp = VecDeque::new();
    for (quantity, from, to) in instructions {
        for _ in 0..quantity {
            if let Some(crayt) = stacks[from].pop_front() {
                temp.push_front(crayt);
            }
        }
        while let Some(crayt) = temp.pop_front() {
            stacks[to].push_front(crayt);
        }
    }
    top(&stacks)
}

#[test]
fn default() {
    let input = get_input(22, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!("ZSQVCCJLL".to_string(), part1(input.clone()));
    assert_eq!("QZFJRWHGS".to_string(), part2(input));
}

// Input parsed (46μs)
// 1. ZSQVCCJLL (18μs)
// 2. QZFJRWHGS (14μs)
// Total: 81μs