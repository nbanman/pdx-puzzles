use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::{enums::cardinals::Cardinal, structs::{stopwatch::{ReportDuration, Stopwatch}, str_grid::StrGrid}};

type State = (usize, Cardinal);

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 19).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(input.clone()), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> impl Iterator<Item = char> + '_ + Clone {
    let maze = StrGrid::new(input).unwrap();
    let start = (maze.s.iter().position(|&b| b != b' ').unwrap(), Cardinal::South);
    successors(Some(start),  move |&state| move_mouse(maze, state))
        .map(|(pos, _)| maze.s[pos] as char)
}

fn move_mouse(maze: StrGrid, (pos, dir): State) -> Option<State> {
    let ground = maze.s[pos];
    if ground == b'+' {
        maze.move_direction(pos, dir.left())
            .map(|left| {
                if left.b != b' ' {
                    Some((left.pos, left.dir))
                } else {
                    None
                }
            })
            .flatten()
            .or_else(|| {
                let right = maze.move_direction(pos, dir.right()).unwrap();
                Some((right.pos, right.dir))
            })
    } else {
        maze.move_direction(pos, dir)
            .map(|straight| {
                if straight.b != b' ' {
                    Some((straight.pos, dir))
                } else {
                    None
                }
            })
            .flatten()
    }
}

fn part1(run_maze: impl Iterator<Item = char>) -> String {
    run_maze.filter(|c| c.is_ascii_alphabetic()).collect()
}

fn part2(run_maze: impl Iterator<Item = char>) -> usize {
    run_maze.count()
}

#[test]
fn default() {
    let input = get_input(17, 19).unwrap();
    let input = parse_input(&input);
    assert_eq!("EOCZQMURF".to_string(), part1(input.clone()));
    assert_eq!(16312, part2(input));
}

// Input parsed (60μs)
// 1. EOCZQMURF (66μs)
// 2. 16312 (60μs)
// Total: 189μs