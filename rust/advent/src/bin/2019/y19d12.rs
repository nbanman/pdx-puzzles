use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashSet};
use std::collections::HashSet;
use utilities::{
    math::formulae::lcm,
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<Moon>;
type Output = usize;
type Pos = Coord<i64, 3>;
type Moon = (Pos, Pos);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(x, y, z)| (Pos::new3d(x, y, z), Pos::origin()))
        .collect()
}

fn total_energy(pos: Pos, vel: Pos) -> Output {
    pos.manhattan_distance(Pos::origin()) * vel.manhattan_distance(Pos::origin())
}

fn apply_force(a: i64, b: i64) -> i64 {
    match a - b {
        it if it < 0 => 1,
        it if it > 0 => -1,
        _ => 0,
    }
}

fn part1(mut moons: Input) -> Output {
    for _ in 0..1000 {
        moons = moons
            .iter()
            .map(|&moon| {
                let (pos, vel) = moon;
                let new_vel = moons.iter().filter(|&&cmp| moon != cmp).fold(
                    Pos::origin(),
                    |acc, &(other, _)| {
                        let vel_delta = Pos::new3d(
                            apply_force(pos.x(), other.x()),
                            apply_force(pos.y(), other.y()),
                            apply_force(pos.z(), other.z()),
                        );
                        acc + vel_delta
                    },
                );
                (pos + vel + new_vel, vel + new_vel)
            })
            .collect();
    }

    moons
        .into_iter()
        .map(|(pos, vel)| total_energy(pos, vel))
        .sum()
}

fn get_period(moons: &Input, axis: fn(&Coord<i64, 3>) -> i64) -> i64 {
    let mut positions = moons
        .iter()
        .map(|&(pos, vel)| (axis(&pos), axis(&vel)))
        .collect_vec();
    let mut visited: HashSet<Vec<(i64, i64)>, FxBuildHasher> = FxHashSet::default();
    let mut counter = 0;
    loop {
        positions = positions
            .iter()
            .map(|&moon| {
                let (pos, vel) = moon;
                let new_vel = positions
                    .iter()
                    .filter(|&&other| moon != other)
                    .fold(0, |acc, &(other_pos, _)| acc + apply_force(pos, other_pos));
                (pos + vel + new_vel, vel + new_vel)
            })
            .collect();
        if !visited.insert(positions.clone()) {
            break;
        }
        counter += 1;
    }
    counter
}

fn part2(moons: Input) -> Output {
    [
        get_period(&moons, Pos::x),
        get_period(&moons, Pos::y),
        get_period(&moons, Pos::z),
    ]
    .into_iter()
    .reduce(|acc, period| lcm(acc, period))
    .map(|period| period.abs())
    .unwrap() as Output
}

#[test]
fn default() {
    let input = get_input(19, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(10028, part1(input.clone()));
    assert_eq!(314610635824376, part2(input));
}

// Input parsed (12μs)
// 1. 10028 (45μs)
// 2. 314610635824376 (60ms)
// Total: 60ms
