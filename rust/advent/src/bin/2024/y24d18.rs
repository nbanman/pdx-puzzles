use std::collections::{HashSet, VecDeque};

use advent::utilities::get_input::get_input;
use bit_set::BitSet;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2U,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<BitSet>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples::<(usize, usize)>()
        .scan(BitSet::new(), |state, (x, y)| {
            let mut next = BitSet::new();
            next.insert(y * 71 + x);
            state.union_with(&next);
            Some(state.clone())
        })
        .collect()
}

fn solve(bytes: &Input, simulate: usize) -> Option<usize> {
    let bytes = &bytes[simulate - 1];
    let bounds = 0..71;
    let start = (Coord2U::origin(), 0usize);
    let end = Coord2U::new2d(70, 70);
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some((pos, steps)) = q.pop_front() {
        if pos == end {
            return Some(steps);
        } else {
            visited.insert(pos);
            let neighbors = pos.adjacent(false).into_iter().filter(|neighbor| {
                bounds.contains(&neighbor.x())
                    && bounds.contains(&neighbor.y())
                    && !bytes.contains(neighbor.get_index(&[71]).unwrap())
                    && visited.insert(*neighbor)
            });
            for neighbor in neighbors {
                q.push_back((neighbor, steps + 1));
            }
        }
    }
    None
}

fn part1(bytes: &Input) -> usize {
    solve(bytes, 1024).unwrap()
}

fn part2(bytes: &Input) -> String {
    let mut l = 1024;
    let mut r = bytes.len() - 1;
    while l != r {
        let m = (l + r) / 2;
        if solve(bytes, m).is_none() {
            r = m;
        } else {
            l = m + 1;
        }
    }
    let byte = bytes[r - 1].difference(&bytes[r - 2]).next().unwrap();

    format!("{},{}", byte % 71, byte / 71)
}

#[test]
fn default() {
    let input = get_input(24, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(312, part1(&input));
    assert_eq!("28,26".to_string(), part2(&input));
}

// Input parsed (1ms)
// 1. 312 (981μs)
// 2. 28,26 (1ms)
// Total: 3ms
