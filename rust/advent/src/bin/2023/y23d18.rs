use advent::utilities::get_input::get_input;
use utilities::{
    enums::cardinals::Cardinal,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

type Input<'a> = &'a str;
type Output = i64;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 18).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(plans: Vec<(Cardinal, i64)>) -> i64 {
    let (mut y, mut area, mut perimeter) = (0i64, 0i64, 0i64);
    for (dir, dist) in plans {
        match dir {
            Cardinal::North => y -= dist,
            Cardinal::South => y += dist,
            Cardinal::West => area -= y * dist,
            Cardinal::East => area += y * dist,
        }
        perimeter += dist;
    }
    perimeter / 2 + 1 + area.unsigned_abs() as i64
}

fn part1(input: Input) -> Output {
    let plans: Vec<(Cardinal, i64)> = input
        .lines()
        .map(|s| {
            let (dir_str, rest) = s.split_once(' ').unwrap();
            let (dist_str, _) = rest.split_once(' ').unwrap();
            let dir = match dir_str {
                "U" => Cardinal::North,
                "D" => Cardinal::South,
                "L" => Cardinal::West,
                "R" => Cardinal::East,
                x => panic!("String must begin with [UDLR], but begins with {}.", x),
            };
            let dist: i64 = dist_str.parse().unwrap();
            (dir, dist)
        })
        .collect();
    solve(plans)
}

fn part2(input: Input) -> Output {
    let plans: Vec<(Cardinal, i64)> = input
        .lines()
        .filter_map(|s| {
            let (_, color) = s.split_once('#')?;
            let dir = &color[5..color.len() - 1];
            let dir = match dir {
                "0" => Cardinal::East,
                "1" => Cardinal::South,
                "2" => Cardinal::West,
                "3" => Cardinal::North,
                x => panic!("String must begin with [0-4], but begins with {}.", x),
            };
            let dist = i64::from_str_radix(&color[..color.len() - 2], 16).ok()?;
            Some((dir, dist))
        })
        .collect();
    solve(plans)
}

#[test]
fn default() {
    let input = get_input(23, 18).unwrap();
    assert_eq!(50746, part1(&input));
    assert_eq!(70086216556038, part2(&input));
}

// Input parsed (17μs)
// 1. 50746 (28μs)
// 2. 70086216556038 (31μs)
// Total: 78μs
