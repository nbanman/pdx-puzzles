use std::{cmp::{max, min}, collections::VecDeque};

use advent::utilities::get_input::get_input;
use itertools::Itertools;
use utilities::structs::{coord::Coord2, stopwatch::{ReportDuration, Stopwatch}, str_grid::{AdjacentMetadata, StrGrid}};

type Output = usize;
type Pos = Coord2;

#[derive(Debug)]
struct Input<'a> {
    racetrack: StrGrid<'a>,
    end: usize,
    from_start: Vec<usize>,
    from_end: Vec<usize>,
    threshold: usize,
}

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 20).unwrap();
    let input = parse_input(&input);
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

fn parse_input(input: &str) -> Input {
    let racetrack = StrGrid::new(input).unwrap();
    let start = input.find('S').unwrap();
    let end  = input.find('E').unwrap();
    let mut visited_from_start: Vec<usize> = vec![1_000_000; input.len()];
    visited_from_start[start] = 0;
    let mut visited_from_end = vec![1_000_000; input.len()];
    visited_from_end[end] = 0;
    let from_start = bfs(&racetrack, start, end, visited_from_start);
    let from_end = bfs(&racetrack, end, start, visited_from_end);
    let threshold = from_start[end] - 100;
    Input {
        racetrack,
        end,
        from_start,
        from_end,
        threshold,
    }
}

fn bfs(
    racetrack: &StrGrid,
    start: usize, 
    end: usize, 
    mut visited: Vec<usize>, 
)-> Vec<usize> {
    let mut q = VecDeque::new();
    q.push_back((0, start));
    while let Some((steps, current)) = q.pop_front() {
        if current == end { 
            break; 
        }
        let neighbors = racetrack.adjacent(current)
            .filter_map(|AdjacentMetadata { pos, dir: _, b }| {
                if visited[pos] != 1_000_000 { return None; }
                if b == b'#' {
                    visited[pos] = steps + 1;
                    None
                } else {
                    let neighbor_steps = steps + 1;
                    visited[pos] = neighbor_steps;
                    Some((neighbor_steps, pos))
                }
            });
        for neighbor in neighbors {
            q.push_back(neighbor);
        }
    }
    visited
}

fn count_valid(input: &Input, pos: usize, end_coord: Pos, steps: usize) -> usize {
    if input.racetrack.get(pos) == Some(b'#') { return 0; }

    let pos_coord = Pos::from_index(pos, input.racetrack.width).unwrap();
    let difference = end_coord - pos_coord;
    let min_steps = steps + difference.manhattan_distance(&Pos::origin());
    
    if min_steps > input.threshold { return 0; }
    
    let mut count = 0;
    let allowance = ((input.threshold - min_steps) / 2) as i64;
    let (north, south) = [pos_coord.y(), end_coord.y()].into_iter()
        .minmax()
        .into_option()
        .unwrap();
    let y_min = max(1, min(north - allowance, pos_coord.y() - 20));
    let y_max = min(
        (input.racetrack.height - 2) as i64, 
        min(south + allowance, pos_coord.y() + 20)
    );
    for y in y_min..=y_max {
        let (west, east) = [pos_coord.x(), end_coord.x()].into_iter()
            .minmax()
            .into_option()
            .unwrap();
        
        let x_allowance = if y < north {
            allowance - (north - y)
        } else if y > south {
            allowance - (y - south)
        } else {
            allowance
        };
        
        let total_allowed = 20 - (pos_coord.y() - y).abs();
        let x_min = max(
            1,
            max(pos_coord.x() - total_allowed, west - x_allowance)
        );
        let x_max = min(
            (input.racetrack.width - 3) as i64,
            min(pos_coord.x() + total_allowed, east + x_allowance)
        );
        for x in x_min..=x_max {
            let re_pos = (y * input.racetrack.width as i64 + x) as usize;
            if input.racetrack.get(re_pos) == Some(b'#') { continue; }
            let cheat_steps = steps 
                + pos_coord.manhattan_distance(&Pos::new2d(x, y))
                + input.from_end[re_pos];
            if cheat_steps <= input.threshold {
                count += 1;
            }
        }
    }
    count
}


fn part1(input: &Input) -> Output {
    input.from_start.iter().enumerate()
        .filter(|&(pos, &steps)| {
            input.racetrack.get(pos) == Some(b'#') 
                && steps + input.from_end[pos] <= input.threshold
        })
        .count()
}

fn part2(input: &Input) -> Output {
    input.from_start.iter().enumerate()
        .map(|(pos, &steps)| {
            count_valid(
                input,
                pos, 
                Pos::from_index(input.end, input.racetrack.width).unwrap(), 
                steps
            )
        })
        .sum()
}

#[test]
fn default() {
    let input = get_input(24, 20).unwrap();
    let input = parse_input(&input);
    assert_eq!(1406, part1(&input));
    assert_eq!(1006101, part2(&input));
}

// Input parsed (397μs)
// 1. 1406 (35μs)
// 2. 1006101 (16ms)
// Total: 16ms