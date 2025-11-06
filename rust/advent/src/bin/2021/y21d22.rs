use std::{cmp::{max, min}, ops::Range};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{parsing::get_numbers::ContainsNumbers, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input = Vec<Cuboid>;
type Output = usize;

#[derive(Debug, Clone)]
struct Cuboid {
    turn_on: bool,
    x: Range<i64>,
    y: Range<i64>,
    z: Range<i64>,
}

impl Cuboid {
    fn in_range(&self) -> bool {
        let range = -50..51;
        [&self.x, &self.y, &self.z].iter().all(|&axis| {
            axis.start >= range.start && axis.end <= range.end
        })
    }

    fn volume(&self) -> usize {
        if self.turn_on {
            self.x.clone().count() * self.y.clone().count() * self.z.clone().count()
        } else {
            0
        }
    }

    fn disjoint_cubes(&self, other: Cuboid) -> Vec<Cuboid> {
        let Some(overlap) = self.intersect(&other) else {
            return vec![other];
        };
        let mut disjoint_cubes: Vec<Cuboid> = Vec::new();
        
        let disjoint_x = Self::disjoint_ranges(&self.x, &other.x).into_iter()
            .map(|x| Cuboid { turn_on: true, x, y: other.y.clone(), z: other.z.clone() });
        disjoint_cubes.extend(disjoint_x);
        
        let disjoint_y = Self::disjoint_ranges(&self.y, &other.y).into_iter()
            .map(|y| Cuboid { turn_on: true, x: overlap.x.clone(), y, z: other.z.clone() }); 
        disjoint_cubes.extend(disjoint_y);

        let disjoint_z = Self::disjoint_ranges(&self.z, &other.z).into_iter()
            .map(|z| Cuboid { turn_on: true, x: overlap.x.clone(), y: overlap.y.clone(), z }); 
        disjoint_cubes.extend(disjoint_z);

        disjoint_cubes
    }

    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let x = Self::union(&self.x, &other.x)?;
        let y = Self::union(&self.y, &other.y)?;
        let z = Self::union(&self.z, &other.z)?;
        Some(Cuboid { turn_on: true, x, y, z })
    }

    fn union(a: &Range<i64>, b: &Range<i64>) -> Option<Range<i64>> {
        if a.end >= b.start && a.start <= b.end {
            let new = max(a.start, b.start)..min(a.end, b.end);
            Some(new)
        } else {
            None
        }
    }

    fn disjoint_ranges(a: &Range<i64>, b: &Range<i64>) -> Vec<Range<i64>> {
        let mut disjoints = Vec::new();
        if b.start < a.start {
            disjoints.push(b.start ..a.start);
        }
        if b.end > a.end {
            disjoints.push(a.end .. b.end);
        }
        disjoints
    }
}


fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 22).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.lines()
        .map(|line| {
            let turn_on = line.starts_with("on");
            let (x1, x2, y1, y2, z1, z2) = line.get_numbers().collect_tuple().unwrap();
            Cuboid { turn_on, x: x1..x2 + 1, y: y1..y2 + 1, z: z1..z2 + 1 }
        })
        .collect()
}

fn find_cubes(cuboids: Input) -> Output {
    let mut visited: Vec<Cuboid> = Vec::with_capacity(cuboids.len());
    for cuboid in cuboids {
        visited = visited.into_iter()
            .flat_map(|prior| cuboid.disjoint_cubes(prior))
            .collect();
        if cuboid.turn_on {
            visited.push(cuboid);
        }
    }
    visited.into_iter()
        .map(|cuboid| cuboid.volume())
        .sum()
}

fn part1(cuboids: Input) -> Output {
    let cuboids = cuboids.into_iter().filter(|it| it.in_range()).collect();
    find_cubes(cuboids)
}

fn part2(cuboids: Input) -> Output {
    find_cubes(cuboids)
}

#[test]
fn default() {
    let input = get_input(21, 22).unwrap();
    let input = parse_input(&input);
    assert_eq!(587097, part1(input.clone()));
    assert_eq!(1359673068597669, part2(input));
}

// Input parsed (109μs)
// 1. 587097 (87μs)
// 2. 1359673068597669 (10ms)
// Total: 10ms