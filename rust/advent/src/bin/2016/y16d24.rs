use std::{cmp::Reverse, collections::BinaryHeap};

use advent::utilities::get_input::get_input;
use rustc_hash::{FxHashMap, FxHashSet};
use utilities::structs::{grid::Grid2, stopwatch::{ReportDuration, Stopwatch}};

type Input = FxHashMap<char, Vec<(usize, char)>>;
type Output = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    c: char,
    visited: u8,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(16, 24).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let layout: Grid2<char> = input.try_into().unwrap();

    let edge_map: Input = layout.iter().enumerate()
        .filter(|(_, c)| c.is_numeric())
        .map(|(pos, &c)| {
            let mut visited = vec![false; layout.len()];
            visited[pos] = true;
            let mut todo = vec![(pos, c)];
            let mut next = Vec::new();
            let mut weight = 0;
            let mut numbers = Vec::new();
            while !todo.is_empty() {
                for (cur_pos, cur_c) in todo.drain( .. ) {
                    if cur_c.is_numeric() && cur_c != c {
                        numbers.push((weight, cur_c));
                    }
                    for neighbor in layout.adjacent(cur_pos, false).unwrap() {
                        if neighbor.value != &'#' && !visited[neighbor.index] {
                            visited[neighbor.index] = true;
                            next.push((neighbor.index, *neighbor.value));
                        }
                    }
                }
                weight += 1;
                std::mem::swap(&mut todo, &mut next);
            }
            (c, numbers)
        })
        .collect();
  
    edge_map
}

fn solve<F>(edges: &Input, end_condition: F) -> Output
where F: Fn(char) -> bool,
{
    let start = State { c: '0', visited: 1 };
    let mut weights = FxHashMap::default();
    weights.insert(start, 0);
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, start)));
    let mut visited = FxHashSet::default();
    while let Some(Reverse((cur_weight, cur))) = q.pop() {
        if !visited.insert(cur) { continue; }
        if cur.visited == 255 && end_condition(cur.c) {
            return cur_weight;
        }
        for &(edge_weight, edge_c) in edges.get(&cur.c).unwrap() {
            let edge_visited = cur.visited | (1 << (edge_c as u8 - 48));
            let edge_state = State { c: edge_c, visited: edge_visited };
            let alternate_weight = cur_weight + edge_weight;
            let known_weight = *weights.get(&edge_state).unwrap_or(&usize::MAX);
            if alternate_weight < known_weight {
                weights.insert(edge_state, alternate_weight);
                q.push(Reverse((alternate_weight, State { c: edge_c, visited: edge_visited })));
            }
        }
    }
    unreachable!()
}

fn part1(edges: &Input) -> Output {
    solve(edges, |_| true)
}

fn part2(edges: &Input) -> Output {
    solve(edges, |c| c == '0')
}

#[test]
fn default() {
    let input = get_input(16, 24).unwrap();
    let input = parse_input(&input);
    assert_eq!(470, part1(&input));
    assert_eq!(720, part2(&input));
}

// Input parsed (2ms)
// 1. 470 (56μs)
// 2. 720 (69μs)
// Total: 2ms