use std::{cmp::max, iter::successors, ops::Add};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord2;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

#[derive(Debug)]
struct Robot {
    p: Pos,
    v: Pos,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 14).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_robots(input: Input) -> Vec<Robot> {
    input
        .get_numbers()
        .tuples()
        .map(|(px, py, vx, vy)| Robot {
            p: Pos::new2d(px, py),
            v: Pos::new2d(vx, vy),
        })
        .collect()
}

fn score(robots: Vec<Pos>) -> usize {
    let mut quadrants = [0usize; 4];
    let split_x = WIDTH / 2;
    let split_y = HEIGHT / 2;

    robots
        .iter()
        .filter_map(|robot| {
            let x = match robot.x() {
                x if (0..split_x).contains(&x) => Some(0),
                x if x == split_x => None,
                _ => Some(2),
            }?;
            let y = match robot.y() {
                y if (0..split_y).contains(&y) => Some(0),
                y if y == split_y => None,
                _ => Some(1),
            }?;
            Some((x, y))
        })
        .for_each(|(x, y)| {
            quadrants[x + y] += 1;
        });

    quadrants
        .into_iter()
        .reduce(|acc, count| acc * count)
        .unwrap()
}

fn part1(input: Input) -> Output {
    let robots = parse_robots(input);
    let moved_robots: Vec<Pos> = robots
        .iter()
        .map(|robot| {
            Pos::new2d(
                (robot.p.x() + robot.v.x() * 100).rem_euclid(WIDTH),
                (robot.p.y() + robot.v.y() * 100).rem_euclid(HEIGHT),
            )
        })
        .collect();
    score(moved_robots)
}

fn part2(input: Input) -> Output {
    let robots = parse_robots(input);
    let sample: Vec<(f64, f64)> = (0..max(WIDTH, HEIGHT))
        .map(|moves| {
            let robots: Vec<Pos> = robots
                .iter()
                .map(|robot| {
                    Pos::new2d(
                        (robot.p.x() + robot.v.x() * moves).rem_euclid(WIDTH),
                        (robot.p.y() + robot.v.y() * moves).rem_euclid(HEIGHT),
                    )
                })
                .collect();

            // note that this isn't totally accurate for width because the larger height sample is used,
            // but it should still work because the stars will align only once per period and the
            // variance for that instance should be dramatically lower than for anything else.
            let (x_mean, y_mean) = robots
                .iter()
                .copied()
                .reduce(Pos::add)
                .map(|pos| {
                    (
                        pos.x() as f64 / (robots.len() as f64),
                        pos.y() as f64 / (robots.len() as f64),
                    )
                })
                .unwrap();
            let (x_var, y_var) = robots
                .iter()
                .map(|robot| {
                    (
                        (robot.x() as f64 - x_mean).powf(2.0),
                        (robot.y() as f64 - y_mean).powf(2.0),
                    )
                })
                .unzip::<_, _, Vec<_>, Vec<_>>();

            let x_var = x_var.iter().fold(0.0, f64::add) / x_var.len() as f64;
            let y_var = y_var.iter().fold(0.0, f64::add) / y_var.len() as f64;
            (x_var, y_var)
        })
        .collect();

    let x_offset = sample
        .iter()
        .enumerate()
        .min_by_key(|(_, (variance, _))| OrderedFloat(*variance))
        .unwrap()
        .0;
    let y_offset = sample
        .iter()
        .enumerate()
        .min_by_key(|(_, (_, variance))| OrderedFloat(*variance))
        .unwrap()
        .0;

    successors(Some(x_offset), |it| Some(*it + WIDTH as usize))
        .find(|it| (*it as i64 - y_offset as i64).rem_euclid(HEIGHT) == 0)
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(24, 14).unwrap();
    assert_eq!(210587128, part1(&input));
    assert_eq!(7286, part2(&input));
}

// Input parsed (19μs)
// 1. 210587128 (68μs)
// 2. 7286 (346μs)
// Total: 435μs
