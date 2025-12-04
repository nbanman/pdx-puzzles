use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 4).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn solve(input: Input, run_until_stable: bool) -> usize {
    let mut grid = Grid2::new2d_map_str(input, |c| c == '@').unwrap();
    let mut total_removed = 0;
    loop {
        let removable = grid.iter().enumerate()
            .filter_map(|(idx, &b)| {
                if b {
                    let movable = grid.adjacent(idx, true)
                        .unwrap()
                        .filter(|adj| *adj.value)
                        .count() < 4;
                    if movable {
                        Some(idx)
                    } else {
                        None
                    }
                } else {
                    None
                }
                
            })
            .collect_vec();
        for &idx in removable.iter() {
            grid[idx] = false;
        }
        total_removed += removable.len();
        if !run_until_stable || removable.is_empty() {
            break;
        }
    }
    total_removed
}

fn part1(notes: Input) -> Output {
    solve(notes, false)
}

fn part2(notes: Input) -> Output {
    solve(notes, true)
}

#[test]
fn default() {
    let input = get_input(25, 4).unwrap();
    assert_eq!(1604, part1(&input));
    assert_eq!(9397, part2(&input));
}

#[test]
fn test1() {
    let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    assert_eq!(13, part1(&input));
}

#[test]
fn test2() {
    let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    assert_eq!(43, part2(&input));
}

// Input parsed (29μs)
// 1. 1604 (1.380ms)
// 2. 9397 (27.140ms)
// Total: 28.553ms
