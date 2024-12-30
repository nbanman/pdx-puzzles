use std::collections::HashMap;

use advent::utilities::get_input::get_input;
use rustc_hash::{FxBuildHasher, FxHashMap};
use utilities::{enums::cardinals::Cardinal, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = (Maze<'a>, Cache);
type State = (usize, Cardinal);
type Cache = HashMap<State, Vec<(usize, State)>, FxBuildHasher>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 16).unwrap();
    let (maze, mut cache) = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(maze, &mut cache), stopwatch.lap().report());
    println!("2. {} ({})", part2(maze, &mut cache), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug)]
struct Maze<'a> {
    maze: &'a [u8],
    width: usize,
    start: usize,
    end: usize,
}

fn parse_input<'a>(input: &'a str) -> Input<'a> {
    let maze = input.as_bytes();
    let width = input.find('\n').unwrap() + 1;
    let start = input.find('S').unwrap();
    let end = input.find('E').unwrap();
    let cache: Cache = FxHashMap::default();

    (Maze { maze, width, start, end }, cache)
}

fn get_edges(state: State, maze: Maze) -> Vec<(usize, State)> {
    let (pos, dir) = state;
    [dir, dir.left(), dir.right()].into_iter()
        .filter_map(|new_dir| {
            let new_pos = match new_dir {
                Cardinal::North => pos.checked_sub(maze.width),
                Cardinal::East => Some(pos + 1),
                Cardinal::South => Some(pos + maze.width),
                Cardinal::West => Some(pos - 1),
            }?;
            let block = maze.maze.get(new_pos)?;
            if *block == b'#' {
                None
            } else {
                let weight = if dir == new_dir { 1 } else { 1001 };
                Some((weight, (new_pos, new_dir)))
            }
        })
        .collect()
}

fn part1(maze: Maze, cache: &mut Cache) -> Output {
    let Maze { maze, width, start, end } = maze;

    todo!()
}

fn part2(maze: Maze, cache: &mut Cache) -> Output {
    let Maze { maze, width, start, end } = maze;
    todo!()
}

#[test]
fn default() {
    let input = get_input(24, 16).unwrap();
    let (maze, mut cache) = parse_input(&input);
    assert_eq!(105496, part1(maze, &mut cache));
    // assert_eq!(524, part2(&input));
}

// #[test]
// fn examples() {
//     let inputs = [r"", ];
//     assert_eq!(Y, part1(input[0]));
//     // assert_eq!(Y, part2(input[0]));
// }