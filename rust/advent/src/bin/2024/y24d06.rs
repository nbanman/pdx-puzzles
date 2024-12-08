use std::{collections::HashSet, iter::successors};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use utilities::{enums::cardinals::Cardinal, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 6).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: usize,
    dir: Cardinal,
    turned: bool,
}

fn move_dir(pos: usize , dir: Cardinal, width: usize) -> Option<usize> {
    match dir {
        Cardinal::North => pos.checked_sub(width),
        Cardinal::East => Some(pos + 1),
        Cardinal::South => Some(pos + width),
        Cardinal::West => pos.checked_sub(1),
    }
}

fn move_guard(lab: Input, width: usize, state: &State, obstacle: Option<usize>) -> Option<State> {
    let forward = move_dir(state.pos, state.dir, width)?;
    let lab = lab.as_bytes();
    let space = *lab.get(forward)?;
    if Some(forward) != obstacle && space != b'#' && space != b'\n' {
        Some(State { pos: forward, dir: state.dir, turned: false })
    } else {
        let right = state.dir.right();
        let new_move = move_dir(state.pos, right, width);
        if new_move == None || new_move == obstacle || lab[new_move.unwrap()] == b'\n' || lab[new_move.unwrap()] == b'#' {
            let flipped = state.dir.flip();
            let flip_move = move_dir(state.pos, flipped, width).unwrap();
            Some(State { pos: flip_move, dir: flipped, turned: true })
        } else {
            Some(State { pos: new_move.unwrap(), dir: right, turned: true })
        }
    }
}

fn part1(lab: Input) -> Output {
    let width = lab.find('\n').unwrap() + 1;
    let start = lab.find('^').unwrap();
    let start = State { pos: start, dir: Cardinal::North, turned: false };
    golden_path(lab, width, start)
        .unique()
        .count()
}

fn golden_path(lab: &str, width: usize, start: State) -> impl Iterator<Item = usize> + use<'_>{
    successors(Some(start), move |state| {
        move_guard(lab, width, state, None)
    })
        .map(|state| state.pos)
}

fn part2(lab: &str) -> Output {
    let width = lab.find('\n').unwrap() + 1;
    let start = lab.find('^').unwrap();
    let start = State { pos: start, dir: Cardinal::North, turned: false };
    let mut first = true;
    golden_path(lab, width, start)
        .dropping(1)
        .unique()
        //.par_bridge()
        .filter(|&obstacle| {
            let mut visited = HashSet::new();
            let truth = successors(Some(start), move |state| {
                move_guard(lab, width, state, Some(obstacle))
            })
                .find(|state| {
                    if first {
                        print!(
                            "({}, {}), {:?}, {}-",
                            state.pos % width,
                            state.pos / width,
                            state.dir,
                            state.turned,
                        );
                    }
                    if state.turned {
                        if !visited.insert(state.clone()) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }).is_some();
            if first { first = !first; }
            truth
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(24, 6).unwrap();
    assert_eq!(5444, part1(&input));
    assert_eq!(1946, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
", ];
    assert_eq!(41, part1(&inputs[0]));
    assert_eq!(6, part2(&inputs[0]));
}