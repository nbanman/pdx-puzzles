use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use utilities::structs::{
    coord::Coord2U, grid::{Grid2, GridIterator, GridRotation}, stopwatch::{ReportDuration, Stopwatch}
};

type Input = FxHashMap<Grid, Grid>;
type Output = usize;
type Grid = Grid2<bool>;
type Pos = Coord2U;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(17, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let test: Vec<_> = input
        .replace("/", "")
        .lines()
        .flat_map(|line| {
            let (prev, next) = line.split_once(" => ").unwrap();
            let side = prev.len().isqrt();
            let prev = prev
                .as_bytes()
                .iter()
                .map(|b| *b == b'#')
                .try_collect_grid(side)
                .unwrap();
            let next = next
                .as_bytes()
                .iter()
                .map(|b| *b == b'#')
                .try_collect_grid(side + 1)
                .unwrap();
            [prev.clone(), prev.rotate(GridRotation::FlipX)]
                .into_iter()
                .flat_map(|it| {
                    successors(Some(it.clone()), |acc| {
                        Some(acc.rotate(GridRotation::Right))
                    })
                        .take(4)
                })
                .map(move |it| (it, next.clone()))
        })
        .collect();
    test.into_iter().collect()
}

fn expand_grid(grid: &Grid, rules: &Input) -> Grid {
    let side = if grid.width() & 1 == 0 { 2 } else { 3 };
    let transformed = (0..grid.height()).step_by(side)
        .cartesian_product((0..grid.width()).step_by(side))
        .map(|(y, x)| {
            let start = Pos::new2d(x, y);
            let size = Pos::new2d(side, side);
            let sub_grid = grid
                .sub_grid(
                    start,
                    size,
                )
                .unwrap();
            let transformed = rules.get(&sub_grid).unwrap();
            transformed
        })
        .try_collect_grid(grid.width() / side)
        .unwrap();

    let expanded_size = (side + 1) * (side + 1) * transformed.len();
    let expanded_side = expanded_size.isqrt();
    let mut expanded_grid = Grid::new2d(vec![false; expanded_size], expanded_side).unwrap();

    for (sub_pos, &sub_grid) in transformed.iter_with_coords() {
        let offset = Pos::new2d(sub_pos.x() * (side + 1), sub_pos.y() * (side + 1));
        for (pos, &value) in sub_grid.iter_with_coords() {
            expanded_grid[pos + offset] = value;
        }
    }
    expanded_grid
}

fn solve(rules: &Input, iterations: usize) -> usize {
    let initial = vec![false, true, false, false, false, true, true, true, true];
    let initial = Grid::new2d(initial, 3).unwrap();
    successors(Some(initial), |grid| Some(expand_grid(grid, rules)))
        .take(iterations + 1)
        .last()
        .unwrap()
        .iter()
        .filter(|&&it| it)
        .count()
}

fn part1(rules: &Input) -> Output {
    solve(rules, 5)
}

fn part2(rules: &Input) -> Output {
    solve(rules, 18)
}

#[test]
fn default() {
    let input = get_input(17, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(150, part1(&input));
    assert_eq!(2606275, part2(&input));
}

// Input parsed (174μs)
// 1. 150 (136μs)
// 2. 2606275 (1.469s)
// Total: 1.469s