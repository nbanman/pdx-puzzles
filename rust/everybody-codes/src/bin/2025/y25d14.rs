use std::iter::successors;
use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

#[derive(Debug)]
struct Floor(Vec<u64>);

impl Floor {
    fn next(&self, mask: u64) -> Self {
        let shaken = self.0.iter()
            .map(|&row| {
                let shl = (row << 1) & mask;
                let shr = row >> 1;
                shl ^ shr
            })
            .collect_vec();

        let inner = self.0.iter().enumerate()
            .map(|(i, &row)| {
                let up = if i == 0 {
                    0
                } else {
                    shaken[i - 1]
                };
                let down = if i == shaken.len() - 1 {
                    0
                } else {
                    shaken[i + 1]
                };
                !(row ^ (up ^ down)) & mask
            })
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

#[derive(Debug)]
struct SymmetricFloor([u64; 17]);

impl SymmetricFloor {
    const MASK: u64 = 17_179_869_183;

    fn next(&self) -> Self {
        let shaken = self.0.map(|row| {
            let shl = (row << 1) & Self::MASK;
            let shr = row >> 1;
            shl ^ shr
        });

        let inner: [u64; 17] = std::array::from_fn(|i| {
            let up = if i == 0 {
                0
            } else {
                shaken[i - 1]
            };
            let row = self.0[i];
            let down = if i == 16 {
                shaken[i]
            } else {
                shaken[i + 1]
            };
            !(row ^ (up ^ down)) & Self::MASK
        });
        Self(inner)
    }

    fn active(&self) -> u64 {
        self.0.iter().map(|row| row.count_ones() as u64).sum::<u64>() * 2
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
    let floor = SymmetricFloor([0u64; 17]);
    let center = input.as_bytes()[0..input.as_bytes().len() / 2].iter()
        .fold(0u64, |acc, &b| {
            match b {
                b'#' => acc << 1 | 1,
                b'.' => acc << 1,
                b'\n' => acc,
                _ => unreachable!(),
            }
        });

    // skip the first round b/c it has nothing in it and does not cycle
    let total_rounds = 999_999_999;
    let cycle_length = 4095;
    let cycles = (total_rounds / cycle_length) as u64;
    let remainder = total_rounds % cycle_length;
    let mut cycle_sum = 0;
    let mut remainder_sum = 0;

    for (index, floor) in successors(Some(floor), |it| Some(it.next()))
        .enumerate()
        .skip(1)
        .take(cycle_length)
    {
        let floor_center = floor.0[13..17].iter()
            .fold(0u64, |acc, &row| acc << 8 | (row >> 13 & 0xFF));
        if floor_center == center {
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

// Input parsed (29μs)
// 1. 474 (9μs)
// 2. 1170584 (148μs)
// 3. 1012942728 (121μs)
// Total: 311μs