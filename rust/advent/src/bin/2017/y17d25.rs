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

#[derive(Debug)]
enum Dir { Left, Right }

#[derive(Debug)]
struct Action {
    write: bool,
    dir: Dir,
    change: usize,
}

#[derive(Debug)]
struct State {
    zero: Action,
    one: Action,
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
            let dir0 = if dir0 == "left" { Dir::Left } else { Dir::Right };
            let change0 = (change0.as_bytes()[0] - b'A') as usize;
            let write1 = write1 == "1";
            let dir1 = if dir1 == "left" { Dir::Left } else { Dir::Right };
            let change1 = (change1.as_bytes()[0] - b'A') as usize;
            State {
                zero: Action { write: write0, dir: dir0, change: change0 },
                one: Action { write: write1, dir: dir1, change: change1 },
            }
        })
        .collect_vec();

    let mut slots = VecDeque::with_capacity(3746);
    slots.push_front(false);
    let mut state = &states[state_idx];
    let mut node = 0;

    for _ in 0..steps {
        let node_val = slots.get_mut(node).unwrap();
        let action = if *node_val { &state.one } else { &state.zero };
        *node_val = action.write;
        match action.dir {
            Dir::Left => {
                if node == 0 {
                    slots.push_front(false);
                } else {
                    node -= 1;
                }
            }
            Dir::Right => {
                if node == slots.len() - 1 {
                    slots.push_back(false);
                }
                node += 1;
            }
        }

        state = &states[action.change];
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
