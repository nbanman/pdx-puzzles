use std::cmp::min;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

const SECONDS: u32 = 2503;

type Input = Vec<Reindeer>;
type Output = u32;

#[derive(Debug, Copy, Clone)]
struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance(&self, seconds: u32) -> u32 {
        let interval = self.duration + self.rest;
        let whole_intervals = seconds / interval;
        let remainder = seconds % interval;
        whole_intervals * (self.speed * self.duration) + min(remainder, self.duration) * self.speed
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(15, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.get_numbers().tuples()
        .map(|(speed, duration, rest)| Reindeer { speed, duration, rest })
        .collect()
}

fn part1(racers: &Input) -> Output {
    racers
        .iter()
        .map(|racer| racer.distance(SECONDS))
        .max()
        .unwrap()
}

fn part2(racers: &Input) -> Output {
    let mut leaderboard = vec![0; racers.len()];
    for t in 1..=SECONDS {
        let distances: Vec<u32> = racers.iter().map(|racer| racer.distance(t)).collect();
        let max_distance = *distances.iter().max().unwrap();
        for (racer, distance) in distances.into_iter().enumerate() {
            if distance == max_distance {
                leaderboard[racer] += 1;
            }
        }
    }
    leaderboard.into_iter().max().unwrap()
}

#[test]
fn default() {
    let input = get_input(15, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(2640, part1(&input));
    assert_eq!(1102, part2(&input));
}

// Input parsed (20μs)
// 1. 2640 (6μs)
// 2. 1102 (51μs)
// Total: 81μs
