use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{enums::cardinals::Cardinal, structs::{stopwatch::{ReportDuration, Stopwatch}, strgrid::{adjacent_metadata::AdjacentMetadata, str_grid::StrGrid}}};

type Input = Vec<Cardinal>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 10).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let field = StrGrid::new(input).unwrap();
    let start_pos = input.find('S').unwrap();
    let north = field
        .move_direction(start_pos, Cardinal::North)
        .unwrap();
    let start_dir = if north.b == b'7' || north.b == b'F' || north.b == b'|' {
        Cardinal::North
    } else {
        let east = field
            .move_direction(start_pos, Cardinal::East)
            .unwrap();
        if east.b == b'7' || east.b == b'J' || east.b == b'-' {
            Cardinal::East
        } else {
            Cardinal::South
        }
    };

    let move_along_pipe = |pos, dir| {
        let new_pos: AdjacentMetadata<usize> = field.move_direction(pos, dir).unwrap();
        let new_dir = match new_pos.b {
            b'L' | b'7' => if dir.ordinal() & 1 == 1 { dir.right() } else { dir.left() },
            b'J' | b'F' => if dir.ordinal() & 1 == 0 { dir.right() } else { dir.left() },   
            _ => dir,
        };
        (new_pos, new_dir)
    };

    let pipe = successors(Some(move_along_pipe(start_pos, start_dir)), |(next_pos, next_dir)| {
        Some(move_along_pipe(next_pos.pos, *next_dir))
    })
        .take_while_inclusive(|(next_pos, _)| {
            next_pos.b != b'S'
        })
        .map(|(_, dir)| dir)
        .collect();

    pipe
}

fn part1(pipe: &Input) -> Output {
    pipe.len() / 2
}

fn part2(pipe: &Input) -> Output {
    let area: i32 = pipe.iter()
        .fold((0, 0), |(sum, d), dir| {
            match &dir {
                Cardinal::North => (sum, d - 1),
                Cardinal::East => (sum + d, d),
                Cardinal::South => (sum, d + 1),
                Cardinal::West => (sum - d, d),
            }
        })
        .0;
    (area.abs() as usize) - (pipe.len() / 2) + 1
}

#[test]
fn default() {
    let input = get_input(23, 10).unwrap();
    let input = parse_input(&input);
    assert_eq!(7086, part1(&input));
    assert_eq!(317, part2(&input));
}

// Input parsed (194μs)
// 1. 7086 (5μs)
// 2. 317 (60μs)
// Total: 261μs