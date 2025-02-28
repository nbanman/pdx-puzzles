use std::collections::VecDeque;

use advent::utilities::get_input::get_input;
use utilities::structs::stopwatch::{ReportDuration, Stopwatch};

type Input = (usize, Vec<u8>);
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(23, 21).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(garden: &str) -> Input {
    let garden_slice = garden.as_bytes();
    let width = garden.find('\n').unwrap();
    let start = garden.find('S').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start, 0u8));
    let mut visited = vec![false; garden.len()];
    visited[start] = true;
    let mut step_count = Vec::new();

    // BFS finds neighbors and runs until the queue is empty, meaning that no more neighbors are found
    // due to everything already being visited.
    while let Some((pos, steps)) = q.pop_front() {
        step_count.push(steps);
        let pos = pos as isize;
        let width = width as isize;
        let neighbors: Vec<_> = vec![pos - (width + 1), pos + 1, pos - 1, pos + (width + 1)]
            .into_iter()
            .filter(|neighbor| {
                if let Ok(neighbor) = usize::try_from(*neighbor) {
                    if let Some(c) = garden_slice.get(neighbor) {
                        "#\n".find(*c as char) == None && !visited[neighbor]
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .collect();
        neighbors.into_iter().for_each(|neighbor| {
            visited[neighbor as usize] = true;
            q.push_back((neighbor as usize, steps + 1));
        });
    }
    (width, step_count)
}

fn part1(input: &Input) -> Output {
    let (_, garden_path) = input;
    garden_path
        .iter()
        .filter(|steps| steps <= &&64u8 && *steps & 1 == 0)
        .count()
}

fn part2(input: &Input) -> Output {
    let (width, garden_path) = input;
    let (even_path, odd_path): (Vec<u8>, Vec<u8>) =
        garden_path.iter().partition(|&it| *it & 1 == 0);
    let even_corners = even_path.iter().filter(|it| it > &&65).count();
    let odd_corners = odd_path.iter().filter(|it| it > &&65).count();
    let n = (26501365 - width / 2) / width;
    (n + 1) * (n + 1) * odd_path.len() + n * n * even_path.len() - (n + 1) * odd_corners
        + n * even_corners
}

#[test]
fn default() {
    let input = get_input(23, 21).unwrap();
    let input = parse_input(&input);
    assert_eq!(3782, part1(&input));
    assert_eq!(630661863455116, part2(&input));
}

// Input parsed (506μs)
// 1. 3782 (13μs)
// 2. 630661863455116 (30μs)
// Total: 551μs
