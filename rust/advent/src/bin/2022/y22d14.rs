use std::{cmp::max, iter::successors};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (Cavern, i64);
type Output = usize;
type Pos = Coord2;
type Cavern = FxHashSet<Pos>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 14).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut cavern = Cavern::default();
    for line in input.lines() {
        for (prev, next) in line
            .get_numbers()
            .tuples()
            .map(|(x, y)| Pos::new2d(x, y))
            .tuple_windows()
        {
            let x_delta = (next.x() - prev.x()).signum();
            let y_delta = (next.y() - prev.y()).signum();
            let delta = Pos::new2d(x_delta, y_delta);
            let steps = max((next.x() - prev.x()).abs(), (next.y() - prev.y()).abs()) as usize + 1;
            for pos in successors(Some(prev), |&state| {
                Some(state + delta)
            })
                .take(steps) 
            {
                cavern.insert(pos);
            }
        }
    }

    let depth = cavern.iter().map(|pos| pos.y()).max().unwrap();
        
    (cavern, depth)
}

fn solve<F>(cavern: &mut Cavern, depth: i64, predicate: F) -> usize
    where
        F: Fn(Pos) -> bool
{
    let mut index = 0usize;
    let options: Vec<Pos> = vec![Pos::new2d(0, 1), Pos::new2d(-1, 1), Pos::new2d(1, 1)];
    loop {
        let grain = settle(cavern, depth, &options);
        if predicate(grain) { return index; }
        cavern.insert(grain);
        index += 1;
    }
}

fn fall(cavern: &Cavern, grain: Pos, options: &Vec<Pos>) -> Option<Pos> {
    options
        .into_iter()
        .map(|&v| v + grain)
        .find(|v| !cavern.contains(v))
}

fn settle(cavern: &Cavern, depth: i64, options: &Vec<Pos>) -> Pos {
    successors(Some(Pos::new2d(500, 0)), |&grain| fall(cavern, grain, options))
        .take_while(|grain| grain.y() <= depth + 1)
        .last()
        .unwrap()
}

fn part1((mut cavern, depth): Input) -> Output {
    solve(&mut cavern, depth, |v| v.y() > depth)
}

fn part2((mut cavern, depth): Input) -> Output {
    let top = Pos::new2d(500, 0);
    solve(&mut cavern, depth, |v| v == top) + 1
}

#[test]
fn default() {
    let input = get_input(22, 14).unwrap();
    let input = parse_input(&input);
    assert_eq!(825, part1(input.clone()));
    assert_eq!(26729, part2(input));
}

// Input parsed (162μs)
// 1. 825 (390μs)
// 2. 26729 (15ms)
// Total: 16ms