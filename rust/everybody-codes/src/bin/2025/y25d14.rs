#![feature(const_trait_impl)]

use everybody_codes::utilities::inputs::get_event_inputs;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;

#[derive(Debug)]
struct Floor<const N: usize> {
    tiles: [u64; N],
    shaken: [u64; N],
}

impl<const N: usize> Floor<N> {
    fn next(&mut self, mask: u64) -> &Self {
        for i in 0..self.shaken.len() {
            let row = self.tiles[i];
            let shl = (row << 1) & mask;
            let shr = row >> 1;
            self.shaken[i] = shl ^ shr
        }
        for (i, row) in self.tiles.iter_mut().enumerate() {
            let up = if i == 0 { 0 } else { self.shaken[i - 1] };
            let down = if i == self.shaken.len() - 1 {
                0
            } else {
                self.shaken[i + 1]
            };
            *row = !(*row ^ (up ^ down)) & mask;
        }
        self
    }

    fn active(&self) -> u64 {
        self.tiles.iter().map(|row| row.count_ones() as u64).sum()
    }
}

impl<const N: usize> From<&str> for Floor<N> {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let tiles = std::array::from_fn(|_| {
            lines
                .next()
                .unwrap()
                .as_bytes()
                .iter()
                .fold(0u64, |acc, &b| match b {
                    b'#' => (acc << 1) | 1,
                    b'.' => acc << 1,
                    b'\n' => acc,
                    _ => unreachable!(),
                })
        });
        let shaken = [0u64; N];
        Self { tiles, shaken }
    }
}

#[derive(Debug)]
struct SymmetricFloor {
    tiles: [u32; 17],
    shaken: [u32; 17],
}

impl SymmetricFloor {
    const MASK: u32 = 131_071;

    fn next(&mut self) {
        for i in 0..self.shaken.len() {
            let row = self.tiles[i];
            let shl = (row << 1) | (row & 1) & Self::MASK;
            let shr = row >> 1;
            self.shaken[i] = shl ^ shr
        }

        for (i, row) in self.tiles.iter_mut().enumerate() {
            let up = if i == 0 { 0 } else { self.shaken[i - 1] };
            let down = if i == 16 {
                self.shaken[i]
            } else {
                self.shaken[i + 1]
            };
            *row = !(*row ^ (up ^ down)) & Self::MASK
        }
    }

    fn active(&self) -> u64 {
        self.tiles
            .iter()
            .map(|row| row.count_ones() as u64)
            .sum::<u64>()
            * 4
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

fn sum_all_active<const N: usize>(mut floor: Floor<N>, rounds: usize) -> u64 {
    let mask = 2u64.pow(N as u32) - 1;
    let mut active = 0;
    for _ in 0..rounds {
        active += floor.next(mask).active();
    }
    active
}

fn part1(input: Input) -> u64 {
    let floor: Floor<10> = input.into();
    sum_all_active(floor, 10)
}

fn part2(input: Input) -> u64 {
    let floor: Floor<34> = input.into();
    sum_all_active(floor, 2025)
}

fn part3(input: Input) -> u64 {
    let mut floor = SymmetricFloor {
        tiles: [0; 17],
        shaken: [0; 17],
    };
    let center = input.as_bytes()[0..input.as_bytes().len() / 2]
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, &b)| {
            if idx % 9 > 3 {
                acc
            } else {
                match b {
                    b'#' => acc << 1 | 1,
                    b'.' => acc << 1,
                    b'\n' => acc,
                    _ => unreachable!(),
                }
            }
        });

    // skip the first round b/c it has nothing in it and does not cycle
    let total_rounds = 999_999_999;
    let cycle_length = 4095;
    let cycles = (total_rounds / cycle_length) as u64;
    let remainder = total_rounds % cycle_length;
    let mut cycle_sum = 0;
    let mut remainder_sum = 0;

    for index in 0..cycle_length {
        floor.next();
        let floor_center = floor.tiles[13..]
            .iter()
            .fold(0u32, |acc, &row| acc << 4 | (row & 0xF));
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

// Input parsed (26μs)
// 1. 474 (8μs)
// 2. 1170584 (74μs)
// 3. 1012942728 (25μs)
// Total: 137μs
