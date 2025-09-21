use std::collections::HashSet;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use utilities::{math::formulae::gcd, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (usize, Pos, Vec<Pos>);
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let asteroids: Vec<Pos> = input.replace('\n', "").chars().enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(index, _)| Pos::new2d((index % width) as i64, (index / width) as i64))
        .collect();
    let (detectable_from_station, station) = asteroids.iter()
        .map(|&asteroid| {
            let detectable = asteroids.iter()
                .filter(|&&other| other != asteroid)
                .map(|&other| {
                    let relative_pos = asteroid - other;
                    let gcd = gcd(relative_pos.x(), relative_pos.y()).abs();
                    let new = relative_pos / gcd as i64;
                    new
                })
                .collect::<HashSet<Pos>>()
                .len();
            (detectable, asteroid)
        })
        .max_by_key(|(detectable, _)| *detectable)
        .unwrap();
    (detectable_from_station, station, asteroids)
}

fn part1(input: &Input) -> usize {
    let (detectable_from_station, _, _) = input;
    *detectable_from_station
}

fn part2(input: &Input) -> i64 {
    let (_, station, asteroids) = input;
    let angles = asteroids.iter()
        .filter(|&asteroid| asteroid != station)
        .map(|&asteroid| {
            let relative_pos = *station - asteroid;
            let gcd = gcd(relative_pos.x(), relative_pos.y()).abs();
            let new = relative_pos / gcd as i64;
            let new_angle = (new.x() as f64).atan2(new.y() as f64);
            let new_angle = if new_angle <= 0.0 {
                new_angle
            } else {
                -2.0 * std::f64::consts::PI + new_angle
            };
            // println!("({}, {}, {})", asteroid, new, new_angle);
            (OrderedFloat(new_angle), asteroid)
        })
        .sorted_unstable_by_key(|(_, asteroid)| asteroid.manhattan_distance(station))
        .into_group_map_by(|(angle, _)| *angle)
        .into_values()
        .collect_vec();
    let mut pq = Vec::new();
    for angle in angles {
        for (index, (angle, asteroid)) in angle.into_iter().enumerate() {
            pq.push((OrderedFloat(-10.0 * index as f64) + angle, asteroid));
        }
    }
    pq.sort_unstable_by_key(|(angle, _)| OrderedFloat(angle.abs()));
    let asteroid = pq[199].1;
    asteroid.x() * 100 + asteroid.y()
}

#[test]
fn default() {
    let input = get_input(19, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(286, part1(&input));
    assert_eq!(504, part2(&input));
}

// Input parsed (7ms)
// 1. 286 (6μs)
// 2. 504 (82μs)
// Total: 7ms