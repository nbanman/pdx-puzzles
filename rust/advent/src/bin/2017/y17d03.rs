use std::cmp::{max, min};

use advent::utilities::get_input::get_input;
use rustc_hash::FxHashMap;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = usize;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

fn part1(square: Input) -> Output {
    let square_root = (square as f64).sqrt().ceil() as usize;
    let square_root = square_root + if square_root & 1 == 0 { 1 } else { 0 };
    let farthest = square_root / 2 * 2;
    let br = square_root * square_root;
    let diff = (br - square) % farthest;
    farthest - min(diff, farthest / 2) + max(0, diff - farthest / 2)
}

struct Turtle {
    dir: Cardinal,
    pos: Pos,
    vel: usize,
}
fn part2(input: Input) -> Output {
    let mut turtle = Turtle { dir: Cardinal::South, pos: Pos::origin(), vel: 0 };
    let mut space: FxHashMap<Pos, usize> = FxHashMap::default();
    space.insert(Pos::origin(), 1);
    loop {
        turtle.dir = turtle.dir.left();
        if turtle.dir == Cardinal::East || turtle.dir == Cardinal::West {
            turtle.vel += 1;
        }
        for _ in 1..=turtle.vel {
            turtle.pos = turtle.pos.move_direction(turtle.dir, 1).unwrap();
            let square_val: usize = turtle.pos.adjacent(true).into_iter()
                .map(|adj| space.get(&adj).copied().unwrap_or_default())
                .sum();
            if square_val > input {
                return square_val;
            }
            space.insert(turtle.pos, square_val);
        }
    }
}


#[test]
fn default() {
    let input = get_input(17, 3).unwrap();
    let input = parse_input(&input);
    assert_eq!(552, part1(input));
    assert_eq!(330785, part2(input));
}

// Input parsed (12μs)
// 1. 552 (5μs)
// 2. 330785 (11μs)
// Total: 30μs
