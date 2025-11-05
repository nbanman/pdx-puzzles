use everybody_codes::utilities::inputs::get_event_inputs;
use itertools::Itertools;
use utilities::parsing::get_numbers::ContainsNumbers;
use utilities::structs::coord::Coord2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input<'a> = &'a str;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let (input1, input2, input3) = get_event_inputs(25, 2);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input1), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input2), stopwatch.lap().report());
    println!("3. {} ({})", part3(&input3), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse(input: Input) -> Pos {
    let (x, y) = input.get_numbers().collect_tuple().unwrap();
    Pos::from((x, y))
}
fn square(a: Pos) -> Pos {
    let x = a.x() * a.x() - a.y() * a.y();
    let y = a.x() * a.y() + a.x() * a.y();
    Pos::from((x,y))
}

fn solve(input: Input, step: usize) -> usize {
    let tl = parse(input);
    let br = tl + 1000;
    let mut engraved_points = 0;
    for y in (tl.y()..=br.y()).step_by(step) {
        for x in (tl.x()..=br.x()).step_by(step) {
            let point = Pos::from((x, y));
            if engravable_point(100, point, 100_000).is_some() {
                engraved_points += 1;
            }
        }
    }
    engraved_points
}

fn engravable_point(cycles: usize, point: Pos, divisor: i64) -> Option<Pos> {
    let range = -1000000 ..= 1000000;
    let mut acc = Pos::origin();
    for _ in 0..cycles {
        acc = point + square(acc) / divisor;
        if !range.contains(&acc.x()) || !range.contains(&acc.y()) {
            return None;
        }
    }
    Some(acc)
}

fn part1(input: Input) -> String {
    let point = parse(input);
    let ans = engravable_point(3, point, 10).unwrap();
    format!("[{},{}]", ans.x(), ans.y())
}

fn part2(input: Input) -> usize {
    solve(input, 10)
}

fn part3(input: Input) -> usize {
    solve(input, 1)
}

#[test]
fn default() {
    let (input1, input2, input3) = get_event_inputs(25, 2);
    assert_eq!("[206456,960631]".to_string(), part1(&input1));
    assert_eq!(1367, part2(&input2));
    assert_eq!(134600, part3(&input3));
}

// Input parsed (26μs)
// 1. [206456,960631] (7μs)
// 2. 1367 (1ms)
// 3. 134600 (156ms)
// Total: 158ms