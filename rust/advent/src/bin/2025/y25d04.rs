use advent::utilities::get_input::get_input;
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Grid2<bool>;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(25, 4).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Grid2::new2d_map_str(input, |c| c == '@').unwrap()
}

fn part1(grid: &Input) -> Output {
    grid.iter().enumerate()
        .filter(|&(idx, &b)| {
            if b {
                grid.adjacent(idx, true)
                    .unwrap()
                    .filter(|adj| *adj.value)
                    .count() < 4
            } else {
                false
            }
        })
        .count()
}

fn part2(mut grid: Input) -> Output {
    let mut total_removed = 0;
    loop {
        let removed = remove_paper(&mut grid);
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }
    total_removed
}

fn remove_paper(grid: &mut Grid2<bool>) -> usize {
    let total_movable: Vec<_> = grid.iter().copied().enumerate()
        .filter(|&(idx, b)| {
            if b {
                let movable = grid.adjacent(idx, true)
                    .unwrap()
                    .filter(|adj| *adj.value)
                    .count() < 4;
                movable
            } else {
                false
            }
        })
        .collect();
    for (idx, _) in total_movable.iter() {
        grid[*idx] = false;
    }
    total_movable.len()
}

#[test]
fn default() {
    let input = get_input(25, 4).unwrap();
    let input = parse_input(&input);
    assert_eq!(1604, part1(&input));
    assert_eq!(9397, part2(input));
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
    let input = parse_input(input);
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
    let input = parse_input(input);
    assert_eq!(43, part2(input));
}

// Input parsed (81μs)
// 1. 1604 (1.075ms)
// 2. 9397 (22.038ms)
// Total: 23.200ms
