use std::iter::successors;

use advent::utilities::get_input::get_input;
use utilities::{enums::cardinals::Cardinal, structs::{stopwatch::{ReportDuration, Stopwatch}, strgrid::str_grid::StrGrid}};

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

fn out_of_forest(tree: Int, trees: &Trees) -> bool {
    trees.get(tree).is_none()
}

fn rays<'a>(tree: Int, trees: &'a Trees) -> Vec<impl Iterator<Item = Int> + 'a> {
    Cardinal::entries().into_iter()
        .map(|dir| {
            successors(
                move_one(trees, tree, dir), move |&next| move_one(trees, next, dir)
            )
        })
        .collect()
}

fn move_one(trees: &Trees, tree: Int, dir: Cardinal) -> Option<Int> {
    trees.move_direction(tree, dir)
        .map(|tree| tree.pos)
}

fn terminating(pos: Int, tree: Int, trees: &Trees) -> bool {
    if out_of_forest(pos, trees) {
        true
    } else {
        trees.get(pos).unwrap() >= trees.get(tree).unwrap()
    }
}

fn is_visible(tree: Int, trees: &Trees) -> bool {
    rays(tree, trees).into_iter()
        .any(|ray| {
            let terminal = 
        })
}



fn part1(trees: &Trees) -> Int {
    todo!()
}

fn part2(trees: &Trees) -> Int {
    3
}

#[test]
fn default() {
    let binding = get_input(22, 8).unwrap();
    let input = binding.as_bytes();
    assert_eq!(1845, part1(input));
    // assert_eq!(230112, part2(&input));
}
