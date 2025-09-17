use advent::utilities::get_input::get_input;
use utilities::structs::coord::Coord2U;
use utilities::structs::grid::Grid2;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = Grid2<bool>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(20, 3).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let slope = Grid2::new2d_map_str(input, |c| c == '#').unwrap();
    slope
}

fn solve(slope: &Input, right: usize, down: usize) -> Output {
    (down..slope.height()).step_by(down).enumerate()
        .fold(0, |acc, (idx, y)| {
            acc + if slope[Coord2U::new2d((idx + 1) * right % slope.width(), y)] { 1 } else { 0 }
        })
}

fn part1(input: &Input) -> Output {
    solve(input, 3, 1)
}

fn part2(input: &Input) -> Output {
    [
        solve(input, 1, 1),
        solve(input, 3, 1),
        solve(input, 5, 1),
        solve(input, 7, 1),
        solve(input, 1, 2),
    ]
        .into_iter()
        .fold(1, |acc, x| acc * x)
}

#[test]
fn default() {
    let input = get_input(20, 3).unwrap();
    let input = parse_input(&input);
    assert_eq!(294, part1(&input));
    assert_eq!(5774564250, part2(&input));
}

// Input parsed (61μs)
// 1. 294 (23μs)
// 2. 5774564250 (78μs)
// Total: 164μs