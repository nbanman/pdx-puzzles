use std::{collections::{HashMap, HashSet}, iter::successors};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{coord::Coord, stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;
type Pos = Coord<isize, 2>;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 8).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve<F>(city_limits: Input, get_antinodes: &F) -> Output 
where 
    F: Fn((&Pos, &Pos)) -> Vec<Pos>,
{
    get_antennae(city_limits)
        .iter()
        .flat_map(|positions| {
            positions.iter().tuple_combinations::<(_, _)>().flat_map(get_antinodes)
        })
        .collect::<HashSet<_>>()
        .len()
}

fn within_city_limits(pos: &Pos, width: usize, height: usize) -> bool {
    (0..width as isize - 1).contains(&pos.x()) 
                    && (0..height as isize ).contains(&pos.y())
}

fn part1(city_limits: Input) -> Output {
    let width = city_limits.find('\n').unwrap() + 1;
    let height = (city_limits.len() + 1) / width;
    let get_antinodes = |(&a, &b): (&Pos, &Pos)| {
        let diff = a - b;
        [a + diff, b - diff].into_iter()
            .filter(|antinode| within_city_limits(antinode, width, height))
            .collect::<Vec<Pos>>()
    };
    
    solve(city_limits, &get_antinodes)
}

fn part2(city_limits: Input) -> Output {
    let width = city_limits.find('\n').unwrap() + 1;
    let height = (city_limits.len() + 1) / width;
    let get_antinodes = |(&a, &b): (&Pos, &Pos)| {
        let diff = a - b;
        let mut a_ray: Vec<Pos> = ray(a, &diff, width, height).collect();
        let neg_diff = -diff;
        let b_ray = ray(b, &neg_diff, width, height);
        a_ray.extend(b_ray);
        a_ray
    };
    
    solve(city_limits, &get_antinodes)
}

fn ray(start: Pos, diff: &Pos, width: usize, height: usize) -> impl Iterator<Item = Pos> + use<'_> {
    successors(Some(start), move |pos| {
        let next = *pos + *diff;
        if within_city_limits(&next, width, height) {
            Some(next)
        } else {
            None
        }
    })
}

fn get_antennae(city_limits: Input) -> Vec<Vec<Pos>> {
    let width = city_limits.find('\n').unwrap() + 1;
    let mut antennae = HashMap::new();
    for (index, roof) in city_limits.chars().enumerate() {
        if roof.is_ascii_alphanumeric() {
            let pos = Pos::from_index(index, width).unwrap();
            antennae.entry(roof).or_insert(Vec::new()).push(pos);
        }
    }
    antennae.values().cloned().collect()
}

#[test]
fn default() {
    let input = get_input(24, 8).unwrap();
    assert_eq!(228, part1(&input));
    assert_eq!(766, part2(&input));
}

// Input parsed (21μs)
// 1. 228 (78μs)
// 2. 766 (141μs)
// Total: 245μs

#[test]
fn examples() {
    let inputs = [r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
", ];
    assert_eq!(14, part1(inputs[0]));
    // assert_eq!(34, part2(&input[0]));
}