use std::collections::HashSet;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = (Vec<Pos>, Vec<Pos>, Vec<Pos>);
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let (wires1, wires2) = input
        .lines()
        .map(|line| {
            let mut wires = Vec::new();
            let mut last = Coord2::origin();
            for instruction in line.split(',') {
                let (dir, num) = instruction.split_at(1);
                let dir = match dir {
                    "R" => Cardinal::East,
                    "L" => Cardinal::West,
                    "U" => Cardinal::North,
                    "D" => Cardinal::South,
                    c => {
                        panic!("{c} not a valid direction")
                    }
                };
                let num: usize = num.parse().unwrap();
                for i in 1..num {
                    wires.push(last.move_direction(dir, i as i64).unwrap());
                }
                last = last.move_direction(dir, num as i64).unwrap();
                wires.push(last.clone());
            }
            wires
        })
        .collect_tuple()
        .unwrap();
    let temp_set: HashSet<Pos> = wires1.iter().cloned().collect();
    let intersections = wires2
        .iter()
        .filter(|&wire| temp_set.contains(wire))
        .cloned()
        .collect();
    (intersections, wires1, wires2)
}

fn part1(input: &Input) -> Output {
    let (intersections, _, _) = input;
    intersections
        .iter()
        .map(|pos| pos.manhattan_distance(Pos::origin()))
        .min()
        .unwrap()
}

fn part2(input: &Input) -> Output {
    let (intersections, wires1, wires2) = input;
    intersections
        .iter()
        .map(|pos| {
            let val1 = wires1.iter().position(|other| pos == other).unwrap() + 1;
            let val2 = wires2.iter().position(|other| pos == other).unwrap() + 1;
            val1 + val2
        })
        .min()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(19, 3).unwrap();
    let input = parse_input(&input);
    assert_eq!(266, part1(&input));
    assert_eq!(19242, part2(&input));
}

// Input parsed (8ms)
// 1. 266 (5μs)
// 2. 19242 (3ms)
// Total: 12ms
