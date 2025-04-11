use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::{minmax::minmax, parsing::get_numbers::ContainsNumbers, structs::{coord::Coord2U, stopwatch::{ReportDuration, Stopwatch}}};

type Int = usize;
type Pos = Coord2U;
type Line = (Pos, Pos);
type Input = Vec<Line>;
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 5).unwrap();
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
        .map(|(x1, y1, x2, y2)| {
            (Pos::new2d(x1, y1), Pos::new2d(x2, y2))
        })
        .collect()
}

fn straight_range(line: &Line, diagonals: bool) -> Vec<Pos> {
    let (start, end) = line;
    if start.x() == end.x() {
        let (&small, &large) = minmax(&start.y(), &end.y());
        (small..=large).map(|y| Pos::new2d(start.x(), y)).collect()
    } else if start.y() == end.y() {
        let (&small, &large) = minmax(&start.x(), &end.x());
        (small..=large).map(|x| Pos::new2d(x, start.y())).collect()
    } else if diagonals {
        let x_range = if start.x() < end.x() {
            Box::new(start.x()..=end.x()) as Box<dyn Iterator<Item = Int>>
        } else {
            Box::new((end.x()..=start.x()).rev()) as Box<dyn Iterator<Item = Int>>
        };
        let y_increment = if start.y() < end.y() { 1 } else { -1 };
        x_range.enumerate()
            .map(|(i, x)| {
                let i = i as i32;
                Pos::new2d(x, (start.y() as i32 + i * y_increment) as Int)
            })
            .collect()
    } else {    
        Vec::new()
    }
}

fn solve(lines: &Input, diagonals: bool) -> Int {
    let mut frequency = FxHashMap::default();
    for line in lines {
        for pos in straight_range(line, diagonals) {
            frequency
                .entry(pos)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    frequency.iter().filter(|&(_, &count)| count >= 2).count()
}

fn part1(lines: &Input) -> Output {
    solve(lines, false)
}

fn part2(lines: &Input) -> Output {
    solve(lines, true)
}

#[test]
fn default() {
    let input = get_input(21, 5).unwrap();
    let input = parse_input(&input);
    assert_eq!(5774, part1(&input));
    assert_eq!(18423, part2(&input));
}

// Input parsed (46μs)
// 1. 5774 (4ms)
// 2. 18423 (7ms)
// Total: 11ms