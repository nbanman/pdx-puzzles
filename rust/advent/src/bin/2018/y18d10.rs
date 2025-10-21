use std::collections::HashSet;
use advent_ocr::ocr;
use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::{Coord, Coord2},
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = (Vec<Pos>, usize);
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut points = Vec::new();
    let mut velocities = Vec::new();
    for (px, py, vx, vy) in input.get_numbers().tuples() {
        points.push(Pos::new([px, py]));
        velocities.push(Pos::new([vx, vy]));
    }
    let velocities = velocities;
    let mut second = 0;

    loop {
        second += 1;
        points = points
            .into_iter()
            .zip(velocities.iter())
            .map(|(p, &v)| p + v)
            .collect();
        let (min, max) = Coord::min_max(&points);
        if max.y() - min.y() == 9 {
            return (points, second);
        }
    }
}

fn part1(input: &Input) -> String {
    let (points, _) = input;
    let points: HashSet<Pos> = points.into_iter().copied().collect();
    let graphic = Coord::coords_to_graphic(&points);
    ocr(graphic.as_str()).unwrap()
}

fn part2(input: &Input) -> usize {
    let (_, second) = input;
    *second
}

#[test]
fn default() {
    let input = get_input(18, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!("LRCXFXRP".to_string(), part1(&input));
    assert_eq!(10630, part2(&input));
}

// Input parsed (3ms)
// 1. LRCXFXRP (47μs)
// 2. 10630 (2μs)
// Total: 3ms