use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (Int, Vec<Bus>);
type Int = usize;
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 13).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

struct Bus {
    id: Int,
    offset: Int,
}

#[derive(Debug)]
struct State {
    id: Int,
    time: Int,
}

fn modular_inverse(ni: Int, modulus: Int) -> Int {
    (1..)
        .find(|it| (ni % modulus * *it) % modulus == 1)
        .unwrap()
}

fn crt(buses: &Vec<Bus>) -> Bus {
    let n = buses.iter().fold(1, |acc, bus| acc * bus.id);
    let big_phase: Int = buses.iter()
        .map(|bus| {
            let ni = n / bus.id;
            bus.offset * ni * modular_inverse(ni, bus.id)
        })
        .sum();
    Bus { id: n, offset: big_phase % n }
}

fn parse_input(input: &str) -> Input {
    let (start, buses) = input.trim_end().split_once('\n').unwrap();
    let start = start.parse().unwrap();
    let buses = buses.split(',').enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(index, id)| {
            Bus { id: id.parse().unwrap(), offset: index }
        })
        .collect();
    (start, buses)
}

fn part1(input: &Input) -> Output {
    let (start, buses) = input;
    let mut time_sequence = successors(Some(State { id: 0, time: *start}), |State { id: _, time }| {
        let id = buses.iter()
            .find(|bus| (*time + 1) % bus.id == 0)
            .map(|bus| bus.id)
            .unwrap_or(0);
        let next_state = State { id, time: *time + 1 };
        Some(next_state)
    });

    time_sequence
        .find(|state| state.id != 0)
        .map(|state| state.id * (state.time - *start))
        .unwrap()
}

fn part2(input: &Input) -> Output {
    let (_, buses) = input;
    let bus = crt(buses);
    bus.id - bus.offset
}

#[test]
fn default() {
    let input = get_input(20, 13).unwrap();
    let input = parse_input(&input);
    assert_eq!(115, part1(&input));
    assert_eq!(756261495958122, part2(&input));
}

// Input parsed (22μs)
// 1. 115 (5μs)
// 2. 756261495958122 (3μs)
// Total: 33μs