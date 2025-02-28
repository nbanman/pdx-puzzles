use std::iter::successors;

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::{
    enums::cardinals::Cardinal,
    structs::{
        stopwatch::{ReportDuration, Stopwatch},
        str_grid::{AdjacentMetadata, StrGrid},
    },
};

type Trees<'a> = StrGrid<'a>;
type Int = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 8).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Trees {
    StrGrid::new(input).unwrap()
}

fn rays<'a>(tree: Int, trees: &'a Trees) -> Vec<impl Iterator<Item = AdjacentMetadata<Int>> + 'a> {
    Cardinal::entries()
        .into_iter()
        .map(|dir| {
            successors(trees.move_direction(tree, dir), move |next| {
                trees.move_direction(next.pos, dir)
            })
        })
        .collect()
}

fn is_visible(tree: Int, height: u8, trees: &Trees) -> bool {
    rays(tree, trees)
        .into_iter()
        .any(|mut ray| ray.all(|pos| height > pos.b))
}

fn scenic_score(tree: Int, height: u8, trees: &Trees) -> Int {
    rays(tree, trees)
        .into_iter()
        .map(|ray| {
            ray.enumerate()
                .find_or_last(|(_, it)| trees.get(it.pos).unwrap() >= height)
                .map(|(index, _)| index + 1)
                .unwrap_or_default()
        })
        .reduce(std::ops::Mul::mul)
        .unwrap()
}

fn part1(trees: &Trees) -> Int {
    trees
        .s
        .iter()
        .enumerate()
        .filter(|&(tree, &height)| height != b'\n' && is_visible(tree, height, trees))
        .count()
}

fn part2(trees: &Trees) -> Int {
    trees
        .s
        .iter()
        .enumerate()
        .filter(|&(_, &height)| height != b'\n')
        .map(|(tree, &height)| scenic_score(tree, height, trees))
        .max()
        .unwrap()
}

#[test]
fn default() {
    let input = get_input(22, 8).unwrap();
    let input = parse_input(&input);
    assert_eq!(1708, part1(&input));
    assert_eq!(504000, part2(&input));
}

#[test]
fn example() {
    let input = parse_input(
        "30373
25512
65332
33549
35390",
    );
    assert_eq!(part1(&input), 21);
    assert_eq!(part2(&input), 8);
}

// Input parsed (22μs)
// 1. 1708 (485μs)
// 2. 504000 (882μs)
// Total: 1ms
