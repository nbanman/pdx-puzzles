#![feature(int_lowest_highest_one)]

use std::fmt::Display;
use std::iter::successors;
use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Floor(Vec<u64>);

impl Floor {
    fn next(&self, mask: u64) -> Self {
        let shaken = self.0.iter().map(|&row| {
            let shl = (row << 1) & mask;
            let shr = row >> 1;
            shl ^ shr
        });
        let inner = std::iter::once(0u64)
            .chain(shaken)
            .chain(std::iter::once(0))
            .tuple_windows::<(_, _, _)>()
            .zip(self.0.iter())
            .map(|((up, _, down), &row)| !(row ^ (up ^ down)) & mask)
            .collect();
        Self(inner)
    }

    fn active(&self) -> u64 {
        self.0.iter().map(|row| row.count_ones() as u64).sum()
    }
}

impl From<&str> for Floor {
    fn from(value: &str) -> Self {
        let inner = value
            .lines()
            .map(|line| {
                line.as_bytes().iter().fold(0u64, |acc, &b| match b {
                    b'#' => (acc << 1) | 1,
                    b'.' => acc << 1,
                    b'\n' => acc,
                    _ => unreachable!(),
                })
            })
            .collect();
        Self(inner)
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.0.iter()
            .map(|&n| n.highest_one().unwrap() + 1)
            .max()
            .unwrap();
        let mut grid = String::new();
        for &n in self.0.iter() {
            let mut n = n;
            for _ in 0..size {
                if n & 1 == 1 {
                    grid.push('#');
                } else {
                    grid.push('.');
                }
                n = n >> 1;
            }
            grid.push('\n')
        }
        grid.pop();
        write!(f, "{grid}")
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 14);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn sum_all_active(input: Input, rounds: usize) -> u64 {
    let floor: Floor = input.into();
    let mask = 2u64.pow(input.chars().position(|c| c == '\n').unwrap() as u32) - 1;
    (0..rounds)
        .scan(floor, |state, _| {
            *state = state.next(mask);
            Some(state.active())
        })
        .sum()
}

fn part1(input: Input) -> u64 {
    sum_all_active(input, 10)
}

fn part2(input: Input) -> u64 {
    sum_all_active(input, 2025)
}

fn part3(input: Input) -> u64 {
    let floor = Floor(vec![0u64; 34]);
    let center = input.as_bytes().iter().fold(0u64, |acc, &b| {
        match b {
            b'#' => acc << 1 | 1,
            b'.' => acc << 1,
            b'\n' => acc,
            _ => unreachable!(),
        }
    });
    let center_mask = 2u64.pow(8) - 1;
    let matches_center = |floor: &Floor| {
        center == floor.0[13..21].iter().fold(0u64, |acc, &row| {
            let row = row >> 13 & center_mask;
            acc << 8 | row
        })
    };

    let mask = 17_179_869_183;

    // skip the first round b/c it has nothing in it and does not cycle
    let total_rounds = 999_999_999;
    let cycle_length = 4095;
    let cycles = (total_rounds / cycle_length) as u64;
    let remainder = total_rounds % cycle_length;
    let mut cycle_sum = 0;
    let mut remainder_sum = 0;

    for (index, floor) in successors(Some(floor), |it| Some(it.next(mask)))
        .enumerate()
        .skip(1)
        .take(cycle_length)
    {
        if matches_center(&floor) {
            cycle_sum += floor.active();
        }
        if index == remainder {
            remainder_sum = cycle_sum;
        }
    }
    cycle_sum * cycles + remainder_sum
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 14);
    assert_eq!(474, part1(&input1));
    assert_eq!(1170584, part2(&input2));
    assert_eq!(1012942728, part3(&input3));
}

// Input parsed (30μs)
// 1. 474 (7μs)
// 2. 1170584 (261μs)
// 3. 1012942728 (471μs)
// Total: 772μs