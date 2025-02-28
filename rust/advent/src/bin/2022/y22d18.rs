use std::{collections::VecDeque, ops::RangeInclusive};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{
    parsing::get_numbers::ContainsNumbers,
    structs::{
        coord::Coord,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = FxHashSet<Cube>;
type Output = usize;
type Cube = Coord<i64, 3>;

const CROSS: [Cube; 6] = [
    Coord([1, 0, 0]),
    Coord([-1, 0, 0]),
    Coord([0, 1, 0]),
    Coord([0, -1, 0]),
    Coord([0, 0, 1]),
    Coord([0, 0, -1]),
];

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 18).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .get_numbers()
        .tuples()
        .map(|(x, y, z)| Cube::new3d(x, y, z))
        .collect()
}

fn adjacent(cube: Cube) -> impl Iterator<Item = Cube> {
    CROSS.into_iter().map(move |adj| cube + adj)
}

fn surface_area<F>(cubes: &FxHashSet<Cube>, predicate: F) -> usize
where
    F: Fn(&Cube) -> bool,
{
    cubes
        .iter()
        .map(|&cube| {
            adjacent(cube)
                .filter(|neighbor| predicate(neighbor))
                .count()
        })
        .sum()
}

fn part1(cubes: &Input) -> Output {
    surface_area(cubes, |cube| !cubes.contains(cube))
}

fn get_bounds(cubes: &Input) -> [RangeInclusive<i64>; 3] {
    let mut maximums = [i64::MIN; 3];
    let mut minimums = [i64::MAX; 3];
    for cube in cubes {
        for dim in 0..3 {
            let v = cube.0[dim];
            if v > maximums[dim] {
                maximums[dim] = v;
            } else if v < minimums[dim] {
                minimums[dim] = v;
            }
        }
    }
    [
        minimums[0] - 1..=maximums[0] + 1,
        minimums[1] - 1..=maximums[2] + 1,
        minimums[1] - 1..=maximums[2] + 1,
    ]
}

fn part2(cubes: &Input) -> Output {
    let [x_bounds, y_bounds, z_bounds] = get_bounds(cubes);
    let mut exterior: FxHashSet<Cube> = FxHashSet::default();
    let start = Cube::new3d(*x_bounds.start(), *y_bounds.start(), *z_bounds.start());
    let mut q = VecDeque::new();
    q.push_back(start);
    exterior.insert(start);
    while let Some(cube) = q.pop_front() {
        let neighbors: Vec<_> = adjacent(cube)
            .filter(|neighbor| {
                !cubes.contains(neighbor)
                    && !exterior.contains(neighbor)
                    && x_bounds.contains(&neighbor.x())
                    && y_bounds.contains(&neighbor.y())
                    && z_bounds.contains(&neighbor.z())
            })
            .collect();
        for neighbor in neighbors {
            exterior.insert(neighbor);
            q.push_back(neighbor);
        }
    }
    surface_area(cubes, |cube| exterior.contains(cube))
}

#[test]
fn default() {
    let input = get_input(22, 18).unwrap();
    let input = parse_input(&input);
    assert_eq!(4332, part1(&input));
    assert_eq!(2524, part2(&input));
}

// Input parsed (233μs)
// 1. 4332 (160μs)
// 2. 2524 (1ms)
// Total: 1ms
