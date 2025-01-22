use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use utilities::structs::{stopwatch::{ReportDuration, Stopwatch}, strgrid::str_grid::StrGrid};

type Input<'a> = (StrGrid<'a>, usize);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(22, 12).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let map = StrGrid::new(input).unwrap();
    let end = map.s.iter().position(|&x| x == b'E').unwrap();
    (map, end)
}

fn height(pos: u8) -> u8 {
    match pos {
        b'S' => b'a',
        b'E' => b'z',
        pos => pos
    }
}

fn solve(map: &StrGrid, end: usize, targets: &[u8]) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((end, 0));
    let mut visited: Vec<bool> = vec![false; map.s.len()];
    while let Some((pos, steps)) = q.pop_front() {
        let pos_height = height(map.s[pos]) - 1;
        for neighbor in map.adjacent(pos) {
            if visited[neighbor.pos] || height(neighbor.b) < pos_height {
                continue;
            } else if targets.contains(&neighbor.b) {
                return steps + 1;
            } else {
                visited[neighbor.pos] = true;
                q.push_back((neighbor.pos, steps + 1));
            }
        }
    }
    panic!("Path not found!")
}

fn part1((map, end): &Input) -> Output {
    solve(map, *end, &[b'S'])
}

fn part2((map, end): &Input) -> Output {
    solve(map, *end, &[b'S', b'a'])
}

#[test]
fn default() {
    let input = get_input(22, 12).unwrap();
    let input = parse_input(&input);
    assert_eq!(361, part1(&input));
    assert_eq!(354, part2(&input));
}

// Input parsed (18μs)
// 1. 361 (58μs)
// 2. 354 (50μs)
// Total: 129μs