use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

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
    input.get_numbers().tuples()
        .map(|(px, py, vx, vy)| Robot { p: Pos::new2d(px, py), v: Pos::new2d(vx, vy) } )
        .collect()
}

fn warp(robots: &[Robot]) -> Vec<Robot> {
    robots.iter()
        .map(|robot| {
            let p = Pos::new2d(
                (robot.p.x() + robot.v.x()).rem_euclid(WIDTH), 
                (robot.p.y() + robot.v.y()).rem_euclid(HEIGHT)
            );
            Robot { p, ..*robot }
        })
        .collect()
}

fn score(robots: Vec<Robot>) -> usize {
    let mut quadrants = [0usize; 4];
    let split_x = WIDTH / 2;
    let split_y = HEIGHT / 2;

    robots.iter()
        .filter_map(|robot| {
            let p = robot.p;
            let x = match p.x() {
                x if (0..split_x).contains(&x) => Some(0),
                x if x == split_x => None,
                _ => Some(2),
            }?;
            let y = match p.y() {
                y if (0..split_y).contains(&y) => Some(0),
                y if y == split_y => None,
                _ => Some(1),
            }?;
            Some((x, y))
        })
        .for_each(|(x, y)| { quadrants[x + y] += 1; });
    
    quadrants.into_iter().reduce(|acc, count| acc * count).unwrap()
}

fn part1(input: Input) -> Output {
    let robots = parse_robots(input);
    let moved_robots = successors(Some(robots), |robots| Some(warp(robots)))
        .take(101)
        .last()
        .unwrap();
    score(moved_robots)
}


fn part2(input: Input) -> Output {
    let robots = parse_robots(input);
    successors(Some(robots), |robots| Some(warp(robots)))
        .map(|robots| robots.iter().map(|robot| robot.p).collect::<Vec<_>>())
        .enumerate()
        .find(|(_, robots)| robots.len() == robots.iter().unique().count())
        .unwrap()
        .0    
}

#[test]
fn default() {
    let input = get_input(24, 14).unwrap();
    assert_eq!(210587128, part1(&input));
    assert_eq!(7286, part2(&input));
}

// Input parsed (20μs)
// 1. 210587128 (257μs)
// 2. 7286 (115ms)
// Total: 116ms
