use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::{enums::cardinals::Cardinal, parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (FxHashMap<Pos, Ground>, usize);
type Output = usize;
type Pos = Coord2U;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Ground { Clay, StillWater, MovingWater, }

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(18, 17).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut cavern: FxHashMap<Pos, Ground> = input.lines()
        .flat_map(|line| {
            let (fixed, low, high) = line.get_numbers().collect_tuple().unwrap();
            let positions: Vec<_> = if line.chars().next() == Some('x') {
                (low..=high).map(|y| Pos::new2d(fixed, y)).collect()
            } else {
                (low..=high).map(|x| Pos::new2d(x, fixed)).collect()
            };
            positions.into_iter().map(|pos| (pos, Ground::Clay))
        })
        .collect();

    let depth = cavern.keys()
        .map(|pos| pos.y())
        .max_by_key(|depth| *depth)
        .unwrap();

    let start = Pos::new2d(500, 1);

    let first_clay_depth = cavern.keys()
        .map(|pos| pos.y())
        .min()
        .unwrap();

    let mut still_running: FxHashSet<Pos> = FxHashSet::default();
    still_running.insert(start);
    while !still_running.is_empty() {
        still_running = still_running.into_iter()
            .flat_map(|pos| {
                let below = pos.move_direction(Cardinal::South, 1).unwrap();
                let mut next = Vec::new();
                if cavern.get(&below).unwrap_or(&Ground::MovingWater) == &Ground::MovingWater {
                    cavern.insert(pos, Ground::MovingWater);
                    if below.y() <= depth {
                        next.push(below);
                    }
                } else {
                    let mut left: Vec<Pos> = successors(
                        Some(pos.move_direction(Cardinal::West, 1).unwrap()),
                        |&next| {
                            next.move_direction(Cardinal::West, 1)
                        })
                                .take_while_inclusive(|next| {
                                    let south = next.move_direction(Cardinal::South, 1).unwrap();
                                    let south = cavern.get(&south);
                                    cavern.get(next) != Some(&Ground::Clay)
                                        && south.is_some()
                                        && south.unwrap() != &Ground::MovingWater
                                })
                                .collect();

                    let left_wall = cavern.get(&left.last().unwrap()) == Some(&Ground::Clay);
                    if left_wall {
                        left.pop();
                    } else {
                        if cavern.get(&left.last().unwrap().move_direction(Cardinal::South, 1).unwrap()) == Some(&Ground::MovingWater) {
                            left.clear();
                        }
                    }
                    let mut right: Vec<Pos> = successors(
                        Some(pos),
                        |&next| {
                            next.move_direction(Cardinal::East, 1)
                        })
                                .take_while_inclusive(|next| {
                                    let south = next.move_direction(Cardinal::South, 1).unwrap();
                                    let south = cavern.get(&south);
                                    cavern.get(next) != Some(&Ground::Clay)
                                        && south.is_some()
                                        && south.unwrap() != &Ground::MovingWater
                                })
                                .collect();
                    let right_wall = cavern.get(&right.last().unwrap()) == Some(&Ground::Clay);
                    if right_wall {
                        right.pop();
                    } else {
                        if cavern.get(&right.last().unwrap().move_direction(Cardinal::South, 1).unwrap()) == Some(&Ground::MovingWater) {
                            right.clear();
                        }
                    }

                    if left_wall && right_wall {
                       left.iter().for_each(|&it| { cavern.insert(it, Ground::StillWater); });
                       right.iter().for_each(|&it| { cavern.insert(it, Ground::StillWater); });
                       next.push(pos.move_direction(Cardinal::North, 1).unwrap());
                    } else {
                       left.iter().for_each(|&it| { cavern.insert(it, Ground::MovingWater); });
                       right.iter().for_each(|&it| { cavern.insert(it, Ground::MovingWater); });
                       if !left_wall && !left.is_empty() {
                           next.push(left.last().unwrap().move_direction(Cardinal::South, 1).unwrap());
                       }
                       if !right_wall && !right.is_empty() {
                           next.push(right.last().unwrap().move_direction(Cardinal::South, 1).unwrap());
                       }
                    }
                }
                next.into_iter()
            })
            .collect();
    }
    (cavern, first_clay_depth)
}

fn part1(input: &Input) -> Output {
    let (cavern, first_clay_depth) = input;
    cavern.into_iter()
        .filter(|&(pos, &ground)| {
            pos.y() >= *first_clay_depth && ground != Ground::Clay
        })
        .count()
}

fn part2(input: &Input) -> Output {
    let (cavern, _) = input;
    cavern.values().filter(|&&it| it == Ground::StillWater).count()
}

#[test]
fn default() {
    let input = get_input(18, 17).unwrap();
    let input = parse_input(&input);
    assert_eq!(40879, part1(&input));
    assert_eq!(34693, part2(&input));
}

// Input parsed (5ms)
// 1. 40879 (102μs)
// 2. 34693 (94μs)
// Total: 5ms