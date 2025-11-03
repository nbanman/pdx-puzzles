use std::collections::BinaryHeap;

use advent::utilities::get_input::get_input;
use utilities::structs::{coord::Coord2U, grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = Cavern;
type Output = usize;
type Pos = Coord2U;
type Cavern = Grid2<u8>;


#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: usize,
    weight: usize,
    f: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f).then(other.weight.cmp(&self.weight))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.f.cmp(&self.f).then(other.weight.cmp(&self.weight)))
    }
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(21, 15).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    Cavern::new2d_map_str(input, |c| c as u8 - 48).unwrap()
}

fn shortest_path(cavern: &Cavern) -> usize {
    let width = cavern.width();
    let target_index = cavern.data.len() - 1;
    let target = Pos::new2d(target_index % width, target_index / width);
    let heuristic = |i: usize| {
        let pos = Pos::new2d(i % width, i / width);
        pos.manhattan_distance(target)
    };

    let mut open = BinaryHeap::new();
    let start = State {
        pos: 0,
        weight: 0,
        f: heuristic(0),
    };
    open.push(start);
    let mut closed = vec![false; cavern.len()];

    while let Some(State { pos, weight, f: _ }) = open.pop() {
        if closed[pos] {
            continue;
        } 

        closed[pos] = true;
        if pos == target_index {
            return weight;
        }

        for adj in cavern.adjacent(pos, false).unwrap() {
            let adj_pos = adj.index;
            if closed[adj_pos] { continue; }
            let adj_weight = weight + *adj.value as usize;
            let adj_state = State {
                pos: adj_pos,
                weight: adj_weight,
                f: adj_weight + heuristic(adj_pos),
            };
            open.push(adj_state);
        }
    }
    unreachable!()
}

fn part1(cavern: &Input) -> Output {
    shortest_path(cavern)
}

fn part2(initial_cavern: &Input) -> Output {
    let width = initial_cavern.width();
    let height = initial_cavern.height();
    let expanded_width = width * 5;
    let expanded_height = height * 5;
    let expanded_cavern = Cavern::new_with_fn(
        [expanded_width, expanded_height],
        |i| {
            let x = i % expanded_width;
            let y = i / expanded_width;
            let x_base = x % width;
            let y_base = y % height;
            let risk = initial_cavern[y_base * width + x_base];
            let added_risk = x / width + y / height;
            ((risk as usize + added_risk - 1) % 9 + 1) as u8
        }
    );
    shortest_path(&expanded_cavern)
}

#[test]
fn default() {
    let input = get_input(21, 15).unwrap();
    let input = parse_input(&input);
    assert_eq!(602, part1(&input));
    assert_eq!(2935, part2(&input));
}

// Input parsed (58μs)
// 1. 602 (1ms)
// 2. 2935 (47ms)
// Total: 49ms