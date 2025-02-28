use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use utilities::{
    enums::cardinals::Cardinal,
    structs::stopwatch::{ReportDuration, Stopwatch},
};

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

fn move_dir(pos: usize, dir: Cardinal, width: usize) -> Option<usize> {
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
    if space == b'\n' {
        return None;
    }
    if Some(forward) != obstacle && space != b'#' {
        Some(State {
            pos: forward,
            dir: state.dir,
            turned: false,
        })
    } else {
        let right = state.dir.right();
        Some(State {
            pos: state.pos,
            dir: right,
            turned: true,
        })
    }
}

fn golden_path(lab: &str, width: usize, start: State) -> impl Iterator<Item = State> + use<'_> {
    successors(Some(start), move |state| {
        move_guard(lab, width, state, None)
    })
}

fn part1(lab: Input) -> Output {
    let width = lab.find('\n').unwrap() + 1;
    let start = lab.find('^').unwrap();
    let start = State {
        pos: start,
        dir: Cardinal::North,
        turned: false,
    };
    golden_path(lab, width, start)
        .map(|state| state.pos)
        .unique()
        .count()
}

fn part2(lab: &str) -> Output {
    let width = lab.find('\n').unwrap() + 1;
    let start = lab.find('^').unwrap();
    let start = State {
        pos: start,
        dir: Cardinal::North,
        turned: false,
    };
    let mut obstacles = vec![false; lab.len()];
    golden_path(lab, width, start)
        .tuple_windows::<(_, _)>()
        .filter(|(_, next)| {
            let obstacle = next.pos;
            if obstacles[obstacle] {
                false
            } else {
                obstacles[obstacle] = true;
                true
            }
        })
        .par_bridge()
        .filter(|(current, next)| {
            let obstacle = next.pos;
            let mut visited = vec![false; lab.len()];

            successors(Some(*current), move |state| {
                move_guard(lab, width, state, Some(obstacle))
            })
            .any(|state| {
                if state.turned && state.dir == Cardinal::North {
                    if visited[state.pos] {
                        true
                    } else {
                        visited[state.pos] = true;
                        false
                    }
                } else {
                    false
                }
            })
        })
        .count()
}

#[test]
fn default() {
    let input = get_input(24, 6).unwrap();
    assert_eq!(5444, part1(&input));
    assert_eq!(1946, part2(&input));
}

// Input parsed (26μs)
// 1. 5444 (355μs)
// 2. 1946 (4ms)
// Total: 5ms

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
"];
    assert_eq!(41, part1(inputs[0]));
    assert_eq!(6, part2(inputs[0]));
}
