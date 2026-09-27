use std::collections::VecDeque;

use everybody_codes::utilities::inputs::get_story_inputs_provisional;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use utilities::{
    parsing::get_numbers::{ContainsNumbers, NumberIterator},
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input<'a> = &'a str;
type Pos = Coord2;

#[derive(Clone, Debug)]
struct Notes {
    start: Pos,
    beacons: Vec<Pos>,
    moves: Vec<usize>,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_story_inputs_provisional(26, 4, 2);
    println!("Input parsed ({})", stopwatch.lap().report());
    if let Some(input1) = input1 {
        println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    }
    if let Some(input2) = input2 {
        println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    }
    if let Some(input3) = input3 {
        println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    }
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> Notes {
    let (_, moves) = input.rsplit_once('=').unwrap();
    let moves: Vec<usize> = if moves.starts_with('[') {
        Vec::new()
    } else {
        moves
            .trim_end()
            .as_bytes()
            .iter()
            .map(|&b| (b - b'A') as usize)
            .collect()
    };

    let mut nums: NumberIterator<'_, i64> = input.get_numbers();
    let start = Pos::new2d(nums.next().unwrap(), nums.next().unwrap());
    let beacons = nums.tuples().map(|(x, y)| Pos::new2d(x, y)).collect();
    Notes {
        start,
        beacons,
        moves,
    }
}

fn part1(input: Input) -> usize {
    let notes = parse(input);
    let mut sparkballs = FxHashSet::default();
    sparkballs.insert(notes.start);

    let mut pos = notes.start;

    for beacon in notes.moves {
        pos = Pos::new2d(
            (pos.x() + notes.beacons[beacon].x()) / 2,
            (pos.y() + notes.beacons[beacon].y()) / 2,
        );
        sparkballs.insert(pos);
    }
    sparkballs.len()
}

fn part2(input: Input) -> usize {
    let notes = parse(input);
    let mut sparkballs = FxHashSet::default();
    sparkballs.insert(notes.start);

    let mut pos = notes.start;

    for beacon in notes.moves {
        pos = Pos::new2d(
            (pos.x() + notes.beacons[beacon].x()) / 2,
            (pos.y() + notes.beacons[beacon].y()) / 2,
        );
        sparkballs.insert(pos);
    }

    let mut fireflies = FxHashSet::default();

    for sparkball in sparkballs.iter() {
        for adj in sparkball.adjacent(false) {
            if !sparkballs.contains(&adj) {
                fireflies.insert(adj);
            }
        }
    }
    fireflies.len()
}

fn part3(input: Input) -> usize {
    let notes = parse(input);
    let mut sparkballs = FxHashSet::default();
    sparkballs.insert(notes.start);

    let mut q = VecDeque::new();
    q.push_back(notes.start);
    while let Some(pos) = q.pop_front() {
        for &beacon in notes.beacons.iter() {
            let next = Pos::new2d((pos.x() + beacon.x()) / 2, (pos.y() + beacon.y()) / 2);
            if !sparkballs.contains(&next) {
                sparkballs.insert(next);
                q.push_back(next);
            }
        }
    }
    let mut fireflies = FxHashSet::default();

    for sparkball in sparkballs.iter() {
        for adj in sparkball.adjacent(false) {
            if !sparkballs.contains(&adj) {
                fireflies.insert(adj);
            }
        }
    }
    fireflies.len()
}

#[test]
fn default() {
    // let (input1, input2, input3) =
    //        everybody_codes::utilities::inputs::get_story_inputs(26, XX, XX);
    // assert_eq!(ZZ, part1(&input1));
    // assert_eq!(ZZ, part2(&input2));
    // assert_eq!(ZZ, part3(&input3));
}

#[test]
fn example() {
    let input1 = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=ABCCBABCA";
    assert_eq!(8, part1(input1));
    assert_eq!(25, part2(input1));
    let input2 = r"START=[5,0]
A=[0,0]
B=[10,0]
C=[5,10]
MOVES=BABCAABBCABCCCBBABCCCAAACABABCBCBBCAABBABBCACCBAABCBCBBBCBBBBBCCCAACAACB";
    assert_eq!(46, part2(input2));
    assert_eq!(42, part3(input1));
    let input3 = r"START=[0,0]
A=[0,0]
B=[80,15]
C=[5,30]";
    assert_eq!(432, part3(input3));
}

// Input parsed (23μs)
// 1. 68 (9μs)
// 2. 1697 (170μs)
// 3. 16842 (4.618ms)
// Total: 4.827ms
