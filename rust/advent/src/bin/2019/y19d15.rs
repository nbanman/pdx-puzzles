use std::mem;
use advent::utilities::{get_input::get_input, intcode::{IntCode, State}};
use rustc_hash::FxHashSet;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = (FxHashSet<Pos>, Pos);
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(19, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut compy = IntCode::from(input);
    let mut ship = FxHashSet::default();
    let start_pos = Pos::origin();
    let mut o2 = start_pos;
    ship.insert(start_pos);
    let mut pos = start_pos;
    let mut dir = Cardinal::North;
    let mut attempt = 0;
    loop {
        dir = if attempt == 0 {
            dir.right()
        } else {
            dir.left()
        };
        attempt += 1;
        compy.input(dir_to_input(dir));
        let State::Output(output) = compy.run() else {
            panic!("Computer failed to return output")
        };
        if output == 0 { continue; }

        attempt = 0;
        pos = pos.move_direction(dir, 1).unwrap();
        ship.insert(pos);
        if output == 2 {
            o2 = pos;
        }
        if pos == start_pos {
            break;
        }
    }
    (ship, o2)
}

fn dir_to_input(dir: Cardinal) -> i64 {
    match dir {
        Cardinal::North => 1,
        Cardinal::East => 4,
        Cardinal::South => 2,
        Cardinal::West => 3,
    }
}

fn solve<F>(mut ship: FxHashSet<Pos>, start: Pos, end_condition: F) -> Output
where
    F: Fn(Pos) -> bool,
{
    ship.remove(&start);
    let mut steps = 0;
    let mut todo = Vec::new();
    todo.push(start);
    let mut next = Vec::new();
    while !todo.is_empty() {
        for pos in todo.drain( .. ) {
            for neighbor in Pos::adjacent(&pos, false) {
                if ship.remove(&neighbor) {
                    if end_condition(neighbor) {
                        return steps + 1;
                    }
                    next.push(neighbor);
                }
            }
        }
        mem::swap(&mut todo, &mut next);
        steps += 1;
    }
    steps - 1
}

fn part1(input: &Input) -> Output {
    let (ship, o2) = input;
    let ship = ship.clone();
    solve(ship, Pos::origin(), |pos| pos == *o2)
}

fn part2(input: &Input) -> Output {
    let (ship, o2) = input;
    let ship = ship.clone();
    solve(ship, *o2, |_| false)
}

#[test]
fn default() {
    let input = get_input(19, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(250, part1(&input));
    assert_eq!(332, part2(&input));
}

// Input parsed (663μs)
// 1. 250 (28μs)
// 2. 332 (57μs)
// Total: 751μs