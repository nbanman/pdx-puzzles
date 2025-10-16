use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        coord::Coord2,
        stopwatch::{ReportDuration, Stopwatch},
    },
};

type Input = Vec<Vec<Cardinal>>;
type Output = String;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 2).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(Cardinal::from).collect())
        .collect()
}

fn solve<F, G>(instructions: &Input, start: Pos, in_bounds: F, conversion: G) -> String
where
    F: Fn(Pos) -> bool,
    G: Fn(Pos) -> String,
{
    instructions
        .iter()
        .scan(start, |pos, instruction| {
            *pos = instruction.iter().fold(*pos, |acc, &dir| {
                let next = acc.move_direction(dir, 1).unwrap();
                if in_bounds(next) { next } else { acc }
            });
            Some(conversion(*pos))
        })
        .join("")
}

fn part1(instructions: &Input) -> Output {
    let start = Pos::new2d(1, 1);
    let in_bounds = |pos: Pos| pos.chebyshev_distance(start) < 2;
    let to_numpad = |pos: Pos| (pos.y() * 3 + pos.x() + 1).to_string();
    solve(instructions, start, in_bounds, to_numpad)
}

fn part2(instructions: &Input) -> Output {
    let start = Pos::new2d(0, 2);
    let mid = Pos::new2d(2, 2);
    let in_bounds = |pos: Pos| pos.manhattan_distance(mid) < 3;
    let to_numpad = |pos: Pos| {
        let key = 5 + pos.x() + (pos.y() - 2) * 2 + 2 * (pos.y() - 2).signum();
        format!("{:X}", key)
    };
    solve(instructions, start, in_bounds, to_numpad)
}

#[test]
fn default() {
    let input = get_input(16, 2).unwrap();
    let input = parse_input(&input);
    assert_eq!("92435".to_string(), part1(&input));
    assert_eq!("C1A88".to_string(), part2(&input));
}

// Input parsed (22μs)
// 1. 92435 (26μs)
// 2. C1A88 (24μs)
// Total: 75μs
