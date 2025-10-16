use std::collections::VecDeque;

use itertools::Itertools;
use advent::utilities::get_input::get_input;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone)]
struct State {
    write0: bool,
    left0: bool,
    change0: usize,
    write1: bool,
    left1: bool,
    change1: usize,
}

fn penultimate(s: &str) -> &str {
    let ultimate = s.split(' ').rev().next().unwrap();
    &ultimate[..ultimate.len() - 1]
}
fn part1(input: Input) -> Output {
    let (init, states) = input.split_once("\n\n").unwrap();
    let mut init_lines = init.lines();
    let state_idx = penultimate(init_lines.next().unwrap())
        .chars()
        .next()
        .map(|c| (c as u8 - b'A') as usize)
        .unwrap();
    let steps: usize = init_lines.next().unwrap().get_numbers().next().unwrap();

    let states = states.split("\n\n")
        .map(|stanza| {
            let (_, _, write0, dir0, change0, _, write1, dir1, change1) = stanza.lines()
                .map(|line| penultimate(line))
                .collect_tuple()
                .unwrap();
            let write0 = write0 == "1";
            let left0 = dir0 == "left";
            let change0 = (change0.as_bytes()[0] - b'A') as usize;
            let write1 = write1 == "1";
            let left1 = dir1 == "left";
            let change1 = (change1.as_bytes()[0] - b'A') as usize;
            State { write0, left0, change0, write1, left1, change1 }
        })
        .collect_vec();

    let mut slots = VecDeque::new();
    slots.push_front(false);
    let mut state = &states[state_idx];
    let mut node = 0;

    for _ in 0..steps {
        let node_val = slots.get_mut(node).unwrap();
        if *node_val {
            *node_val = state.write1;
            if state.left1 {
                if node == 0 {
                    slots.push_front(false);
                } else {
                    node -= 1;
                }
            } else {
                if node == slots.len() - 1 {
                    slots.push_back(false);
                }
                node += 1;
            }
            state = &states[state.change1];
        } else {
            *node_val = state.write0;
            if state.left0 {
                if node == 0 {
                    slots.push_front(false);
                } else {
                    node -= 1;
                }
            } else {
                if node == slots.len() - 1 {
                    slots.push_back(false);
                }
                node += 1;
            }
            state = &states[state.change0];
        }
    }
    
    slots.into_iter().filter(|&b| b).count()
}
#[test]
fn default() {
    let input = get_input(17, 25).unwrap();
    assert_eq!(3745, part1(&input));
}

// Input parsed (16μs)
// 1. 3745 (21ms)
// Total: 21ms
