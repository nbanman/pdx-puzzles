use std::collections::{HashSet, VecDeque};

use advent::utilities::get_input::get_input;
use utilities::{parsing::try_get::TryGet, structs::stopwatch::{ReportDuration, Stopwatch}};

type Input<'a> = &'a str;
type Output = usize;

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let input = get_input(24, 10).unwrap();
    println!("Input parsed ({})", stopwatch.lap().report());
    println!("1. {} ({})", part1(&input), stopwatch.lap().report());
    println!("2. {} ({})", part2(&input), stopwatch.lap().report());
    println!("Total: {}", stopwatch.stop().report());
}

// returns (topo_map, width, trailheads)
fn parse_input(input: Input) -> (Vec<u8>, isize, Vec<usize>) {
    let topo_map: Vec<u8> = input.as_bytes().iter()
        .filter(|b| b.is_ascii_digit())
        .map(|b| *b - 48)
        .collect();
    let width = (input.find('\n').unwrap()) as isize;
    let trailheads = topo_map.iter().enumerate()
        .filter(|&(_, height)| *height == 0)
        .map(|(pos, _)| pos)
        .collect();
    (topo_map, width, trailheads)
}

fn hike(
    (pos, height): (usize, u8), 
    topo_map: &[u8], 
    width: isize
) -> impl Iterator<Item = (usize, u8)> + use<'_> {
    [-width, 1, width, -1].into_iter()
        .filter_map(move |offset| {
            let neighbor_pos = pos as isize + offset;
            // common footgun for me when I venture away from strings!
            if neighbor_pos as usize == pos + 1 && neighbor_pos % width == 0 { return None; }
            if neighbor_pos + 1 == pos as isize && pos as isize % width == 0 { return None; }  
            let neighbor_height = topo_map.try_get(neighbor_pos)?;
            if *neighbor_height == height + 1 {
                Some((neighbor_pos as usize, *neighbor_height))
            } else {
                None   
            }
        })
}

fn paths(trailhead: usize, topo_map: &[u8], width: isize) -> usize {
    let mut q = VecDeque::new();
    q.push_back((trailhead, 0u8));
    let mut paths = 0;
    let mut visited = HashSet::new();
    while let Some(state) = q.pop_front() {
        if state.1 == 9 { paths += 1; }
        for neighbor in hike(state, topo_map, width)
            .filter(|neighbor| visited.insert(*neighbor))
        {
            q.push_back(neighbor);
        }
    }
    paths
}

fn distinct_paths(trailhead: usize, topo_map: &[u8], width: isize) -> usize {
    let mut q = VecDeque::new();
    q.push_back((trailhead, 0u8));
    let mut paths = 0;
    while let Some(state) = q.pop_front() {
        if state.1 == 9 { paths += 1; }
        for neighbor in hike(state, topo_map, width) {
            q.push_back(neighbor);
        }
    }
    paths
}


fn part1(input: Input) -> Output {
    let (topo_map, width, trailheads) = parse_input(input);
    trailheads.iter()
        .map(|&trailhead| paths(trailhead, &topo_map, width))
        .sum()
}

fn part2(input: Input) -> Output {
    let (topo_map, width, trailheads) = parse_input(input);
    trailheads.iter()
        .map(|&trailhead| distinct_paths(trailhead, &topo_map, width))
        .sum()
}

#[test]
fn default() {
    let input = get_input(24, 10).unwrap();
    assert_eq!(461, part1(&input));
    assert_eq!(875, part2(&input));
}

#[test]
fn examples() {
    let inputs = [r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732", ];
    assert_eq!(36, part1(inputs[0]));
    // assert_eq!(Y, part2(&input));
}

// Input parsed (24μs)
// 1. 461 (460μs)
// 2. 875 (203μs)
// Total: 692μs
