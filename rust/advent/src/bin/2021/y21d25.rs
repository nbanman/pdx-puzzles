use std::{iter::successors};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 25).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cucumbers {
    east: FxHashSet<Pos>,
    south: FxHashSet<Pos>,
    size: Pos,
}

impl Cucumbers {
    fn step(&self) -> Self {
        let new_east: FxHashSet<Pos> = self.east.iter()
            .map(|&pos| {
                let x = pos.x() + 1;
                let x = if x == self.size.x() { 0 } else { x };
                let new = Pos::from((x, pos.y()));
                if self.east.contains(&new) || self.south.contains(&new) {
                    pos
                } else {
                    new
                }
            })
            .collect();
        let new_south = self.south.iter()
            .map(|&pos| {
                let y = pos.y() + 1;
                let y = if y == self.size.y() { 0 } else { y };
                let new = Pos::from((pos.x(), y));
                if new_east.contains(&new) || self.south.contains(&new) {
                    pos
                } else {
                    new
                }
            })
            .collect();
        Self { east: new_east, south: new_south, size: self.size }
    }
}

fn make_cucumbers(input: &str) -> Cucumbers {
    let lines = input.lines().collect_vec();
    let sea_floor: Vec<_> = lines.iter().enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes().iter().enumerate()
                .map(move |(x, &b)| (Pos::from((x, y)), b))
        })
        .collect();
    let east: FxHashSet<Pos> = sea_floor.iter()
        .filter(|&&(_, b)| b == b'>')
        .map(|&(pos, _)| pos)
        .collect();
    let south: FxHashSet<Pos> = sea_floor.iter()
        .filter(|&&(_, b)| b == b'v')
        .map(|&(pos, _)| pos)
        .collect();
    let size = Pos::from((lines[0].len(), lines.len()));
    Cucumbers { east, south, size }
}

fn part1(input: Input) -> Output {
    successors(Some(make_cucumbers(input)), |cucumbers| Some(cucumbers.step()))
        .tuple_windows()
        .position(|(prev, next)| prev == next)
        .unwrap() + 1
}

#[test]
fn default() {
    let input = get_input(21, 25).unwrap();
    assert_eq!(528, part1(&input));
}

// Input parsed (32μs)
// 1. 528 (90ms)
// Total: 90ms