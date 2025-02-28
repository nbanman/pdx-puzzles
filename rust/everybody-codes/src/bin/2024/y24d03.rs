use everybody_codes::utilities::inputs::get_inputs;
use std::{collections::HashSet, iter::successors};
use utilities::structs::coord::Coord2;

fn main() {
    let (input1, input2, input3) = get_inputs(24, 3);
    println!("1. {}", solve(&input1, false));
    println!("2. {}", solve(&input2, false));
    println!("3. {}", solve(&input3, true));
}

fn solve(input: &str, diagonals: bool) -> usize {
    let width = input.find('\n').unwrap() + 1;
    let blocks: HashSet<Coord2> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'#')
        .map(|(idx, _)| {
            let x = (idx % width) as i64;
            let y = (idx / width) as i64;
            Coord2::new2d(x, y)
        })
        .collect();
    let dig = |blocks: &HashSet<Coord2>| -> Option<HashSet<Coord2>> {
        let next: HashSet<Coord2> = blocks
            .iter()
            .filter(|block| {
                block
                    .adjacent(diagonals)
                    .iter()
                    .all(|pos| blocks.contains(pos))
            })
            .copied()
            .collect();
        if next.is_empty() { None } else { Some(next) }
    };
    successors(Some(blocks), dig).fold(0, |count, stage| count + stage.len())
}

#[test]
fn default() {
    let (input1, input2, input3) = get_inputs(24, 3);
    assert_eq!(134, solve(&input1, false));
    assert_eq!(2810, solve(&input2, false));
    assert_eq!(10443, solve(&input3, true));
}
