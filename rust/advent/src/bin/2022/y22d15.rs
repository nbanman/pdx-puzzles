use std::{cmp::max, iter::successors, ops::RangeInclusive};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Pos = Coord2;
type Input = Vec<Sensor>;
type Output = i64;

struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn to_range(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let x_distance = (self.pos.x() - self.beacon.x()).abs() +
            (self.pos.y() - self.beacon.y()).abs() -
            (self.pos.y() - y).abs();
        if x_distance >= 0 {
            Some(self.pos.x() - x_distance..=self.pos.x() + x_distance)
        } else {
            None
        }
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn is_contiguous(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    let binding = [a, b];
    let (&lesser, &greater) = binding.iter()
        .minmax_by_key(|r| r.start())
        .into_option()
        .unwrap();
    if lesser.end() >= greater.start() {
        Some(*lesser.start()..=*max(lesser.end(), greater.end()))
    } else {
        None
    }
}

fn row_ranges(sensors: &Vec<Sensor>, y: i64) -> Vec<RangeInclusive<i64>> {
    let mut row_ranges: Vec<_> = sensors.into_iter()
        .filter_map(|sensor| sensor.to_range(y))
        .collect();

    let mut i: usize;
    let mut size = 0usize;
    while size != row_ranges.len() {
        size = row_ranges.len();
        i = 0;
        while i < row_ranges.len() - 1 {
            let mut j = i + 1;
            while j < row_ranges.len() {
                if let Some(union) = is_contiguous(&row_ranges[i], &row_ranges[j]) {
                    row_ranges[i] = union;
                    row_ranges.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    row_ranges
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(x, y)| Pos::new2d(x, y))
        .tuples()
        .map(|(pos, beacon)| Sensor { pos, beacon })
        .collect()
}

fn part1(sensors: &Input) -> Output {
    row_ranges(sensors, 2_000_000)
        .into_iter()
        .map(|range| range.end() - range.start())
        .sum()
}

fn part2(sensors: &Input) -> Output {
    let (y, range) = successors(Some(0i64), |i| Some(i + 1))
        .map(|y| (y, row_ranges(&sensors, y)))
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();
    let x = range.iter().min_by(|x, y| x.start().cmp(&y.start())).unwrap().end() + 1;
    4_000_000i64 * x + y
}

#[test]
fn default() {
    let input = get_input(22, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(5073496, part1(&input));
    assert_eq!(13081194638237, part2(&input));
}

// Input parsed (20μs)
// 1. 5073496 (5μs)
// 2. 13081194638237 (325ms)
// Total: 325ms