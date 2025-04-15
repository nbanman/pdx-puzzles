use advent::utilities::get_input::get_input;
use std::{collections::VecDeque, ops::Mul};
use itertools::Itertools;
use utilities::structs::{grid::{Grid, GridIterator}, stopwatch::{ReportDuration, Stopwatch}};

type Int = usize;
type Input = (Grid<Int, 2>, Vec<Int>);
type Output = Int;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 9).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let height_map = Grid::try_from(input).unwrap();
    let width = height_map.width();
    let height_map = height_map
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as Int)
        .try_collect_grid(width)
        .unwrap();
    let low_indices = height_map
        .iter()
        .enumerate()
        .filter(|(index, height)| {
            height_map
                .adjacent(*index, false)
                .unwrap()
                .all(|ad| ad.value > *height)
        })
        .map(|(i, _)| i)
        .collect();
    (height_map, low_indices)
}

fn part1((height_map, low_indices): &Input) -> Output {
    low_indices.iter().map(|&it| height_map[it] + 1).sum()
}

fn part2((height_map, low_indices): &Input) -> Output {
    low_indices.iter()
        .map(|id| bfs(id, height_map))
        .sorted()
        .rev()
        .take(3)
        .reduce(Int::mul)
        .unwrap()
}

fn bfs(id: &usize, height_map: &Grid<usize, 2>) -> Int {
    let mut q = VecDeque::new();
    q.push_back((*id, 0));
    let mut visited = vec![false; height_map.len()];
    let mut pos_count = 0;
    while let Some((id, steps)) = q.pop_front() {
        if visited[id] { continue; } else { visited[id] = true; }
        pos_count += 1;
        let height = height_map[id];
        height_map
            .adjacent(id, false)
            .unwrap()
            .filter(|adj| {
                let n = adj.index;
                let nh = height_map[n];
                !visited[n] && nh != 9 && nh > height 
            })
            .for_each(|adj| q.push_back((adj.index, steps + 1)));
    }
    pos_count
}

#[test]
fn default() {
    let input = get_input(21, 9).unwrap();
    let input = parse_input(&input);
    assert_eq!(448, part1(&input));
    assert_eq!(1417248, part2(&input));
}

// Input parsed (905μs)
// 1. 448 (7μs)
// 2. 1417248 (845μs)
// Total: 1ms