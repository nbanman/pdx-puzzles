use advent::utilities::get_input::get_input;
use rustc_hash::FxHashSet;
use utilities::{enums::cardinals::Cardinal, structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}}};

type Input = Vec<Pos>;
type Output = usize;
type Pos = Coord2;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 1).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let mut dir = Cardinal::North;
    input.split(", ")
        .flat_map(|instruction| {
            dir = if instruction.starts_with('L') { dir.left() } else { dir.right() };
            let dist = instruction[1..].parse().unwrap();
            std::iter::repeat(dir).take(dist)
        })
        .scan(Pos::origin(), |pos, dir| {
            *pos = pos.move_direction(dir, 1).unwrap();
            Some(*pos)
        })
        .collect()
}

fn part1(moves: &Input) -> Output {
    moves.last().unwrap().manhattan_distance(Pos::origin())
}

fn part2(moves: &Input) -> Output {
    let mut visited = FxHashSet::default();
    moves.iter()
        .find(|&&pos| !visited.insert(pos))
        .map(|pos| pos.manhattan_distance(Pos::origin()))
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(16, 1).unwrap();
    let input = parse_input(&input);
    assert_eq!(226, part1(&input));
    assert_eq!(79, part2(&input));
}

// Input parsed (35μs)
// 1. 226 (6μs)
// 2. 79 (24μs)
// Total: 68μs
